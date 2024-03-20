use super::*;
use crate::api::{Instance, SearchResult};
use crate::ui::*;
use reqwest::Client;

#[derive(Default)]
pub struct App {
    pub client: Client,
    pub instance: Instance,
    pub mode: Mode,
    pub query: String,
    pub results: Vec<SearchResult>,
    pub cursor: ListState,
    pub is_running: bool,
}

#[derive(Default)]
pub enum Mode {
    #[default] Search,
    Browse,
}

impl App {
    const SOURCE: &'static str = "https://api.invidious.io";

    pub async fn setup() -> Result<Self> {
        let client = Client::new();
        let instance = Instance::fetch(Self::SOURCE, &client).await?;

        Ok(Self {
            instance,
            client,
            ..Default::default()
        })
    }

    pub async fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        self.is_running = true;

        while self.is_running {
            terminal.draw(|frame| {
                frame.render_stateful_widget(&ui::Interface, frame.size(), self);

                if let Some((x, y)) = self.cursor_position(frame) {
                    frame.set_cursor(x, y);
                }
            })?;


            if let Event::Key(event) = event::read()? {
               Interface.handle_event(event, self).await;
            }
        }

        Ok(())
    }

    pub fn current_item(&self) -> Option<&SearchResult> {
        if let Some(index) = self.cursor.selected() {
            self.results.get(index)
        } else {
            None
        }
    }

    pub fn cursor_position(&self, frame: &Frame) -> Option<(u16, u16)> {
        match self.mode {
            Mode::Search => Some(ui::SearchBar.cursor_position(self, frame)),
            _ => None
        }
    }
}
