mod app;
mod models;
mod ui;

use crate::app::App;
use color_eyre::{
    eyre::WrapErr,
    Result,
};
use ratatui::{
    style::Stylize,
    widgets::Widget,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}