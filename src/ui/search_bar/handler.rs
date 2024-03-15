use super::*;

impl StatefulHandler<Message> for SearchBar {
    async fn handle(&self, message: Message, app: &mut App) -> Result<()> {
        match message {
            Message::Append(char) => self.append(app, char),
            Message::Delete => self.delete(app),
            Message::None => Ok(()),
        }
    }
}

impl SearchBar {
    fn append(&self, app: &mut App, char: char) -> Result<()> {
        app.query.push(char);

        Ok(())
    }

    fn delete(&self, app: &mut App) -> Result<()> {
        app.query.pop();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui::test_helpers::*;

    macro_rules! assert_query {
        ($messages:expr, $from:expr, $to: expr) => {
            let mut query = String::from($from);
            let mut app = App { query, ..Default::default() };

            handle_messages!(&SearchBar, $messages, &mut app);

            assert_eq!(app.query, String::from($to));
        }
    }

    #[tokio::test]
    async fn it_appends_characters_to_query() {
        let messages: Vec<_> = "Input".chars().map(Message::Append).collect();

        assert_query!(messages, "", "Input");
    }

    #[tokio::test]
    async fn it_deletes_characters_from_query() {
        let messages: Vec<_> = (0..3).map(|_| Message::Delete).collect();

        assert_query!(messages, "Initial", "Init");
    }

    #[tokio::test]
    async fn it_does_not_delete_past_start_of_buffer() {
        let mut query = String::from("12345");

        let messages: Vec<_> = (0..10).map(|_| Message::Delete).collect();
        assert_query!(messages, "12345", "");

        let messages: Vec<_> = "67890".chars().map(Message::Append).collect();
        assert_query!(messages, "", "67890");
    }
}