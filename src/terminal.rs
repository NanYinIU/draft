use crate::editor::{self, Direction, MoveAction};
use crossterm::{
    cursor::{self, EnableBlinking, MoveTo, MoveToNextLine, SavePosition},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
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

    pub fn process_key_events(&mut self) -> Result<(), io::Error> {
        while let Event::Key(event) = read()? {
            if KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL) == event {
                println!("bye bye!!");
                break;
            }
            // clear screen
            if KeyEvent::new(KeyCode::Char('l'), KeyModifiers::SHIFT) == event {
                self.clear_screen()?;
                continue;
            }
            // clear screen
            if KeyEvent::new(KeyCode::Char('k'), KeyModifiers::SHIFT) == event {
                self.clear_line()?.flush()?;
                continue;
            }
            // if editor.is_clear {
            //     self.clear_screen()?;
            //     self.is_clear = false;
            // }
            match event.code {
                KeyCode::Enter => self.draw_row()?,
                KeyCode::Backspace => {
                    let (column, row) = self.move_direction_to(Direction::Left, 1)?;
                    // println!("this (column, row) is {:?}", (column, row));
                    self.clear_line_purge()?;
                    if column == 0 && row >= 1 {
                        // 向上一行
                        self.move_direction_to(Direction::Up, 1)?;
                        // 移动到上一行的末尾位置？
                    }
                }
                KeyCode::Tab => {
                    self.move_direction_to(Direction::Right, 4)?;
                }
                KeyCode::Char(c) => self.print(c)?,
                _ => {}
            }
        }
        Ok(())
    }
}
