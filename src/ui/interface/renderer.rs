use super::*;

impl StatefulWidget for &Interface {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut App) {
        let [search_area, browse_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
        ]).areas(area);

        SearchBar.render(search_area, buf, app);
        Browser.render(browse_area, buf, app);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::SearchResult;
    use ui::test_helpers::*;

    #[test]
    fn it_renders_the_initially_blank_application() {
        let mut app = App::default();

        let expected = Buffer::with_lines(vec![
            "┌Search────────────┐",
            "│                  │",
            "└──────────────────┘",
            "┌Results───────────┐",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]);

        assert_buffer!(&Interface, &expected, &mut app);
    }

    #[test]
    fn it_renders_the_application_with_state() {
        let mut query = String::from("learn rust");

        let results = vec![
            SearchResult {
                title: String::from("Learn Rust quick"),
                video_id: String::from("abc"),
            },
            SearchResult {
                title: String::from("Learn Rust quicker"),
                video_id: String::from("def"),
            },
            SearchResult {
                title: String::from("Learn Rust quickest"),
                video_id: String::from("123"),
            },
        ];

        let mut app = App { query, results, ..Default::default() };

        app.cursor.select(Some(0));

        let mut expected = Buffer::with_lines(vec![
            "┌Search────────────┐",
            "│learn rust        │",
            "└──────────────────┘",
            "┌Results───────────┐",
            "│Learn Rust quick  │",
            "│Learn Rust quicker│",
            "│Learn Rust quickes│",
            "└──────────────────┘",
        ]);

        expected.set_style(
            Rect::new(1, 4, expected.area.width - 2, 1),
            Style::new().add_modifier(Modifier::REVERSED),
        );

        assert_buffer!(&Interface, &expected, &mut app);
    }
}
