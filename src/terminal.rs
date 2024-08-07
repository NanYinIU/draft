use crate::editor::{self, MoveAction};
use crossterm::{
    cursor::{self, EnableBlinking, MoveTo, MoveToNextLine, SavePosition},
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType, ScrollUp},
    QueueableCommand,
};
use std::{
    fmt::Display,
    io::{self, stdout, Write},
};

#[derive(Debug, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    pub size: Size,
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
        let (_, row) = cursor::position()?;
        if row > self.size.height / 2 {
            self._stdout.queue(ScrollUp(1))?.queue(MoveTo(0, row))?;
        } else {
            self._stdout.queue(MoveToNextLine(1))?;
        }
        self._stdout.queue(SavePosition)?.flush()?;
        Ok(())
    }

    pub fn move_direction_to(
        &mut self,
        direction: editor::Direction,
        step: u16,
    ) -> Result<(u16, u16), io::Error> {
        let position = cursor::position()?;
        let mut move_action = MoveAction::new(direction, position, step);
        let (c, r) = move_action.to_move().get_target_position();
        self.move_to((c, r))?;
        Ok((c, r))
    }

    pub fn move_to(&mut self, (column, row): (u16, u16)) -> Result<(), io::Error> {
        self._stdout.queue(MoveTo(column, row))?.flush()?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), io::Error> {
        self._stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?
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
