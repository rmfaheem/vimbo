use std::io;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

/// Simple Vim cheatsheet and search helper.
#[derive(Parser, Debug)]
#[command(name = "vimbo", version, about = "Terminal Vim cheatsheet and helper")]
struct Cli {
    /// Optional initial search query (e.g. 'copy', 'paste', 'delete')
    #[arg(short, long)]
    query: Option<String>,
}

#[derive(Clone)]
struct CheatEntry {
    category: &'static str,
    command: &'static str,
    description: &'static str,
}

struct App {
    cheats: Vec<CheatEntry>,
    filtered: Vec<usize>,
    query: String,
    selected: usize,
    show_help: bool,
}

impl App {
    fn new(initial_query: Option<String>) -> Self {
        let cheats = default_cheats();
        let mut app = Self {
            cheats,
            filtered: Vec::new(),
            query: initial_query.unwrap_or_default(),
            selected: 0,
            show_help: false,
        };
        app.apply_filter();
        app
    }

    fn apply_filter(&mut self) {
        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered = (0..self.cheats.len()).collect();
        } else {
            self.filtered = self
                .cheats
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    let haystack = format!(
                        "{} {} {}",
                        c.category.to_lowercase(),
                        c.command.to_lowercase(),
                        c.description.to_lowercase()
                    );
                    haystack.contains(&q).then_some(i)
                })
                .collect();
        }
        debug!("filter updated; query='{}', shown={}", self.query, self.filtered.len());
        if self.selected >= self.filtered.len() {
            self.selected = self.filtered.len().saturating_sub(1);
        }
    }
}

fn default_cheats() -> Vec<CheatEntry> {
    vec![
        // Basics
        CheatEntry {
            category: "Basics",
            command: ":q",
            description: "quit (fails if there are unsaved changes)",
        },
        CheatEntry {
            category: "Basics",
            command: ":q!",
            description: "quit discarding changes",
        },
        CheatEntry {
            category: "Basics",
            command: ":w",
            description: "write (save) current buffer",
        },
        CheatEntry {
            category: "Basics",
            command: ":wq / :x / ZZ",
            description: "save and quit",
        },
        CheatEntry {
            category: "Basics",
            command: ":e {file}",
            description: "edit / open file",
        },
        CheatEntry {
            category: "Basics",
            command: ":help {topic}",
            description: "open Vim help (e.g. :help motion)",
        },

        CheatEntry {
            category: "Modes",
            command: "i",
            description: "enter insert mode before cursor",
        },
        CheatEntry {
            category: "Modes",
            command: "a",
            description: "enter insert mode after cursor",
        },
        CheatEntry {
            category: "Modes",
            command: "v",
            description: "enter visual mode",
        },
        CheatEntry {
            category: "Modes",
            command: "V",
            description: "enter visual line mode",
        },
        CheatEntry {
            category: "Modes",
            command: "Ctrl + v",
            description: "enter visual block (blockwise) mode",
        },
        CheatEntry {
            category: "Modes",
            command: "Esc",
            description: "return to normal mode",
        },

        CheatEntry {
            category: "Navigation - line",
            command: "h j k l",
            description: "move cursor left / down / up / right",
        },
        CheatEntry {
            category: "Navigation - line",
            command: "0 / $",
            description: "move cursor to start / end of line",
        },
        CheatEntry {
            category: "Navigation - line",
            command: "^",
            description: "move cursor to first non-blank in line",
        },
        CheatEntry {
            category: "Navigation - scrolling",
            command: "Ctrl + u / Ctrl + d",
            description: "move view half-page up / down",
        },
        CheatEntry {
            category: "Navigation - scrolling",
            command: "Ctrl + b / Ctrl + f",
            description: "move view page up / down",
        },
        CheatEntry {
            category: "Navigation - file",
            command: "gg / G",
            description: "move cursor to first / last line of file",
        },
        CheatEntry {
            category: "Navigation - file",
            command: "{n}G",
            description: "move cursor to line {n}",
        },
        CheatEntry {
            category: "Navigation - screen",
            command: "H / M / L",
            description: "move cursor to top / middle / bottom of screen",
        },
        CheatEntry {
            category: "Navigation - screen",
            command: "zz / zt / zb",
            description: "move view to center / top / bottom current line",
        },
        CheatEntry {
            category: "Navigation - paragraphs",
            command: "{ / }",
            description: "move cursor to previous / next paragraph or block",
        },
        CheatEntry {
            category: "Navigation - sentences",
            command: "( / )",
            description: "move cursor to previous / next sentence",
        },
        CheatEntry {
            category: "Navigation - matching",
            command: "%",
            description: "move cursor to matching bracket/brace/paren",
        },
        CheatEntry {
            category: "Navigation - word",
            command: "w / b / e",
            description: "move cursor to next / previous / end of word",
        },
        CheatEntry {
            category: "Navigation - word",
            command: "W / B / E",
            description: "move cursor WORD-wise next / previous / end",
        },
        CheatEntry {
            category: "Navigation - find",
            command: "f{char} / F{char}",
            description: "move cursor to char right / left",
        },
        CheatEntry {
            category: "Navigation - find",
            command: "t{char} / T{char}",
            description: "move cursor till before char right / left",
        },
        CheatEntry {
            category: "Navigation - find",
            command: "; / ,",
            description: "move cursor by repeating / reversing last f/F/t/T",
        },

        CheatEntry {
            category: "Editing",
            command: "x",
            description: "delete character under cursor",
        },
        CheatEntry {
            category: "Editing",
            command: "dd",
            description: "delete (cut) current line",
        },
        CheatEntry {
            category: "Editing",
            command: "D",
            description: "delete from cursor to end of line",
        },
        CheatEntry {
            category: "Editing",
            command: "cc",
            description: "change (replace) entire line",
        },
        CheatEntry {
            category: "Editing",
            command: "cw / c$",
            description: "change to end of word / line",
        },
        CheatEntry {
            category: "Editing",
            command: "r{char}",
            description: "replace a single character",
        },
        CheatEntry {
            category: "Editing",
            command: "J",
            description: "join current line with next",
        },
        CheatEntry {
            category: "Yank (copy)",
            command: "y{motion}",
            description: "yank text covered by a motion (e.g. yw, y$)",
        },
        CheatEntry {
            category: "Yank (copy)",
            command: "yy / Y",
            description: "yank (copy) current line",
        },
        CheatEntry {
            category: "Yank (copy)",
            command: "yiw / yaw",
            description: "yank inner word / a word incl. space",
        },
        CheatEntry {
            category: "Yank (copy)",
            command: "y0 / y$",
            description: "yank from cursor to start / end of line",
        },
        CheatEntry {
            category: "Paste",
            command: "p / P",
            description: "paste after / before cursor or line",
        },
        CheatEntry {
            category: "Paste",
            command: "gp / gP",
            description: "paste and move cursor to end of paste",
        },
        CheatEntry {
            category: "Indentation",
            command: ">> / <<",
            description: "indent / dedent current line",
        },
        CheatEntry {
            category: "Indentation",
            command: "=",
            description: "auto-indent motion or selection",
        },

        CheatEntry {
            category: "Visual mode",
            command: "v / V / Ctrl + v + motion",
            description: "select characters / lines / block",
        },
        CheatEntry {
            category: "Visual mode",
            command: "y / d / c",
            description: "yank / delete / change selection",
        },
        CheatEntry {
            category: "Visual mode",
            command: "> / <",
            description: "indent / dedent selection",
        },

        CheatEntry {
            category: "Search",
            command: "/pattern",
            description: "search forward for pattern",
        },
        CheatEntry {
            category: "Search",
            command: "n / N",
            description: "next / previous search match",
        },
        CheatEntry {
            category: "Search",
            command: "?pattern",
            description: "search backward for pattern",
        },
        CheatEntry {
            category: "Search & replace",
            command: ":%s/old/new/g",
            description: "replace all 'old' with 'new' in file",
        },
        CheatEntry {
            category: "Search & replace",
            command: ":%s/old/new/gc",
            description: "replace with confirmation",
        },

        CheatEntry {
            category: "Buffers",
            command: ":w / :q / :wq",
            description: "write, quit, write & quit",
        },
        CheatEntry {
            category: "Buffers",
            command: ":ls / :buffers",
            description: "list buffers",
        },
        CheatEntry {
            category: "Buffers",
            command: ":b {n}",
            description: "go to buffer {n}",
        },
        CheatEntry {
            category: "Buffers",
            command: ":bn / :bp",
            description: "next / previous buffer",
        },

        CheatEntry {
            category: "Windows",
            command: ":split / :vsplit",
            description: "horizontal / vertical split",
        },
        CheatEntry {
            category: "Windows",
            command: "Ctrl + w, then h/j/k/l",
            description: "move to window left/down/up/right",
        },
        CheatEntry {
            category: "Windows",
            command: "Ctrl + w, then c / o",
            description: "close current / keep only current",
        },

        CheatEntry {
            category: "Tabs",
            command: ":tabnew {file}",
            description: "open file in a new tab",
        },
        CheatEntry {
            category: "Tabs",
            command: "gt / gT",
            description: "next / previous tab",
        },
        CheatEntry {
            category: "Tabs",
            command: ":tabclose",
            description: "close current tab",
        },

        CheatEntry {
            category: "Registers",
            command: "\"{reg}y / \"{reg}p",
            description: "yank / paste using register {reg}",
        },
        CheatEntry {
            category: "Registers",
            command: "\"+y / \"+p / \"*y",
            description: "use system clipboards (+ or * register)",
        },

        CheatEntry {
            category: "Marks",
            command: "m{a-z}",
            description: "set mark {a-z} on a line",
        },
        CheatEntry {
            category: "Marks",
            command: "'{a-z} / `{a-z}",
            description: "jump to mark line / exact position",
        },

        CheatEntry {
            category: "Macros",
            command: "q{reg} ... q",
            description: "record macro into register {reg}",
        },
        CheatEntry {
            category: "Macros",
            command: "@{reg} / @@",
            description: "play macro / repeat last macro",
        },

        CheatEntry {
            category: "Repeat",
            command: ".",
            description: "repeat last change",
        },
        CheatEntry {
            category: "Undo/Redo",
            command: "u / Ctrl + r",
            description: "undo / redo last change",
        },
    ]
}

fn main() -> Result<()> {
    env_logger::init();
    debug!("starting vimbo");
    let cli = Cli::parse();
    let mut app = App::new(cli.query);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    debug!("exiting vimbo");

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let constraints = if app.show_help {
                vec![
                    Constraint::Length(3), // search bar
                    Constraint::Min(5),    // list
                    Constraint::Length(5), // help pane
                ]
            } else {
                vec![
                    Constraint::Length(3), // search bar
                    Constraint::Min(5),    // list
                    Constraint::Length(1), // status
                ]
            };
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(size);

            // Search input
            let search = Paragraph::new(app.query.as_str())
                .block(
                    Block::default()
                        .title(Span::styled(
                            " Search (type to filter, Esc to quit) ",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(search, chunks[0]);

            // Cheats list
            let items: Vec<ListItem> = app
                .filtered
                .iter()
                .map(|&idx| {
                    let c = &app.cheats[idx];
                    let line = Line::from(vec![
                        Span::styled(
                            format!("[{}] ", c.category),
                            Style::default().fg(Color::Magenta),
                        ),
                        Span::styled(
                            format!("{:<12}", c.command),
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" "),
                        Span::styled(
                            c.description,
                            Style::default().fg(Color::White),
                        ),
                    ]);
                    ListItem::new(line)
                })
                .collect();

            let cheats_block = List::new(items)
                .block(
                    Block::default()
                        .title(Span::styled(
                            " Vim Cheatsheet ",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(
                cheats_block,
                chunks[1],
                &mut ratatui::widgets::ListState::default()
                    .with_selected(if app.filtered.is_empty() {
                        None
                    } else {
                        Some(app.selected)
                    }),
            );

            if app.show_help {
                let help = Paragraph::new(
                    "Keys: ↑/↓ move  •  PgUp/PgDn scroll  •  g/G top/bottom\n\
                     Typing filters cheats  •  Backspace deletes  •  / clears query\n\
                     ? toggle this help  •  Esc to quit",
                )
                .block(
                    Block::default()
                        .title(Span::styled(
                            " Help ",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White));
                f.render_widget(help, chunks[2]);
            } else {
                // Status bar
                let status_text = format!(
                    "Total: {}  Shown: {}  (? for help)",
                    app.cheats.len(),
                    app.filtered.len()
                );
                let status = Paragraph::new(status_text).style(
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                );
                f.render_widget(status, chunks[2]);
            }
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => {
                    debug!("key: {:?}", key.code);
                    match key.code {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Char('?') => {
                            app.show_help = !app.show_help;
                        }
                        KeyCode::Up => {
                            if app.selected > 0 {
                                app.selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.selected + 1 < app.filtered.len() {
                                app.selected += 1;
                            }
                        }
                        KeyCode::PageUp => {
                            let step = 10;
                            app.selected = app.selected.saturating_sub(step);
                        }
                        KeyCode::PageDown => {
                            let step = 10;
                            if app.selected + step < app.filtered.len() {
                                app.selected += step;
                            } else if !app.filtered.is_empty() {
                                app.selected = app.filtered.len() - 1;
                            }
                        }
                        KeyCode::Char('g') => {
                            if key.modifiers.is_empty() {
                                app.selected = 0;
                            }
                        }
                        KeyCode::Char('G') => {
                            if !app.filtered.is_empty() {
                                app.selected = app.filtered.len() - 1;
                            }
                        }
                        KeyCode::Char('/') => {
                            app.query.clear();
                            app.apply_filter();
                        }
                        KeyCode::Backspace => {
                            app.query.pop();
                            app.apply_filter();
                        }
                        KeyCode::Char(c) => {
                            app.query.push(c);
                            app.apply_filter();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
