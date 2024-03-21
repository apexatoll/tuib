use super::*;

mod routers;
use routers::Router;

mod handlers;
pub use handlers::StatefulHandler;

mod search_bar;
pub use search_bar::SearchBar;

mod browser;
use browser::Browser;

mod info;
use info::Info;

mod interface;
pub use interface::Interface;

mod test_helpers {
    pub use serde_json::json;
    pub use httptest::{Server, Expectation, matchers::*, responders::*};

    macro_rules! assert_buffer {
        ($component:expr, $buffer:expr, $state:expr) => {
            let backend = ratatui::backend::TestBackend::new(
                $buffer.area.width,
                $buffer.area.height,
            );

            let mut terminal = Terminal::new(backend).unwrap();

            terminal.draw(|frame| {
                let size = frame.size();

                let reset = Style::new()
                    .fg(Color::Reset)
                    .bg(Color::Reset)
                    .remove_modifier(Modifier::BOLD);

                frame.render_stateful_widget($component, size, $state);
                frame.buffer_mut().set_style(size, reset);
            }).unwrap();


            terminal.backend().assert_buffer($buffer);
        }
    }

    macro_rules! handle_messages {
        ($component:expr, $messages:expr, $state:expr) => {
            for message in $messages.into_iter() {
                $component.handle(message, $state).await.unwrap();
            }
        }
    }

    macro_rules! handle_events {
        ($component:expr, $events:expr, $state:expr) => {
            for event in $events.into_iter() {
                $component.handle_event(event, $state).await.unwrap();
            }
        }
    }

    macro_rules! stub_search {
        ($server:expr, $query:expr, $response:expr) => {
            $server.expect(
                Expectation::matching(
                    all_of![
                        request::method_path("GET", "/api/v1/search"),
                        request::query(url_decoded(contains(("q", $query.clone())))),
                    ]
                ).respond_with(json_encoded($response))
            );
        }
    }

    pub(super) use assert_buffer;
    pub(super) use handle_messages;
    pub(super) use handle_events;
    pub(super) use stub_search;
}
