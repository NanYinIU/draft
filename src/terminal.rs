use crate::{
    editor::{self, Direction, Editor, MoveAction},
    view::View,
};
use crossterm::{
    cursor::{self, MoveTo, MoveToColumn, MoveToNextLine},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, ScrollUp,
    },
    Command,
};
use std::{
    fmt::Display,
    io::{self, stdout, Error, Write},
};

#[derive(Debug, Clone, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Default)]
pub struct Position {
    pub col: u16,
    pub row: u16,
}

pub struct Terminal;
const BANG: &str = "~";
impl Terminal {
    pub fn initialize() -> Result<(), io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), io::Error> {
        disable_raw_mode()?;
        Self::execute()?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Self::execute()?;
        Ok(())
    }
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Self::execute()?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        // clippy::as_conversions: See doc above
        #[allow(clippy::as_conversions)]
        let height = height_u16;
        // clippy::as_conversions: See doc above
        #[allow(clippy::as_conversions)]
        let width = width_u16;
        Ok(Size { height, width })
    }

    pub fn draw_row() -> Result<(), io::Error> {
        let (_, row) = cursor::position()?;
        if row > Self::size()?.height / 2 {
            Self::queue_command(ScrollUp(1))?;
        } else {
            Self::queue_command(MoveToNextLine(1))?;
        }
        Self::queue_command(MoveToColumn(0))?;
        Self::queue_command(Print(BANG))?;
        Self::queue_command(MoveToColumn(0))?;
        Self::execute()?;
        Ok(())
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn move_direction_to(
        direction: editor::Direction,
        step: u16,
    ) -> Result<(u16, u16), io::Error> {
        let position = cursor::position()?;
        let mut move_action = MoveAction::new(direction, position, step);
        let (c, r) = move_action.to_move().get_target_position();
        Terminal::move_to((c, r))?;
        Ok((c, r))
    }

    pub fn move_to((column, row): (u16, u16)) -> Result<(), io::Error> {
        Self::queue_command(MoveTo(column, row))?;
        Ok(())
    }

    pub(crate) fn move_caret_to(position: Position) -> Result<(), io::Error> {
        Self::queue_command(MoveTo(position.col, position.row))?;
        // todo!()
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Self::queue_command(Print(BANG))?;
        Self::queue_command(MoveToColumn(0))?;
        Ok(())
    }

    pub(crate) fn clear_line_purge() -> Result<(), io::Error> {
        Self::queue_command(Clear(ClearType::UntilNewLine))?;
        Self::queue_command(Print(BANG))?;
        Self::queue_command(MoveToColumn(0))?;
        Ok(())
    }

    pub fn print<T: Display>(msg: T) -> Result<(), io::Error> {
        Self::queue_command(Print(msg))?;
        Ok(())
    }
    pub fn println<T: Display>(msg: T) -> Result<(), io::Error> {
        Self::queue_command(Print(msg))?;
        Self::queue_command(MoveToNextLine(1))?;
        Self::queue_command(MoveToColumn(0))?;
        Ok(())
    }
}
