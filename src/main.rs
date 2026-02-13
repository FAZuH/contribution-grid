use std::collections::HashMap;
use std::fs;
use std::path::Path;

use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use contribution_grid::ContributionGraph;
use contribution_grid::builtins::LinearStrategy;
use contribution_grid::builtins::Theme;

struct MockData;

impl MockData {
    fn random(year: i32, density: f64) -> HashMap<NaiveDate, u32> {
        let mut data = HashMap::new();
        let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        let mut curr = start;
        while curr <= end {
            let hash = (curr
                .to_string()
                .as_bytes()
                .iter()
                .map(|&b| b as u32)
                .sum::<u32>())
                * 13;
            if (hash % 100) as f64 / 100.0 < density {
                data.insert(curr, (hash % 15) + 1);
            }
            curr += Duration::days(1);
        }
        data
    }

    fn heavy_activity(year: i32) -> HashMap<NaiveDate, u32> {
        let mut data = HashMap::new();
        let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        let mut curr = start;
        while curr <= end {
            data.insert(curr, 5 + (curr.day() % 10));
            curr += Duration::days(1);
        }
        data
    }

    fn weekend_warrior(year: i32) -> HashMap<NaiveDate, u32> {
        let mut data = HashMap::new();
        let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        let mut curr = start;
        while curr <= end {
            let weekday = curr.weekday();
            if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
                data.insert(curr, 8 + (curr.day() % 5));
            }
            curr += Duration::days(1);
        }
        data
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "result/output/image output example";
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir)?;
    }

    let year = 2026;
    println!(
        "🎨 Generating example contribution graphs in '{}'...",
        output_dir
    );

    let random_data = MockData::random(year, 0.4);
    let start_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

    // 1. GitHub Theme (Standard Random Data)
    ContributionGraph::new()
        .with_data(random_data.clone())
        .start_date(start_date)
        .end_date(end_date)
        .theme(Theme::github(LinearStrategy))
        .generate()
        .save(format!("{}/github_theme_random.png", output_dir))?;
    println!("✅ Generated GitHub Theme (Random Data)");

    // 2. GitHub Old Theme (Heavy Activity)
    let heavy_data = MockData::heavy_activity(year);
    ContributionGraph::new()
        .with_data(heavy_data.clone())
        .start_date(start_date)
        .end_date(end_date)
        .theme(Theme::github_old(LinearStrategy))
        .generate()
        .save(format!("{}/github_old_theme_heavy.png", output_dir))?;
    println!("✅ Generated GitHub Old Theme (Heavy Activity)");

    // 3. Blue Theme (Weekend Warrior)
    let weekend_data = MockData::weekend_warrior(year);
    ContributionGraph::new()
        .with_data(weekend_data.clone())
        .start_date(start_date)
        .end_date(end_date)
        .theme(Theme::blue(LinearStrategy))
        .generate()
        .save(format!("{}/blue_theme_weekend.png", output_dir))?;
    println!("✅ Generated Blue Theme (Weekend Only)");

    // 4. Red Theme (Sparse Data)
    let sparse_data = MockData::random(year, 0.1);
    ContributionGraph::new()
        .with_data(sparse_data.clone())
        .start_date(start_date)
        .end_date(end_date)
        .theme(Theme::red(LinearStrategy))
        .generate()
        .save(format!("{}/red_theme_sparse.png", output_dir))?;
    println!("✅ Generated Red Theme (Sparse)");

    Ok(())
}
