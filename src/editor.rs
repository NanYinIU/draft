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
use crossterm::style::Print;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::ExecutableCommand;
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
                draw_row(&mut stdout, format!("{}", c))?;
            }
            o => {
                draw_row(&mut stdout, format!("print other event key is {:?}", o))?;
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn draw_row(out: &mut io::Stdout, message: String) -> Result<(), io::Error> {
    print!("{message}");
    move_cursor(out, Move2::default())
}

fn move_cursor(out: &mut io::Stdout, move2: Move2) -> Result<(), io::Error> {
    let (column, row) = calc_new_position(cursor::position()?, move2);
    // println!("after calc column and row is {:?}", (column, row));
    execute!(out, cursor::MoveTo(column, row), EnableBlinking)?;
    Ok(())
}
pub fn calc_new_position(position: (u16, u16), move2: Move2) -> (u16, u16) {
    let mut column = position.0;
    let mut row = position.1;
    // println!("before calc column and row is {:?}", (column, row));
    return match move2.direction {
        Direction::Up => {
            row -= move2.offset;
            (column, row)
        }
        Direction::Down => {
            row += move2.offset;
            (column, row)
        }
        Direction::Left => {
            column -= move2.offset;
            (column, row)
        }
        Direction::Right => {
            column += move2.offset;
            (column, row)
        }
    };
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Offset = u16;

pub struct Move2 {
    direction: Direction,
    offset: Offset,
}

impl Move2 {
    // 移动右1
    fn new(direction: Direction, offset: Offset) -> Self {
        Move2 { direction, offset }
    }

    pub fn backspace() -> Self {
        Self::new(Direction::Left, 1)
    }
}

impl Default for Move2 {
    fn default() -> Self {
        Self::new(Direction::Right, 1)
    }
}
