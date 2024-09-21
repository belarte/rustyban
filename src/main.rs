use std::error::Error;

use rustyban::app::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err("No file provided".into());
    }

    let file_name = args[1].clone();

    let mut terminal = ratatui::init();
    let app_result = App::new(file_name).run(&mut terminal);
    ratatui::restore();

    Ok(app_result?)
}

