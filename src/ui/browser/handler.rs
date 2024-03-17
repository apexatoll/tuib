use super::*;

impl StatefulHandler<Message> for Browser {
    async fn handle(&self, message: Message, app: &mut App) -> Result<()> {
        match message {
            Message::Up => self.prev(app),
            Message::Down => self.next(app),
            Message::Select => todo!(),
            Message::None => (),
        }

        Ok(())
    }
}

impl Browser {
    fn next(&self, app: &mut App) {
        if let Some(index) = app.cursor.selected() {
            if index == app.results.len() - 1 {
                app.cursor.select(Some(0));
            } else {
                app.cursor.select(Some(index + 1));
            }
        } else {
            app.cursor.select(Some(0))
        }
    }

    fn prev(&self, app: &mut App) {
        if let Some(index) = app.cursor.selected() {
            if index == 0 {
                app.cursor.select(Some(app.results.len() - 1));
            } else {
                app.cursor.select(Some(index - 1));
            }
        } else {
            app.cursor.select(Some(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_results() -> Vec<SearchResult> {
        vec![
            SearchResult {
                title: String::from("1: Option one"),
                video_id: String::from("abc"),
            },
            SearchResult {
                title: String::from("2: Option two"),
                video_id: String::from("def"),
            },
            SearchResult {
                title: String::from("3: Option three"),
                video_id: String::from("123"),
            },
        ]
    }

    fn build_cursor(index: Option<usize>) -> ListState {
        let mut cursor = ListState::default();

        if let Some(index) = index {
            cursor.select(Some(index));
        }

        cursor
    }

    macro_rules! assert_changes_cursor {
        ($message:expr, $from:expr, $to:expr) => {
            let results = build_results();
            let cursor = build_cursor($from);

            assert_eq!(cursor.selected(), $from);

            let mut app = App {
                results,
                cursor,
                ..Default::default()
            };

            Browser.handle($message, &mut app).await;

            assert_eq!(app.cursor.selected(), $to);
        }
    }

    mod up {
        use super::*;

        mod cursor_not_initially_selected {
            use super::*;

            #[tokio::test]
            async fn it_sets_the_cursor_to_zero() {
                assert_changes_cursor!(Message::Up, None, Some(0));
            }
        }

        mod cursor_initially_selected {
            use super::*;

            mod cursor_not_at_first_index {
                use super::*;
                
                #[tokio::test]
                async fn it_decrements_the_cursor() {
                    assert_changes_cursor!(Message::Up, Some(2), Some(1));
                }
            }

            mod cursor_at_first_index {
                use super::*;

                #[tokio::test]
                async fn it_wraps_the_cursor_to_the_last_index() {
                    assert_changes_cursor!(Message::Up, Some(0), Some(2));
                }
            }
        }
    }

    mod down {
        use super::*;

        mod cursor_not_initially_selected {
            use super::*;

            #[tokio::test]
            async fn it_sets_the_cursor_to_zero() {
                assert_changes_cursor!(Message::Down, None, Some(0));
            }
        }

        mod cursor_initially_selected {
            use super::*;

            mod cursor_not_at_last_index {
                use super::*;
                
                #[tokio::test]
                async fn it_increments_the_cursor() {
                    assert_changes_cursor!(Message::Down, Some(1), Some(2));
                }
            }

            mod cursor_at_last_index {
                use super::*;

                #[tokio::test]
                async fn it_wraps_the_cursor_to_zero() {
                    assert_changes_cursor!(Message::Down, Some(2), Some(0));
                }
            }
        }
    }
}