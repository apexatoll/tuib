use tuib::*;
use std::io::stdout;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

fn setup_terminal() -> Result<Terminal<impl Backend>> {
    let backend = CrosstermBackend::new(stdout());

    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    Ok(Terminal::new(backend)?)
}

fn restore_terminal() -> Result<()> {
    crossterm::execute!(stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app = App::setup().await?;

    app.run(&mut terminal).await.unwrap();

    restore_terminal()?;

    Ok(())
}
