use super::*;

impl StatefulWidget for &Interface {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut App) {
        let [search_area, info_area, browse_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Fill(1),
        ]).areas(area);

        SearchBar.render(search_area, buf, app);
        Info.render(info_area, buf, app);
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
            "┌Info──────────────┐",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
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
                description: String::new(),
                author: String::from("freeCodeCamp.org"),
                published: String::from("1 month ago"),
                length: 50350,
                views: 506608,
                video_id: String::from("BpPEoZW5IiY"),
            },
            SearchResult {
                title: String::from("Learn Rust quicker"),
                ..Default::default()
            },
            SearchResult {
                title: String::from("Learn Rust quickest"),
                ..Default::default()
            },
        ];

        let mut app = App { query, results, ..Default::default() };

        app.cursor.select(Some(0));

        let mut expected = Buffer::with_lines(vec![
            "┌Search────────────┐",
            "│learn rust        │",
            "└──────────────────┘",
            "┌Info──────────────┐",
            "│Learn Rust quick  │",
            "│freeCodeCamp.org  │",
            "│1 month ago       │",
            "│13h59m10s         │",
            "│506608 views      │",
            "└──────────────────┘",
            "┌Results───────────┐",
            "│Learn Rust quick  │",
            "│Learn Rust quicker│",
            "│Learn Rust quickes│",
            "└──────────────────┘",
        ]);

        expected.set_style(
            Rect::new(1, 11, expected.area.width - 2, 1),
            Style::new().add_modifier(Modifier::REVERSED),
        );

        assert_buffer!(&Interface, &expected, &mut app);
    }
}
