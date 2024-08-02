use crate::editor::{self, Position};
use crossterm::{
    cursor::{self, EnableBlinking, MoveTo, MoveToNextLine, SavePosition},
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
    QueueableCommand,
};
use std::{
    fmt::Display,
    io::{self, stdout, Write},
};

#[derive(Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    pub _stdout: io::Stdout,
}

impl Terminal {
    pub fn default() -> Result<Terminal, io::Error> {
        let size = terminal::size()?;
        enable_raw_mode()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout(),
        })
    }

    pub fn quit(self: &mut Self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn draw_row(self: &mut Self) -> Result<(), io::Error> {
        let _ = self
            ._stdout
            .queue(MoveToNextLine(1))?
            .queue(SavePosition)?
            .queue(Print("~\r"))?
            .flush()?;
        Ok(())
    }

    pub fn welcome(self: &mut Self) -> Result<(), io::Error> {
        // 打印欢迎标语，在当前Terminal的正中间
        let width = self.size.width;
        let height = self.size.height;
        let welcome_title = "welcome use draft!";
        let welcome_title_len = welcome_title.len();
        let column_start = (width - welcome_title_len as u16) / 2;
        println!("Terminal size is:{:?}", self.size);
        self._stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(column_start, height / 2))?
            .queue(Print(welcome_title))?;
        self.flush()?;
        Ok(())
    }
    pub fn clear_screen(self: &mut Self) -> Result<(), io::Error> {
        self._stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 1))?
            .queue(Print("~\r"))?
            .queue(EnableBlinking)?;
        // self.draw_row()?;
        self.flush()?;
        Ok(())
    }

    pub fn clear_line(self: &mut Self) -> Result<&mut Self, io::Error> {
        self._stdout
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print("~\r"))?;
        Ok(self)
    }

    pub fn clear_line_purge(self: &mut Self) -> Result<&mut Self, io::Error> {
        self._stdout.queue(Clear(ClearType::Purge))?;
        Ok(self)
    }

    pub fn print<T: Display>(self: &mut Self, msg: T) -> Result<(), io::Error> {
        let _ = self._stdout.queue(Print(msg))?.flush()?;
        Ok(())
    }

    pub fn flush(self: &mut Self) -> Result<(), io::Error> {
        self._stdout.flush()?;
        Ok(())
    }
}
