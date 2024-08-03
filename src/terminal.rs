use crate::editor::{self, MoveAction};
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

    pub fn quit(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn draw_row(&mut self) -> Result<(), io::Error> {
        self._stdout
            .queue(MoveToNextLine(1))?
            .queue(SavePosition)?
            .queue(Print("~\r"))?
            .flush()?;
        Ok(())
    }

    pub fn move_curor_to(
        &mut self,
        direction: editor::Direction,
        step: u16,
    ) -> Result<(u16, u16), io::Error> {
        let position = cursor::position()?;
        let mut move_action = MoveAction::new(direction, position, step);
        let (c, r) = move_action.to_move().get_target_position();
        self._stdout.queue(MoveTo(c, r))?.flush()?;
        Ok((c, r))
    }

    pub fn welcome(&mut self) -> Result<(), io::Error> {
        // 打印欢迎标语，在当前Terminal的正中间
        let width = self.size.width;
        let height = self.size.height;
        let welcome_title = "welcome use draft!";
        let welcome_title_len = welcome_title.len();
        let column_start = (width - u16::try_from(welcome_title_len).unwrap()) / 2;
        println!("Terminal size is:{:?}", self.size);
        let command = self._stdout.queue(Clear(ClearType::All))?;
        let mut i = 0;
        while i < height {
            command.queue(Print("~\n\r"))?;
            i += 1;
        }
        command
            .queue(MoveTo(column_start, height / 2))?
            .queue(Print(welcome_title))?
            .queue(MoveTo(0, 0))?;
        self.flush()?;
        Ok(())
    }
    pub fn clear_screen(&mut self) -> Result<(), io::Error> {
        self._stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 1))?
            .queue(Print("~\r"))?
            .queue(EnableBlinking)?;
        // self.draw_row()?;
        self.flush()?;
        Ok(())
    }

    pub fn clear_line(&mut self) -> Result<&mut Self, io::Error> {
        self._stdout
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print("~\r"))?;
        Ok(self)
    }

    pub fn clear_line_purge(&mut self) -> Result<&mut Self, io::Error> {
        self._stdout.queue(Clear(ClearType::UntilNewLine))?;
        Ok(self)
    }

    pub fn print<T: Display>(&mut self, msg: T) -> Result<(), io::Error> {
        self._stdout.queue(Print(msg))?.flush()?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), io::Error> {
        self._stdout.flush()?;
        Ok(())
    }
}
