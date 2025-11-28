mod app;
mod config;
mod lunar;
mod ui;

use std::{error::Error, io, time::Duration};

use app::App;
use config::{Action, BindingResolver, KeyBindings, load_key_bindings};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
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
    let key_bindings = load_key_bindings();
    let res = run_app(&mut terminal, &mut app, &key_bindings);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res?;
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    bindings: &KeyBindings,
) -> io::Result<()> {
    let mut resolver = BindingResolver::default();
    loop {
        terminal.draw(|frame| ui::draw(frame, app, bindings))?;
        if event::poll(Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            if app.jump_prompt_active() {
                handle_prompt_key(app, key);
                continue;
            }
            if let Some(action) = resolver.process(bindings, key)
                && handle_action(app, action)
            {
                return Ok(());
            }
        }
    }
}

fn handle_action(app: &mut App, action: Action) -> bool {
    match action {
        Action::Quit => true,
        Action::MoveLeft => {
            app.move_selection(-1);
            false
        }
        Action::MoveRight => {
            app.move_selection(1);
            false
        }
        Action::MoveUp => {
            app.move_selection(-7);
            false
        }
        Action::MoveDown => {
            app.move_selection(7);
            false
        }
        Action::PrevMonth => {
            app.prev_month();
            false
        }
        Action::NextMonth => {
            app.next_month();
            false
        }
        Action::PrevYear => {
            app.prev_year();
            false
        }
        Action::NextYear => {
            app.next_year();
            false
        }
        Action::BackToToday => {
            app.back_to_today();
            false
        }
        Action::OpenJumpPrompt => {
            app.start_jump_prompt();
            false
        }
    }
}

fn handle_prompt_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.cancel_jump_prompt(),
        KeyCode::Enter => app.confirm_jump_prompt(),
        KeyCode::Backspace => app.pop_jump_input(),
        KeyCode::Char(ch) => {
            if !key
                .modifiers
                .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER)
            {
                app.push_jump_input(ch);
            }
        }
        _ => {}
    }
}
