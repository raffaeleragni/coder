use std::io;

use app::App;

mod app;
mod tui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new();
    app.run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
