use super::*;

impl StatefulWidget for &Info {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut App) {
        let block = ui::block("Info");

        if let Some(result) = app.current_item() {
            let widths = [Constraint::Fill(1)];
            let rows = self.rows(result);
            let table = Table::new(rows, widths);

            Widget::render(table.block(block), area, buf)
        } else {
            block.render(area, buf);
        }
    }
}

impl Info {
    fn rows(&self, result: &SearchResult) -> Vec<Row> {
        self.lines(result)
            .into_iter()
            .map(|string| Row::new([string]))
            .collect()
    }

    fn lines(&self, result: &SearchResult) -> Vec<String> {
        let length = compound_duration::format_dhms(result.length);

        vec![
            result.title.to_owned(),
            result.author.to_owned(),
            result.published.to_owned(),
            length.to_string(),
            format!("{} views", result.views.to_string()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui::test_helpers::*;

    fn search_results() -> Vec<SearchResult> {
        todo!();
    }

    #[test]
    fn it_renders_empty_info_when_app_is_initialised() {
        let mut app = App::default();

        let expected = Buffer::with_lines(vec![
            "┌Info──────────────┐",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "│                  │",
            "└──────────────────┘",
        ]);

        assert_buffer!(&Info, &expected, &mut app);
    }

    #[test]
    fn it_renders_expected_info_when_result_present() {
        let result = SearchResult {
            title: String::from("Learn Rust"),
            description: String::new(),
            author: String::from("freeCodeCamp.org"),
            published: String::from("1 month ago"),
            length: 50350,
            views: 506608,
            video_id: String::from("BpPEoZW5IiY"),
        };

        let mut cursor = ListState::default();

        cursor.select(Some(0));

        let mut app = App { results: vec![result], cursor, ..Default::default() };

        let expected = Buffer::with_lines(vec![
            "┌Info──────────────┐",
            "│Learn Rust        │",
            "│freeCodeCamp.org  │",
            "│1 month ago       │",
            "│13h59m10s         │",
            "│506608 views      │",
            "└──────────────────┘",
        ]);

        assert_buffer!(&Info, &expected, &mut app);
    }
}
