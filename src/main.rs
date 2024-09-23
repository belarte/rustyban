use std::error::Error;

use rustyban::App;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = match args.get(1) {
        Some(name) => name.clone(),
        None => String::new(),
    };

    let mut terminal = ratatui::init();
    let app_result = App::new(file_name).run(&mut terminal);
    ratatui::restore();

    Ok(app_result?)
}

