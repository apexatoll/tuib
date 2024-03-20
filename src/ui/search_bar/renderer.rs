use super::*;

impl StatefulWidget for &SearchBar {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut Self::State) {
        let inner = area.inner(&Margin::new(1, 1));

        self.render_block(area, buf);
        self.render_query(inner, buf, app);
    }
}

impl SearchBar {
    fn render_block(&self, area: Rect, buf: &mut Buffer) {
        Block::new()
            .borders(Borders::ALL)
            .title("Search")
            .render(area, buf);
    }

    fn render_query(&self, area: Rect, buf: &mut Buffer, app: &mut App) {
        let width = area.width - Self::MARGIN;

        let scroll =
            if app.query.len() as u16 > width {
                app.query.len() as u16 - width
            } else {
                0
            };

        Paragraph::new(app.query.to_owned())
            .scroll((0, scroll))
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui::test_helpers::*;

    macro_rules! assert_render {
        ($query:expr, $buffer:expr) => {
            let mut query = String::from($query);
            let mut app = App { query, ..Default::default() };

            assert_buffer!(&SearchBar, $buffer, &mut app);
        }
    }

    #[test]
    fn it_renders_the_initial_blank_state() {
        let query = "";

        let expected = Buffer::with_lines(vec![
            "┌Search────────────┐",
            "│                  │",
            "└──────────────────┘",
        ]);

        assert_render!(query, &expected);
    }

    #[test]
    fn it_renders_the_search_bar_with_a_query() {
        let query = "Hello world!";

        let expected = Buffer::with_lines(vec![
            "┌Search────────────┐",
            "│Hello world!      │",
            "└──────────────────┘",
        ]);

        assert_render!(query, &expected);
    }

    #[test]
    fn it_renders_long_queries_correctly() {
        let query = "This is a very long search query";

        let expected = Buffer::with_lines(vec![
            "┌Search────────────┐",
            "│ng search query   │",
            "└──────────────────┘",
        ]);

        assert_render!(query, &expected);
    }
}
