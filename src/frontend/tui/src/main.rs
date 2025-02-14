mod api;
mod app;
mod events;
mod models;
mod screens;
mod ui;

use crate::app::App;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new().await.run(&mut terminal);
    ratatui::restore();
    result
}
