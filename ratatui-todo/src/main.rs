use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::Constraint,
    style::{Style, Stylize},
};
use tui_textarea::{Input, Key, TextArea};

fn main() -> Result<()> {
    color_eyre::install()?;
    run()
}

fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut textarea = TextArea::default();

    let mut writing_mode = false;

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let layout = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .margin(1)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(size);
            let block1 = ratatui::widgets::Block::default()
                .title("Top Half")
                .borders(ratatui::widgets::Borders::ALL);

            let mut block2 = ratatui::widgets::Block::default()
                .title(if writing_mode {
                    "Bottom Half (Textarea - Writing | Esc esc to leave)"
                } else {
                    "Bottom Half (Textarea - w to write)"
                })
                .borders(ratatui::widgets::Borders::ALL);

            if writing_mode {
                block2 = block2.border_style(Style::new().green());
            }

            textarea.set_block(block2);
            f.render_widget(block1, layout[0]);
            f.render_widget(&textarea, layout[1]);
        })?;

        if writing_mode {
            match ratatui::crossterm::event::read()?.into() {
                Input {
                    key: Key::Char('s'),
                    ctrl: true,
                    ..
                } => writing_mode = false,
                Input { key: Key::Esc, .. } => writing_mode = false,
                input => {
                    textarea.input(input);
                }
            }
        }

        if !writing_mode {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
                if key.code == KeyCode::Char('w') {
                    writing_mode = true
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
