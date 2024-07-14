mod app;
mod game;
mod text_widget;
mod tui;
mod loader;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new();
    app.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
