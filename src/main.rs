use anyhow::{Context, Result};
use rustyban::AppRunner;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).map(|s| s.as_str()).unwrap_or("");

    let mut terminal = ratatui::init();
    let app_result = AppRunner::new(file_name)
        .run(&mut terminal)
        .context("Failed to run the application")?;
    ratatui::restore();

    Ok(app_result)
}
