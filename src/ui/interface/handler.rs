use super::*;

impl StatefulHandler<Message> for Interface {
    async fn handle(&self, message: Message, app: &mut App) -> Result<()> {
        match message {
            Message::Delegate(event) => self.delegate(event, app).await,
            Message::Quit => self.quit(app),
            Message::None => Ok(()),
        }
    }
}

impl Interface {
    async fn delegate(&self, event: KeyEvent, app: &mut App) -> Result<()> {
        match app.mode {
            Mode::Search => SearchBar.handle_event(event, app).await,
            Mode::Browse => Browser.handle_event(event, app).await,
        }
    }

    fn quit(&self, app: &mut App) -> Result<()> {
        app.is_running = false;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::SearchResult;
    use ui::test_helpers::*;

    fn char_event(char: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(char), KeyModifiers::NONE)
    }

    fn control_char_event(char: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(char), KeyModifiers::CONTROL)
    }

    fn build_results() -> Vec<SearchResult> {
        vec![
            SearchResult {
                title: String::from("Foo"),
                video_id: String::from("abc"),
            },
            SearchResult {
                title: String::from("Bar"),
                video_id: String::from("def"),
            },
            SearchResult {
                title: String::from("Baz"),
                video_id: String::from("123"),
            },
        ]
    }

    fn build_app() -> App {
        let mut cursor = ListState::default();
        cursor.select(Some(0));

        App {
            cursor,
            results: build_results(),
            is_running: true,
            ..Default::default()
        }
    }

    mod delegate {
        use super::*;

        mod in_search_mode {
            use super::*;

            #[tokio::test]
            async fn it_delegates_to_the_search_bar() {
                let mut app = build_app();

                assert_eq!(app.query, String::new());
                assert_eq!(app.cursor.selected().unwrap(), 0);

                let events: Vec<KeyEvent> = "Bonjour"
                    .chars()
                    .map(char_event)
                    .collect();

                handle_events!(&Interface, events, &mut app);

                assert_eq!(app.query, String::from("Bonjour"));
                assert_eq!(app.cursor.selected().unwrap(), 0);
            }
        }

        mod in_browse_mode {
            use super::*;

            #[tokio::test]
            async fn it_delegates_to_the_browser() {
                let mut app = App { mode: Mode::Browse, ..build_app() };

                assert_eq!(app.query, String::new());
                assert_eq!(app.cursor.selected().unwrap(), 0);

                let events: Vec<KeyEvent> = "jj"
                    .chars()
                    .map(char_event)
                    .collect();

                handle_events!(&Interface, events, &mut app);

                assert_eq!(app.query, String::new());
                assert_eq!(app.cursor.selected().unwrap(), 2);
            }
        }
    }

    mod quit {
        use super::*;

        mod in_search_mode {
            use super::*;

            #[tokio::test]
            async fn it_quits_the_program() {
                let mut app = build_app();

                assert!(app.is_running);

                let events = vec![control_char_event('c')];

                handle_events!(&Interface, events, &mut app);

                assert!(!app.is_running);
            }
        }
    }
}
