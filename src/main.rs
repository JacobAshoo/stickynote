#![allow(unused_imports, dead_code, unused_must_use)]
use crossterm::{
    cursor::{self, DisableBlinking, EnableBlinking, SetCursorStyle},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
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

const NOTE_HEIGHT: u16 = 10;
const NOTE_WIDTH: u16 = 20;
const INNER_MARGIN: u16 = 5;
const TOP_MARGIN: u16 = 5;

const BAR_HEIGHT: u16 = 3;

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

#[derive(PartialEq)]
enum EditMode {
    Normal,
    Insert,
}
#[derive(PartialEq)]
enum State {
    Normal,
    Editing,
}

struct App {
    edit_mode: EditMode,
    stacks: Vec<Stack>,
    state: State,
    focus: [usize; 2],
}

impl App {
    fn new_stack(&mut self) {
        self.stacks.insert(
            self.focus[0] + 1,
            Stack {
                notes: vec![Stickynote::new("".to_string())],
            },
        );
    }
    fn get_focused_note(&mut self) -> &mut Stickynote {
        return &mut self.stacks[self.focus[0]].notes[self.focus[1]];
    }

    fn add_to_stack(&mut self) {
        self.stacks[self.focus[0]].notes.push(Stickynote {
            text: "".to_string(),
        });
    }
    fn total_notes(&self) -> u16 {
        let mut total: u16 = 0;
        for stack in &self.stacks {
            total += stack.notes.len() as u16;
        }
        return total;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut stack1 = Stack {
        notes: vec![Stickynote::new("".to_string())],
    };
    let mut stack2 = Stack {
        notes: vec![
            Stickynote::new("".to_string()),
            Stickynote::new("".to_string()),
        ],
    };

    let mut stack3 = Stack {
        notes: vec![
            Stickynote::new("".to_string()),
            Stickynote::new("".to_string()),
            Stickynote::new("".to_string()),
        ],
    };
    let mut stack4 = Stack {
        notes: vec![
            Stickynote::new("".to_string()),
            Stickynote::new("".to_string()),
            Stickynote::new("".to_string()),
        ],
    };

    // create app and run it
    let app = App {
        edit_mode: EditMode::Normal,
        state: State::Normal,
        stacks: vec![stack1, stack2, stack3, stack4],
        focus: [0, 0],
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
    let mut prev_key: KeyCode;
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if app.state == State::Normal {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
                if let KeyCode::Char('n') = key.code {
                    app.add_to_stack();
                }
                if let KeyCode::Char('s') = key.code {
                    app.new_stack();
                }

                if let KeyCode::Char('d') = key.code {
                    if app.total_notes() == 1 {
                        app.get_focused_note().text = "".to_string();
                        continue;
                    }
                    app.stacks[app.focus[0]].notes.remove(app.focus[1]);

                    //remove stack if empty
                    if app.stacks[app.focus[0]].notes.len() == 0 {
                        app.stacks.remove(app.focus[0]);
                    }

                    if app.focus[0] > app.stacks.len() - 1 {
                        app.focus[0] = app.stacks.len() - 1;
                    }
                    if app.focus[0] < 0 {
                        app.focus[0] = 0;
                    }
                    if app.focus[1] > app.stacks[app.focus[0]].notes.len() - 1 {
                        app.focus[1] = app.stacks[app.focus[0]].notes.len() - 1;
                    }
                }
                if let KeyCode::Char('h') = key.code {
                    // if focused note is at index 0
                    if app.focus[0] == 0 {
                        continue;
                    } else {
                        let current_stack_len = app.stacks[app.focus[0]].notes.len();
                        let next_stack_len = app.stacks[app.focus[0] - 1].notes.len();
                        if app.focus[1] > next_stack_len - 1 {
                            app.focus[1] = next_stack_len - 1;
                        }
                    }

                    app.focus[0] -= 1;
                }
                if let KeyCode::Char('l') = key.code {
                    // if focused note is at end
                    if app.focus[0] == app.stacks.len() - 1 {
                        continue;
                    } else {
                        let current_stack_len = app.stacks[app.focus[0]].notes.len();
                        let next_stack_len = app.stacks[app.focus[0] + 1].notes.len();
                        if app.focus[1] > next_stack_len - 1 {
                            app.focus[1] = next_stack_len - 1;
                        }
                        app.focus[0] += 1;
                    }
                }
                if let KeyCode::Char('j') = key.code {
                    if app.focus[1] == app.stacks[app.focus[0]].notes.len() - 1 {
                        app.focus[1] = 0;
                        continue;
                    }
                    app.focus[1] += 1;
                }
                if let KeyCode::Char('k') = key.code {
                    if app.focus[1] == 0 {
                        app.focus[1] = app.stacks[app.focus[0]].notes.len() - 1;
                        continue;
                    }
                    app.focus[1] -= 1;
                }
                if let KeyCode::Left = key.code {
                    // if focused note is at index 0
                    if app.focus[0] == 0 {
                        continue;
                    } else {
                        let current_stack_len = app.stacks[app.focus[0]].notes.len();
                        let next_stack_len = app.stacks[app.focus[0] - 1].notes.len();
                        if app.focus[1] > next_stack_len - 1 {
                            app.focus[1] = next_stack_len - 1;
                        }
                    }

                    app.focus[0] -= 1;
                }

                if let KeyCode::Right = key.code {
                    // if focused note is at end
                    if app.focus[0] == app.stacks.len() - 1 {
                        continue;
                    } else {
                        let current_stack_len = app.stacks[app.focus[0]].notes.len();
                        let next_stack_len = app.stacks[app.focus[0] + 1].notes.len();
                        if app.focus[1] > next_stack_len - 1 {
                            app.focus[1] = next_stack_len - 1;
                        }
                        app.focus[0] += 1;
                    }
                }

                if let KeyCode::Down = key.code {
                    if app.focus[1] == app.stacks[app.focus[0]].notes.len() - 1 {
                        app.focus[1] = 0;
                        continue;
                    }
                    app.focus[1] += 1;
                }
                if let KeyCode::Up = key.code {
                    if app.focus[1] == 0 {
                        app.focus[1] = app.stacks[app.focus[0]].notes.len() - 1;
                        continue;
                    }
                    app.focus[1] -= 1;
                }
                if let KeyCode::Char('e') = key.code {
                    app.state = State::Editing;
                    app.edit_mode = EditMode::Normal;
                }
                if let KeyCode::Char('i') = key.code {
                    app.state = State::Editing;
                    app.edit_mode = EditMode::Insert;
                }
                prev_key = key.code;
            }
            if app.state == State::Editing {
                if app.edit_mode == EditMode::Normal {
                    if let KeyCode::Esc = key.code {
                        app.state = State::Normal;
                    }

                    if let KeyCode::Char('i') = key.code {
                        app.edit_mode = EditMode::Insert;
                    }
                }

                if app.edit_mode == EditMode::Insert {
                    if let KeyCode::Esc = key.code {
                        app.edit_mode = EditMode::Normal;
                    }
                }
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
    let total_stacks_length = (app.stacks.len() as u16) * (INNER_MARGIN + NOTE_WIDTH);

    let first_x = ((area.width - total_stacks_length) / 2) - (NOTE_WIDTH / 2) + INNER_MARGIN;
    let mut first_ys: Vec<u16> = Vec::new();

    for (i, stack) in stacks.iter().enumerate() {
        let total_stack_height = (stack.notes.len() as u16) * (INNER_MARGIN + NOTE_HEIGHT);
        if total_stack_height > area.height {
            continue;
        }
        let first_y = ((area.height - total_stack_height) / 2)
            + (INNER_MARGIN / 2 * stack.notes.len() as u16)
            + INNER_MARGIN / 2;
        first_ys.push(first_y);

        for (j, note) in stack.notes.iter().enumerate() {
            let rect = Rect::new(
                first_x + (i as u16 * (NOTE_WIDTH + INNER_MARGIN)),
                first_y + (j as u16 * (NOTE_HEIGHT / 2 + INNER_MARGIN)),
                NOTE_WIDTH,
                NOTE_HEIGHT,
            );
            let mut p = Paragraph::new(note.text.clone())
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center)
                .wrap(widgets::Wrap { trim: true });

            // focused block
            if i == app.focus[0] && j == app.focus[1] {
                p = p.block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                );
            }
            f.render_widget(p, rect);

            // status bar
            let rect = Rect::new(area.width / 2 - 4, 1, 8, BAR_HEIGHT);
            let text = match app.state {
                State::Normal => "Normal".to_string(),
                State::Editing => match app.edit_mode {
                    EditMode::Normal => "Edit".to_string(),
                    EditMode::Insert => "Insert".to_string(),
                },
            };
            let p = Paragraph::new(text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .alignment(Alignment::Center);
            f.render_widget(p, rect);
        }
    }

    // edit mode
    if app.state == State::Editing {
        f.set_cursor(
            first_x + (app.focus[0] as u16 * (NOTE_WIDTH + INNER_MARGIN)) + 1,
            first_ys[app.focus[0]] + (app.focus[1] as u16 * (NOTE_HEIGHT / 2 + INNER_MARGIN)) + 1,
        );
    }

    // normal edit mode
    if app.edit_mode == EditMode::Normal {
        io::stdout()
            .execute(DisableBlinking)
            .unwrap()
            .execute(SetCursorStyle::DefaultUserShape);
    }

    // insert mode
    if app.edit_mode == EditMode::Insert {
        let mut stdout = io::stdout();
        execute!(stdout, SetCursorStyle::BlinkingBar, EnableBlinking);
    }
}
