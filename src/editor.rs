use std::io;
use std::io::stdout;

use crossterm::cursor;
use crossterm::cursor::EnableBlinking;
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
pub fn run() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    while let Event::Key(event) = read()? {
        if KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL) == event {
            println!("bye bye!!");
            break;
        }
        if KeyEvent::new(KeyCode::Char('l'), KeyModifiers::SHIFT) == event {
            execute!(stdout, Clear(ClearType::All))?;
            continue;
        }
        match event.code {
            KeyCode::Char(c) => {
                draw_row(&mut stdout, &format!("{c}"))?;
            }
            o => {
                draw_row(&mut stdout, &format!("print other event key is {o:?}"))?;
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn draw_row(out: &mut io::Stdout, message: &String) -> Result<(), io::Error> {
    print!("{message}");
    move_cursor(out, &Direction::Right(1))
}

fn move_cursor(out: &mut io::Stdout, direction: &Direction) -> Result<(), io::Error> {
    let (column, row) = direction.calc_new_position(cursor::position()?);
    // println!("after calc column and row is {:?}", (column, row));
    execute!(out, cursor::MoveTo(column, row), EnableBlinking)?;
    Ok(())
}

enum Direction {
    Up(Offset),
    Down(Offset),
    Left(Offset),
    Right(Offset),
}

type Offset = u16;

impl Direction {
    pub fn calc_new_position(self: &Self, position: (u16, u16)) -> (u16, u16) {
        let mut column = position.0;
        let mut row = position.1;
        // println!("before calc column and row is {:?}", (column, row));
        match self {
            Direction::Up(offset) => {
                row -= offset;
                (column, row)
            }
            Direction::Down(offset) => {
                row += offset;
                (column, row)
            }
            Direction::Left(offset) => {
                column -= offset;
                (column, row)
            }
            Direction::Right(offset) => {
                column += offset;
                (column, row)
            }
        }
    }
}
