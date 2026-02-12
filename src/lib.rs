//! A library for generating GitHub-style contribution graphs.
//!
//! This crate provides a builder interface to create contribution heatmaps, similar to those found on GitHub user profiles.
//! It supports custom date ranges, colors, and dimensions, outputting the result as an image.
//!
//! # Examples
//!
//! ```rust
//! use contribution_grid::{ContributionGraph, Theme, LinearStrategy};
//! use chrono::NaiveDate;
//! use std::collections::HashMap;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut data = HashMap::new();
//! data.insert(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 5);
//! data.insert(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(), 12);
//!
//! // Use a built-in theme with a linear mapping strategy
//! let img = ContributionGraph::new()
//!     .with_data(data)
//!     .theme(Theme::blue(LinearStrategy))
//!     .generate();
//!
//! // img is an ImageBuffer from the `image` crate
//! // img.save("graph.png")?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;

use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use image::ImageBuffer;
use image::Rgba;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

/// Defines how to map counts to color indices.
pub trait MappingStrategy {
    /// Maps a count to a color index `[0, num_colors)`.
    fn map(&self, count: u32, max_count: u32, num_colors: usize) -> usize;
}

/// Linear percentile-based mapping.
pub struct LinearStrategy;
impl MappingStrategy for LinearStrategy {
    fn map(&self, count: u32, max_count: u32, num_colors: usize) -> usize {
        if count == 0 || max_count == 0 {
            return 0;
        }
        let ratio = count as f32 / max_count as f32;
        ((ratio * (num_colors - 1) as f32).round() as usize).min(num_colors - 1)
    }
}

/// Logarithmic mapping (emphasizes differences at lower values).
pub struct LogarithmicStrategy;
impl MappingStrategy for LogarithmicStrategy {
    fn map(&self, count: u32, max_count: u32, num_colors: usize) -> usize {
        if count == 0 {
            return 0;
        }
        let log_count = (count as f32 + 1.0).ln();
        let log_max = (max_count as f32 + 1.0).ln();
        let ratio = log_count / log_max;
        ((ratio * (num_colors - 1) as f32).round() as usize).min(num_colors - 1)
    }
}

/// Fixed threshold mapping (ignores `max_count`).
pub struct ThresholdStrategy {
    thresholds: Vec<u32>,
}
impl ThresholdStrategy {
    /// Creates a new [`ThresholdStrategy`] with the given thresholds.
    pub fn new(thresholds: Vec<u32>) -> Self {
        Self { thresholds }
    }
}
impl MappingStrategy for ThresholdStrategy {
    fn map(&self, count: u32, _max_count: u32, _num_colors: usize) -> usize {
        self.thresholds
            .iter()
            .position(|&t| count < t)
            .unwrap_or(self.thresholds.len())
    }
}

/// Defines the color palette and the strategy used to map counts to colors.
pub struct Palette {
    colors: Vec<Rgba<u8>>,
    strategy: Box<dyn MappingStrategy>,
}

impl Palette {
    /// Creates a new [`Palette`] with a list of colors and a mapping strategy.
    pub fn new(colors: Vec<Rgba<u8>>, strategy: impl MappingStrategy + 'static) -> Self {
        Self {
            colors,
            strategy: Box::new(strategy),
        }
    }

    /// Returns the color for a given contribution count based on the global maximum count.
    pub fn get_color(&self, count: u32, max_count: u32) -> Rgba<u8> {
        let index = self.strategy.map(count, max_count, self.colors.len());
        self.colors[index]
    }
}

/// Helper for built-in color themes.
pub struct Theme;

impl Theme {
    /// Modern GitHub green style (5 colors).
    pub fn github(strategy: impl MappingStrategy + 'static) -> Palette {
        Palette::new(
            vec![
                Rgba([235, 237, 240, 255]),
                Rgba([155, 233, 168, 255]),
                Rgba([64, 196, 99, 255]),
                Rgba([48, 161, 78, 255]),
                Rgba([33, 110, 57, 255]),
            ],
            strategy,
        )
    }

    /// Classic GitHub style (5 colors).
    pub fn github_old(strategy: impl MappingStrategy + 'static) -> Palette {
        Palette::new(
            vec![
                Rgba([238, 238, 238, 255]),
                Rgba([198, 228, 139, 255]),
                Rgba([123, 201, 111, 255]),
                Rgba([35, 154, 59, 255]),
                Rgba([25, 97, 39, 255]),
            ],
            strategy,
        )
    }

    /// A blue-shaded theme (7 colors).
    pub fn blue(strategy: impl MappingStrategy + 'static) -> Palette {
        Palette::new(
            vec![
                Rgba([235, 237, 240, 255]),
                Rgba([201, 231, 245, 255]),
                Rgba([168, 196, 238, 255]),
                Rgba([135, 146, 232, 255]),
                Rgba([96, 101, 181, 255]),
                Rgba([61, 62, 125, 255]),
                Rgba([31, 29, 66, 255]),
            ],
            strategy,
        )
    }

    /// A red-shaded theme (5 colors).
    pub fn red(strategy: impl MappingStrategy + 'static) -> Palette {
        Palette::new(
            vec![
                Rgba([235, 237, 240, 255]),
                Rgba([255, 208, 198, 255]),
                Rgba([255, 148, 133, 255]),
                Rgba([234, 86, 70, 255]),
                Rgba([181, 28, 28, 255]),
            ],
            strategy,
        )
    }
}

/// A builder struct for generating GitHub-style contribution graphs.
pub struct ContributionGraph {
    data: HashMap<NaiveDate, u32>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    box_size: u32,
    gap: u32,
    margin: u32,
    palette: Palette,
    background_color: Rgba<u8>,
    round_corners: bool,
}

impl Default for ContributionGraph {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            start_date: None,
            end_date: None,
            box_size: 11,
            gap: 3,
            margin: 20,
            palette: Theme::github(LinearStrategy),
            background_color: Rgba([0, 0, 0, 0]),
            round_corners: true,
        }
    }
}

impl ContributionGraph {
    /// Creates a new [`ContributionGraph`] builder with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the contribution data for the graph.
    ///
    /// The data is a mapping from [`NaiveDate`] to a contribution count (`u32`).
    pub fn with_data(mut self, data: HashMap<NaiveDate, u32>) -> Self {
        self.data = data;
        self
    }

    /// Sets the start date for the graph.
    ///
    /// If not provided, it defaults to the earliest date in the data or January 1st of the current year.
    pub fn start_date(mut self, date: NaiveDate) -> Self {
        self.start_date = Some(date);
        self
    }

    /// Sets the end date for the graph.
    ///
    /// If not provided, it defaults to the latest date in the data or December 31st of the current year.
    pub fn end_date(mut self, date: NaiveDate) -> Self {
        self.end_date = Some(date);
        self
    }

    /// Sets the size of each contribution box in pixels.
    pub fn box_size(mut self, size: u32) -> Self {
        self.box_size = size;
        self
    }

    /// Sets the gap between contribution boxes in pixels.
    pub fn gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }

    /// Sets the margin around the entire grid in pixels.
    pub fn margin(mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }

    /// Sets a predefined or custom [`Palette`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use contribution_grid::{ContributionGraph, Theme, LinearStrategy, Palette, ThresholdStrategy};
    /// use image::Rgba;
    ///
    /// // Using a built-in theme
    /// let graph = ContributionGraph::new()
    ///     .theme(Theme::github(LinearStrategy));
    ///
    /// // Using a custom palette with thresholds
    /// let custom_palette = Palette::new(
    ///     vec![
    ///         Rgba([20, 20, 20, 255]),
    ///         Rgba([0, 255, 0, 255]),
    ///         Rgba([0, 0, 255, 255]),
    ///     ],
    ///     ThresholdStrategy::new(vec![1, 10])
    /// );
    ///
    /// let graph = ContributionGraph::new()
    ///     .theme(custom_palette);
    /// ```
    pub fn theme(mut self, palette: Palette) -> Self {
        self.palette = palette;
        self
    }

    /// Sets the background color of the image.
    pub fn background_color(mut self, color: Rgba<u8>) -> Self {
        self.background_color = color;
        self
    }

    /// Sets whether the contribution boxes should have rounded corners.
    pub fn round_corners(mut self, round: bool) -> Self {
        self.round_corners = round;
        self
    }

    /// Calculates the effective start and end dates for the graph.
    pub fn calculate_date_range(&self) -> (NaiveDate, NaiveDate) {
        let current_year = chrono::Utc::now().naive_utc().year();
        let start = self.start_date.unwrap_or_else(|| {
            if self.data.is_empty() {
                NaiveDate::from_ymd_opt(current_year, 1, 1).unwrap()
            } else {
                *self.data.keys().min().unwrap()
            }
        });

        let end = self.end_date.unwrap_or_else(|| {
            if self.data.is_empty() {
                NaiveDate::from_ymd_opt(current_year, 12, 31).unwrap()
            } else {
                *self.data.keys().max().unwrap()
            }
        });

        (start, end)
    }

    /// Calculates the dimensions (width, height) of the resulting image.
    pub fn calculate_dimensions(&self, start: NaiveDate, end: NaiveDate) -> (u32, u32) {
        let start_weekday = start.weekday().num_days_from_sunday() as i32;
        let total_days = (end - start).num_days() as i32 + 1;
        let weeks = (total_days + start_weekday + 6) / 7;

        let width = self.margin * 2 + (weeks as u32) * (self.box_size + self.gap) - self.gap;
        let height = self.margin * 2 + 7 * (self.box_size + self.gap) - self.gap;

        (width, height)
    }

    /// Calculates the (x, y) coordinates for a given date relative to the start date.
    pub fn get_coordinates(&self, date: NaiveDate, start: NaiveDate) -> (i32, i32) {
        let start_weekday = start.weekday().num_days_from_sunday() as i32;
        let day_idx = date.weekday().num_days_from_sunday() as i32;
        let days_from_start = (date - start).num_days() as i32;
        let week_idx = (days_from_start + start_weekday) / 7;

        let x = self.margin as i32 + week_idx * (self.box_size as i32 + self.gap as i32);
        let y = self.margin as i32 + day_idx * (self.box_size as i32 + self.gap as i32);

        (x, y)
    }

    /// Draws a single contribution cell onto the image buffer.
    fn draw_cell(&self, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32, color: Rgba<u8>) {
        let width = img.width();
        let height = img.height();

        if x >= 0
            && y >= 0
            && (x as u32 + self.box_size) <= width
            && (y as u32 + self.box_size) <= height
        {
            draw_filled_rect_mut(
                img,
                Rect::at(x, y).of_size(self.box_size, self.box_size),
                color,
            );

            if self.round_corners && color[3] > 0 {
                let corner_color = self.background_color;
                if (x as u32) < width && (y as u32) < height {
                    img.put_pixel(x as u32, y as u32, corner_color);
                }
                if (x as u32 + self.box_size - 1) < width && (y as u32) < height {
                    img.put_pixel(
                        (x + self.box_size as i32 - 1) as u32,
                        y as u32,
                        corner_color,
                    );
                }
                if (x as u32) < width && (y as u32 + self.box_size - 1) < height {
                    img.put_pixel(
                        x as u32,
                        (y + self.box_size as i32 - 1) as u32,
                        corner_color,
                    );
                }
                if (x as u32 + self.box_size - 1) < width && (y as u32 + self.box_size - 1) < height
                {
                    img.put_pixel(
                        (x + self.box_size as i32 - 1) as u32,
                        (y + self.box_size as i32 - 1) as u32,
                        corner_color,
                    );
                }
            }
        }
    }

    /// Generates the contribution graph image based on the current configuration.
    pub fn generate(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (start, end) = self.calculate_date_range();
        let (width, height) = self.calculate_dimensions(start, end);

        let max_count = self.data.values().cloned().max().unwrap_or(0);

        let mut img = ImageBuffer::from_pixel(width, height, self.background_color);

        let mut curr = start;
        while curr <= end {
            let count = self.data.get(&curr).cloned().unwrap_or(0);
            let color = self.palette.get_color(count, max_count);

            let (x, y) = self.get_coordinates(curr, start);
            self.draw_cell(&mut img, x, y, color);

            curr += Duration::days(1);
        }

        img
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_mapping() {
        let strategy = LinearStrategy;
        // 0 counts -> index 0
        assert_eq!(strategy.map(0, 100, 5), 0);
        // 100 counts -> index 4
        assert_eq!(strategy.map(100, 100, 5), 4);
        // 50 counts -> index 2
        assert_eq!(strategy.map(50, 100, 5), 2);
    }

    #[test]
    fn test_threshold_mapping() {
        let strategy = ThresholdStrategy::new(vec![1, 5, 10]);
        assert_eq!(strategy.map(0, 0, 4), 0);
        assert_eq!(strategy.map(3, 0, 4), 1);
        assert_eq!(strategy.map(8, 0, 4), 2);
        assert_eq!(strategy.map(15, 0, 4), 3);
    }

    #[test]
    fn test_github_theme_colors() {
        let palette = Theme::github(LinearStrategy);
        assert_eq!(palette.get_color(0, 100), Rgba([235, 237, 240, 255]));
        assert_eq!(palette.get_color(100, 100), Rgba([33, 110, 57, 255]));
    }

    #[test]
    fn test_date_range_calculation() {
        let graph = ContributionGraph::new();
        let (start, end) = graph.calculate_date_range();
        let year = chrono::Utc::now().naive_utc().year();
        assert_eq!(start, NaiveDate::from_ymd_opt(year, 1, 1).unwrap());
        assert_eq!(end, NaiveDate::from_ymd_opt(year, 12, 31).unwrap());
    }

    #[test]
    fn test_dimensions_calculation() {
        let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2023, 1, 7).unwrap();
        let graph = ContributionGraph::new().box_size(10).gap(2).margin(20);
        let (width, height) = graph.calculate_dimensions(start, end);
        assert_eq!(width, 50);
        assert_eq!(height, 122);
    }
}
