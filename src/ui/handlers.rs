use super::*;

pub trait StatefulHandler<Message>
    where Self: Router<Message>,
    for<'a> &'a Self: StatefulWidget {

    async fn handle(&self, message: Message, app: &mut App) -> Result<()>;

    async fn handle_event(&self, event: KeyEvent, app: &mut App) -> Result<()> {
        let message = Self::route(event);

        self.handle(message, app).await?;

        Ok(())
    }
}
