use crate::editor::{self, Direction, MoveAction};
use crossterm::{
    cursor::{self, EnableBlinking, MoveTo, MoveToColumn, MoveToNextLine, SavePosition},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Print,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, ScrollUp,
    },
    Command, QueueableCommand,
};
use std::{
    fmt::Display,
    io::{self, stdout, Error, Write},
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
const BANG: &str = "~";
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
    pub fn initialize() -> Result<Terminal, io::Error> {
        let size = terminal::size()?;
        enable_raw_mode()?;
        return Ok(Terminal {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout(),
        });
    }

    pub fn terminate() -> Result<(), io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn enter_alternate_screen(&mut self) -> Result<(), Error> {
        self.queue_command(EnterAlternateScreen)?;
        Ok(())
    }
    pub fn leave_alternate_screen(&mut self) -> Result<(), Error> {
        self.queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn queue_command(&mut self, command: impl Command) -> Result<(), Error> {
        self._stdout.queue(command)?.flush()?;
        Ok(())
    }

    pub fn execute_cammand() -> Result<(), Error> {
        let mut stdout = io::stdout();
        stdout.queue(LeaveAlternateScreen)?.flush()?;
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
            .queue(MoveTo(0, 0))?
            .queue(Clear(ClearType::All))?
            .queue(Print(BANG))?
            // .queue(MoveToColumn(0))?
            .queue(EnableBlinking)?;
        // self.draw_row()?;
        self.flush()?;
        Ok(())
    }

    pub fn clear_line(&mut self) -> Result<&mut Self, io::Error> {
        self._stdout
            .queue(Clear(ClearType::CurrentLine))?
            .queue(Print(BANG))?
            .queue(MoveToColumn(0))?;
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

    pub fn println<T: Display>(&mut self, msg: T) -> Result<(), io::Error> {
        self._stdout
            .queue(Print(msg))?
            .queue(MoveToNextLine(1))?
            .queue(MoveToColumn(0))?
            .flush()?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), io::Error> {
        self._stdout.flush()?;
        Ok(())
    }

    pub fn process_keyevents(&mut self) -> Result<(), io::Error> {
        while let Event::Key(event) = read()? {
            let loop_action = self.process_shortcuts(event)?;
            if LoopAction::Interrupt == loop_action {
                break;
            }
            if LoopAction::KeepOn == loop_action {
                continue;
            }

            self.process_keypress(event)?;
        }

        Ok(())
    }

    fn process_shortcuts(&mut self, event: KeyEvent) -> Result<LoopAction, io::Error> {
        return match event {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                println!("bye bye!!");
                Ok(LoopAction::Interrupt)
            }
            KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => self.clear_screen().map(|_| LoopAction::SKIP),
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => self.clear_line()?.flush().map(|_| LoopAction::SKIP),
            _ => Ok(LoopAction::SKIP),
        };
    }

    fn process_keypress(&mut self, event: KeyEvent) -> Result<(), io::Error> {
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
        Ok(())
    }
}

#[derive(PartialEq)]
enum LoopAction {
    Interrupt,
    KeepOn,
    SKIP,
}
