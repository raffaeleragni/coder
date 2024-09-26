mod app;
mod game;
mod loader;
mod text_widget;
mod tui;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new();
    app.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
