use super::*;

impl StatefulHandler<Message> for SearchBar {
    async fn handle(&self, message: Message, app: &mut App) -> Result<()> {
        match message {
            Message::Append(char) => self.append(app, char),
            Message::Delete => self.delete(app),
            Message::Submit => self.submit(app).await,
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

    async fn submit(&self, app: &mut App) -> Result<()> {
        app.results = api::search(
            &app.query,
            &app.instance,
            &app.client
        ).await?;

        app.mode = app::Mode::Browse;

        app.cursor.select(Some(0));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui::test_helpers::*;
    use api::SearchResult;

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

    #[tokio::test]
    async fn it_submits_the_search_and_sets_results() {
        let server = httptest::Server::run();
        let instance = api::Instance::from(&server);
        let query = String::from("search text");
        let mut app = App { instance, query, ..Default::default() };

        let response = json!([
            {
                "title": "Video one",
                "description": "Lorem ipsum",
                "author": "foo",
                "publishedText": "1 month ago",
                "lengthSeconds": 123,
                "viewCount": 20,
                "videoId": "abc",
            },
            {
                "title": "Video two",
                "description": "Dolor est",
                "author": "bar",
                "publishedText": "4 months ago",
                "lengthSeconds": 123,
                "viewCount": 200,
                "videoId": "def",
            },
            {
                "title": "Video three",
                "description": "Quia amet",
                "author": "baz",
                "publishedText": "2 months ago",
                "lengthSeconds": 123,
                "viewCount": 2000,
                "videoId": "123",
            },
        ]);

        let expected = vec![
            SearchResult {
                title: String::from("Video one"),
                description: String::from("Lorem ipsum"),
                author: String::from("foo"),
                published: String::from("1 month ago"),
                length: 123,
                views: 20,
                video_id: String::from("abc"),
            },

            SearchResult {
                title: String::from("Video two"),
                description: String::from("Dolor est"),
                author: String::from("bar"),
                published: String::from("4 months ago"),
                length: 123,
                views: 200,
                video_id: String::from("def"),
            },

            SearchResult {
                title: String::from("Video three"),
                description: String::from("Quia amet"),
                author: String::from("baz"),
                published: String::from("2 months ago"),
                length: 123,
                views: 2000,
                video_id: String::from("123"),
            },
        ];

        stub_search!(server, app.query, response);

        assert!(app.results.is_empty());
        assert!(matches!(app.mode, app::Mode::Search));
        assert!(app.cursor.selected().is_none());

        SearchBar.handle(Message::Submit, &mut app).await.unwrap();

        assert_eq!(app.results, expected);
        assert!(matches!(app.mode, app::Mode::Browse));
        assert_eq!(app.cursor.selected().unwrap(), 0);
    }
}
