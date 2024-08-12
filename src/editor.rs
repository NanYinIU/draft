use crate::terminal::Size;
use crate::terminal::Terminal;
use std::fs::read_to_string;
use std::io;
use std::panic::set_hook;
use std::panic::take_hook;
use std::path::PathBuf;

pub fn run(path: Option<PathBuf>) -> Result<(), io::Error> {
    let mut editor = Editor::new(path)?;
    let mut terminal = editor.terminal;
    terminal.enter_alternate_screen()?;
    // terminal.clear_screen()?;
    if editor.is_clear {
        terminal.clear_screen()?;
        editor.is_clear = false;
    }
    editor.view.render(&mut terminal)?;

    terminal.process_keyevents()?;
    terminal.leave_alternate_screen()?;
    Terminal::terminate()?;
    Ok(())
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct MoveAction {
    direction: Direction,
    move_step: u16,
    target_position: (u16, u16),
}

impl MoveAction {
    pub fn new(direction: Direction, position: (u16, u16), move_step: u16) -> Self {
        MoveAction {
            direction,
            move_step,
            target_position: position,
        }
    }

    pub fn to_move(&mut self) -> &mut Self {
        match self.direction {
            Direction::Up => {
                self.target_position.1 = self.target_position.1.saturating_sub(self.move_step);
            }
            Direction::Down => {
                self.target_position.1 = self.target_position.1.saturating_add(self.move_step);
            }
            Direction::Left => {
                self.target_position.0 = self.target_position.0.saturating_sub(self.move_step);
            }
            Direction::Right => {
                self.target_position.0 = self.target_position.0.saturating_add(self.move_step);
            }
        };
        self
    }

    pub fn get_target_position(&mut self) -> (u16, u16) {
        self.target_position
    }
}

pub struct Editor {
    terminal: Terminal,
    view: View,
    is_clear: bool,
}
impl Default for Editor {
    fn default() -> Self {
        let terminal = Terminal::default().unwrap();
        let is_clear = true;
        Editor {
            terminal,
            view: View::default(),
            is_clear,
        }
    }
}

impl Editor {
    pub fn new(path: Option<PathBuf>) -> Result<Self, io::Error> {
        let hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            hook(panic_info);
        }));
        let terminal = Terminal::initialize()?;

        match path {
            None => {
                // 初始化默认editor和view
                Ok(Editor {
                    terminal,
                    view: View::default(),
                    is_clear: true,
                })
            }
            Some(e) => {
                // println!("path,{:?}", e)
                // 读取文件，获取文件内容
                let read = read_to_string(e)?;
                let lines: Vec<&str> = read.lines().collect();
                if lines.is_empty() {
                    return Ok(Editor {
                        terminal,
                        view: View::default(),
                        is_clear: true,
                    });
                }
                let mut view_new = View::new(false);

                lines.into_iter().for_each(|line| {
                    view_new.buffer.lines.push(String::from(line));
                });
                Ok(Editor {
                    terminal,
                    view: view_new,
                    is_clear: false,
                })
            }
        }
    }
}
pub struct View {
    buffer: Buffer,
    gen_default: bool,
}

impl Default for View {
    fn default() -> Self {
        // 渲染View，前面加 ~ 这种
        View {
            buffer: Buffer::default(),
            gen_default: true,
        }
    }
}
impl View {
    pub fn new(gen_default: bool) -> Self {
        View {
            buffer: Buffer::new(),
            gen_default,
        }
    }
    pub fn render(&self, terminal: &mut Terminal) -> Result<(), io::Error> {
        let terminal_size: Size = terminal.size.clone();
        let h = terminal_size.height;
        let w = terminal_size.width;
        if self.gen_default {
            let height_anchor = terminal_size.height / 2 as u16;
            for line in 0..h {
                terminal.move_to((0, line))?;
                terminal.print("~\r")?;
                if let Some(line_content) = self.buffer.lines.get(line as usize) {
                    let width_anchor = (w - u16::try_from(line_content.len()).unwrap()) / 2;
                    terminal.move_to((width_anchor, height_anchor + line - 1))?;
                    terminal.print(line_content)?;
                }
            }
        } else {
            for line in 0..h {
                if let Some(b_line) = self.buffer.lines.get(line as usize) {
                    terminal.println(b_line)?;
                } else {
                    terminal.println("~")?;
                }
            }
        }

        terminal.flush()?;

        Ok(())
    }
}
pub struct Buffer {
    lines: Vec<String>,
}
impl Buffer {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        let welcome_title = String::from("welcome use draft!");
        let version_content = String::from("version 0.0.1 ");
        let author = String::from("by author<nanyin>");
        let buf = vec![welcome_title, version_content, author];

        Buffer { lines: buf }
    }
}
