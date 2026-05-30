use std::collections::HashMap;

use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use contribution_grid::ContributionGraph;
use contribution_grid::Palette;
use contribution_grid::builtins::Strategy;
use contribution_grid::builtins::Theme;
use image::Rgba;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build some sample data
    let mut data = HashMap::new();
    let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 6, 30).unwrap();

    let mut curr = start;
    while curr <= end {
        data.insert(curr, (curr.day() % 8) + 1);
        curr += Duration::days(1);
    }

    // Basic usage: GitHub theme with linear strategy
    ContributionGraph::new()
        .with_data(data.clone())
        .start_date(start)
        .end_date(end)
        .theme(Theme::github(Strategy::linear()))
        .generate()
        .save("basic_github.png")?;

    // Custom palette with threshold strategy
    let custom = Palette::new(
        vec![
            Rgba([20, 20, 20, 255]),
            Rgba([0, 255, 128, 255]),
            Rgba([128, 0, 255, 255]),
            Rgba([255, 0, 128, 255]),
        ],
        Strategy::threshold(vec![2, 5, 8]),
    );
    ContributionGraph::new()
        .with_data(data)
        .start_date(start)
        .end_date(end)
        .theme(custom)
        .box_size(8)
        .gap(2)
        .round_corners(false)
        .generate()
        .save("basic_custom.png")?;

    Ok(())
}
