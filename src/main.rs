mod app;
mod lunar;
mod ui;

use std::{error::Error, io, time::Duration};

use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;
        if event::poll(Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && handle_key_event(app, key)
        {
            return Ok(());
        }
    }
}

fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc => return true,
        KeyCode::Char(ch) => match ch {
            'q' | 'Q' => return true,
            'h' | 'H' => app.move_selection(-1),
            'l' | 'L' => app.move_selection(1),
            'k' | 'K' => app.move_selection(-7),
            'j' | 'J' => app.move_selection(7),
            't' | 'T' => app.back_to_today(),
            _ => {}
        },
        KeyCode::Left => app.prev_month(),
        KeyCode::Right => app.next_month(),
        KeyCode::Up => app.prev_year(),
        KeyCode::Down => app.next_year(),
        _ => {}
    }
    false
}
