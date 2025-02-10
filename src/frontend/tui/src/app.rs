use color_eyre::eyre::{WrapErr};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use crate::models::screen::Screen;
use crate::models::server::Server;
use crate::models::user::User;
use crate::ui::ui;

pub struct App {
    pub running: bool,
    pub screen: Screen,
    pub logged_user: Option<User>,
    pub server: Option<Server>,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
            screen: Screen::Servers,
            logged_user: None,
            server: None
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("failed to handle events")?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        ui(frame, self);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(e) if e.kind == KeyEventKind::Press => {
                self.handle_key_event(e)
                    .wrap_err_with(|| format!("failed to handle key event:\n{e:#?}"))
            }
            _ => Ok(())
        }
    }

    fn handle_key_event(&mut self, e: KeyEvent) -> Result<()> {
        if e.modifiers.contains(KeyModifiers::CONTROL) {
            match e.code {
                KeyCode::Char('c') => self.quit(),
                KeyCode::Left => self.prev_screen(),
                KeyCode::Right => self.next_screen(),
                _ => {}
            }
        } else {
            match e.code {
                _ => {}
            }
        }
        Ok(())
    }

    fn prev_screen(&mut self) {
        self.screen = self.screen.previous();
    }

    fn next_screen(&mut self) {
        self.screen = self.screen.next();
    }

    fn quit(&mut self) {
        self.running = false;
    }
}