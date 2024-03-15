use super::*;

mod routers;
use routers::Router;

mod handlers;
use handlers::StatefulHandler;

mod search_bar;

mod browser;

mod test_helpers {
    macro_rules! assert_buffer {
        ($component:expr, $buffer:expr, $state:expr) => {
            let backend = ratatui::backend::TestBackend::new(
                $buffer.area.width,
                $buffer.area.height,
            );

            let mut terminal = Terminal::new(backend).unwrap();

            terminal.draw(|frame|
                frame.render_stateful_widget($component, frame.size(), $state)
            ).unwrap();

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

    pub(super) use assert_buffer;
    pub(super) use handle_messages;
}
