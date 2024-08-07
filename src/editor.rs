use crate::terminal::Size;
use crate::terminal::Terminal;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn run() -> Result<(), io::Error> {
    let mut editor = Editor::default();
    let mut terminal = editor.terminal;
    terminal.clear_screen()?;
    if editor.is_clear {
        terminal.clear_screen()?;
        editor.is_clear = false;
    }
    editor.view.render(&mut terminal)?;
    // terminal.welcome()?;

    terminal.process_keyevents()?;
    // terminal.draw_row()?;

    terminal.quit()?;
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
    fn new(path: &String) -> Self {
        let terminal = Terminal::default().unwrap();

        if Some(path) == None || path.trim().len() == 0 {
            // 初始化默认editor和view
            Editor {
                terminal,
                view: View::default(),
                is_clear: true,
            }
        } else {
            // 读取文件，获取文件内容
            let path = PathBuf::from("../README.md");
            fs::read(path);
            todo!()
        }
    }
}
pub struct View {
    buffer: Buffer,
}

impl Default for View {
    fn default() -> Self {
        // 渲染View，前面加 ~ 这种
        View {
            buffer: Buffer::default(),
        }
    }
}
impl View {
    pub fn new() -> Self {
        todo!()
    }
    pub fn render(&self, terminal: &mut Terminal) -> Result<(), io::Error> {
        let terminal_size: Size = terminal.size.clone();
        let h = terminal_size.height;
        let w = terminal_size.width;
        let start_h = terminal_size.height / 2 as u16;
        for line in 0..h {
            terminal.move_to((0, line))?;
            terminal.print("~\r")?;
            if let Some(b_line) = self.buffer.lines.get(line as usize) {
                let line_len = b_line.len();
                let column_start = (w - u16::try_from(line_len).unwrap()) / 2;
                terminal.move_to((column_start, start_h + line - 1))?;
                terminal.print(b_line)?;
            }
        }

        terminal.flush()?;

        Ok(())
    }
}
pub struct Buffer {
    lines: Vec<String>,
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
