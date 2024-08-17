use std::{fs::read_to_string, io};

use crate::terminal::{Position, Size, Terminal};
#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
    pub gen_default: bool,
    pub size: Size,
    pub redraw: bool,
}

impl View {
    pub fn render_line(at: usize, line_text: &str) -> Result<(), io::Error> {
        // Terminal::move_to(&mut self, _)
        todo!()
    }
    pub fn render(&self) -> Result<(), io::Error> {
        // println!("self.buffer.lines:{:?}", self.buffer.lines);
        // if self.redraw {
        //     return Ok(());
        // }

        let terminal_size: Size = self.size.clone();
        // println!("terminal_size:{:?}", terminal_size);
        let h = terminal_size.height;
        let w = terminal_size.width;
        // println!("buffer.lines:{:?}", self.buffer.lines);
        if self.buffer.lines.is_empty() {
            let height_anchor = terminal_size.height / 2 as u16;
            for line in 0..h {
                Terminal::move_to((0, line))?;
                Terminal::print("~\r")?;
                if let Some(line_content) = self.buffer.lines.get(line as usize) {
                    let width_anchor = (w - u16::try_from(line_content.len()).unwrap()) / 2;
                    Terminal::move_to((width_anchor, height_anchor + line - 1))?;
                    Terminal::print(line_content)?;
                }
            }
        } else {
            for line in 0..h {
                Terminal::move_to((0, line))?;
                Terminal::print("~\r")?;
                if let Some(b_line) = self.buffer.lines.get(line as usize) {
                    // Terminal::clear_line_purge()?;
                    Terminal::println(b_line)?;
                }
            }
        }

        // Terminal::execute()?;

        Ok(())
    }

    pub fn resize(&mut self, position: Position) -> Result<(), io::Error> {
        // let terminal_size: Position = position.clone();
        // self.size = terminal_size;
        self.size = Terminal::size()?;
        self.redraw = true;
        Ok(())
    }

    pub fn refresh_screen(&mut self) -> Result<(), io::Error> {
        todo!()
    }

    pub(crate) fn load(&mut self, path: Option<std::path::PathBuf>) -> Result<(), io::Error> {
        // 读文件，填充到buffer
        if let Some(p) = path {
            let read = read_to_string(p)?;
            let lines: Vec<&str> = read.lines().collect();
            if lines.is_empty() {
                return Ok(());
            }
            // let mut view_new = View::new(false, Terminal::size()?);
            lines.clone().into_iter().for_each(|line| {
                self.buffer.lines.push(String::from(line));
            });
            self.size = Terminal::size()?;
            Terminal::move_caret_to(Position {
                col: 0,
                row: lines.len() as u16,
            })?;
        }
        self.redraw = false;
        self.render()?;
        Ok(())
    }
}
#[derive(Debug)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        // let welcome_title = String::from("welcome use draft!");
        // let version_content = String::from("version 0.0.1 ");
        // let author = String::from("by author<nanyin>");
        // let buf = vec![welcome_title, version_content, author];

        Buffer { lines: vec![] }
    }
}
