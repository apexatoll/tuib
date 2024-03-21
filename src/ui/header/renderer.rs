use super::*;

impl Widget for &Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = include_str!("header.txt");
        let para = Paragraph::new(text).fg(Color::Yellow);

        para.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui::test_helpers::*;

    #[test]
    fn it_renders_the_header() {
        let expected = Buffer::with_lines(vec![
            "  _        _ _      ",
            " | |_ _  _(_| |__   ",
            " |  _| || | | '_ \\  ",
            "  \\__|\\_,_|_|_.__/  ",
            "                    ",
        ]);

        assert_buffer!(&Header, &expected);
    }
}
