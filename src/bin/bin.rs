use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use sheets_tui::app::{get_sheet, App, Sheet};
use sheets_tui::ui::ui;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let connection = sheets_tui::network::Connection::new().await;
    let mut app = App::new();
    let sheet = match connection
        .get_sheet(
            "1TmJIfNXwfYNox_uToEWvGyl0ZcyavG9Z68kox5WgjdA",
            Option::<&str>::None,
        )
        .await
    {
        Ok(res) => res,
        _ => panic!(),
    };

    app.add_sheet(Some("Sheet 1".into()), sheet);
    app.switch_sheet("Sheet 1");

    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') => app.next(),
                KeyCode::Char('k') => app.previous(),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_derive_sheet() {
        let r = r#"
        [
            {"Date":"11\/17\/2022","Happiness":"3","Steady":"2","Relaxed":"4","Balance":"7","Drive\/Goal":"4"},
            {"Date":"","Happiness":"6","Steady":"4","Relaxed":"7","Balance":"8","Drive\/Goal":"5"}]
        "#;
        let sheet: Sheet = serde_json::from_str(r).unwrap();

        println!("{:?}", sheet);
    }
}
