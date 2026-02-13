//! Built-in themes and strategy factory for the contribution grid library.
//!
//! This module provides:
//! - [`Theme`] factory for creating predefined color palettes
//! - [`Strategy`] factory for creating built-in strategy instances
//! - Built-in strategy implementations: [`LinearStrategy`], [`LogarithmicStrategy`], [`ThresholdStrategy`]

use image::Rgba;

use crate::MappingStrategy;
use crate::Palette;

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

/// Factory for creating built-in color theme palettes.
pub struct Theme;

impl Theme {
    /// Modern GitHub green style (5 colors).
    pub fn github(strategy: impl MappingStrategy + 'static) -> Palette {
        Palette::new(
            vec![
                Rgba([235, 237, 240, 255]),
                Rgba([155, 233, 168, 255]),
                Rgba([48, 196, 99, 255]),
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

/// Factory for creating built-in strategy instances.
pub struct Strategy;

impl Strategy {
    /// Creates a linear mapping strategy.
    ///
    /// # Example
    ///
    /// ```rust
    /// use contribution_grid::builtins::{Theme, Strategy};
    ///
    /// let palette = Theme::github(Strategy::linear());
    /// ```
    pub fn linear() -> impl MappingStrategy + 'static {
        LinearStrategy
    }

    /// Creates a logarithmic mapping strategy.
    ///
    /// # Example
    ///
    /// ```rust
    /// use contribution_grid::builtins::{Theme, Strategy};
    ///
    /// let palette = Theme::github(Strategy::logarithmic());
    /// ```
    pub fn logarithmic() -> impl MappingStrategy + 'static {
        LogarithmicStrategy
    }

    /// Creates a threshold-based mapping strategy.
    ///
    /// # Example
    ///
    /// ```rust
    /// use contribution_grid::builtins::{Theme, Strategy};
    ///
    /// let palette = Theme::github(Strategy::threshold(vec![1, 5, 10, 20]));
    /// ```
    pub fn threshold(thresholds: Vec<u32>) -> impl MappingStrategy + 'static {
        ThresholdStrategy::new(thresholds)
    }
}
