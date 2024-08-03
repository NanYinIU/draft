use std::io;

use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;

use crate::terminal::Terminal;

pub fn run() -> Result<(), io::Error> {
    let mut terminal = Terminal::default().unwrap();
    terminal.welcome()?;
    // terminal.clear_screen()?;
    // terminal.draw_row()?;
    while let Event::Key(event) = read()? {
        if KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL) == event {
            println!("bye bye!!");
            break;
        }
        // clear screen
        if KeyEvent::new(KeyCode::Char('l'), KeyModifiers::SHIFT) == event {
            terminal.clear_screen()?;
            continue;
        }
        // clear screen
        if KeyEvent::new(KeyCode::Char('k'), KeyModifiers::SHIFT) == event {
            terminal.clear_line()?.flush()?;
            continue;
        }
        match event.code {
            KeyCode::Enter => terminal.draw_row()?,
            KeyCode::Backspace => {
                let (column, row) = terminal.move_curor_to(Direction::Left, 1)?;
                terminal.clear_line_purge()?;
                if column == 0 && row > 1 {
                    // 向上一行
                    terminal.move_curor_to(Direction::Right, 1)?;
                    // 移动到上一行的末尾位置？
                }
            }
            KeyCode::Tab => {
                terminal.move_curor_to(Direction::Right, 4)?;
            }
            KeyCode::Char(c) => terminal.print(c)?,
            _ => {}
        }
    }

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
