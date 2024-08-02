use std::io;
use std::io::Write;

use crossterm::cursor::MoveTo;

use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;

use crossterm::QueueableCommand;

use crate::terminal::Terminal;

pub fn run() -> Result<(), io::Error> {
    // enable_raw_mode()?;
    let mut terminal = Terminal::default().unwrap();
    terminal.welcome()?;
    // terminal.clear_screen()?;
    // terminal.draw_row()?;
    // (&mut stdout)?;
    // 用户key行为,terminal.xxx()
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
            KeyCode::Char(c) => terminal.print(c)?,
            _ => {}
        }
    }

    terminal.quit()?;
    Ok(())
}

#[derive(Debug)]
pub struct Position {
    column: u16,
    row: u16,
}

impl Position {
    pub fn init(column: u16, row: u16) -> Self {
        Position { column, row }
    }
    pub fn move_position(
        self: &mut Self,
        terminal: &mut Terminal,
        move_step: &Self,
    ) -> Result<Self, io::Error> {
        let p = Position {
            column: self.column + move_step.column,
            row: self.row + move_step.row,
        };
        terminal._stdout.queue(MoveTo(p.column, p.row))?.flush()?;
        Ok(p)
    }
}
