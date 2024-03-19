use super::*;

impl StatefulWidget for &Browser {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut App) {
        ratatui::widgets::StatefulWidget::render(
            self.list(&app.results),
            area,
            buf,
            &mut app.cursor,
        )
    }
}

impl Browser {
    fn list(&self, results: &[SearchResult]) -> List {
        let rows = self.rows(results);

        List::new(rows)
            .block(self.block())
            .highlight_style(self.highlight_style())
    }

    fn rows(&self, results: &[SearchResult]) -> Vec<String> {
        results.iter().map(|result| result.title.to_owned()).collect()
    }

    fn block(&self) -> Block {
        Block::new()
            .borders(Borders::ALL)
            .title("Results")
    }

    fn highlight_style(&self) -> Style {
        Style::new().add_modifier(Modifier::REVERSED)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_helpers::*;

    macro_rules! assert_render {
        ($results:expr, $buffer:expr) => {
            let mut cursor = ListState::default();

            cursor.select(Some(0));

            let mut app = App {
                cursor,
                results: $results,
                ..Default::default()
            };

            assert_buffer!(&Browser, $buffer, &mut app);
        }

    }

    #[test]
    fn it_renders_the_browser_without_items() {
        let results = Vec::new();

        let expected = Buffer::with_lines(vec![
            "┌Results───────────┐",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]);

        assert_render!(results, &expected);
    }

    #[test]
    fn it_renders_the_browser_with_items() {
        let results = vec![
            SearchResult {
                title: String::from("1: Option one"),
                ..Default::default()
            },
            SearchResult {
                title: String::from("2: Option two"),
                ..Default::default()
            },
            SearchResult {
                title: String::from("3: Option three"),
                ..Default::default()
            },
        ];

        let mut expected = Buffer::with_lines(vec![
            "┌Results───────────┐",
            "│1: Option one     │",
            "│2: Option two     │",
            "│3: Option three   │",
            "└──────────────────┘",
        ]);

        expected.set_style(
            Rect::new(1, 1, expected.area.width - 2, 1),
            Style::new().add_modifier(Modifier::REVERSED),
        );

        assert_render!(results, &expected);
    }
}
