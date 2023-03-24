//main branch
#![allow(unused_imports, dead_code)]
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{self, Block, BorderType, Borders, Paragraph, Widget},
    Frame, Terminal,
};

const NOTE_HEIGHT: usize = 10;
const NOTE_WIDTH: usize = 20;

struct Stickynote {
    text: String,
}
impl Stickynote {
    pub fn new(_text: String) -> Stickynote {
        Stickynote { text: _text }
    }
}
struct Stack {
    notes: Vec<Stickynote>,
}
impl Stack {
    pub fn new(notes: Vec<Stickynote>) -> Stack {
        Stack { notes: notes }
    }
}

enum EditMode {
    Normal,
    Insert,
}

enum State {
    Normal,
    Editing,
}

struct App {
    edit_mode: EditMode,
    stacks: Vec<Stack>,
    state: State,
    focus: (usize, usize),
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut stack1 = Stack {
        notes: vec![
            Stickynote::new("note 11".to_string()),
            Stickynote::new("note 12".to_string()),
            Stickynote::new("note 13".to_string()),
        ],
    };

    let mut stack2 = Stack {
        notes: vec![
            Stickynote::new("note 21".to_string()),
            Stickynote::new("note 22".to_string()),
        ],
    };

    // create app and run it
    let app = App {
        edit_mode: EditMode::Normal,
        state: State::Normal,
        stacks: vec![stack1, stack2],
        focus: (0, 0),
    };

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
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let area = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Stickynote")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, area);
    // render notes
    let stacks = &app.stacks;

    for (i, stack) in stacks.iter().enumerate() {
        for (j, note) in stack.notes.iter().enumerate() {
            let block = Block::default().borders(Borders::ALL);
            let rect = Rect {
                x: u16::try_from(i * (NOTE_WIDTH + 5) + 30).unwrap(),
                y: u16::try_from((j * NOTE_HEIGHT) + 3).unwrap(),
                width: u16::try_from(NOTE_WIDTH).unwrap(),
                height: u16::try_from(NOTE_HEIGHT).unwrap(),
            };
            let p = Paragraph::new(note.text.clone())
                .alignment(layout::Alignment::Center)
                .block(block)
                .wrap(widgets::Wrap { trim: true });
            f.render_widget(p, rect);
        }
    }
}
