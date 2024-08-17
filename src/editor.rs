use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;

use crate::terminal::Position;
use crate::terminal::Terminal;
use crate::view::View;
use std::env;
use std::io;
use std::panic::set_hook;
use std::panic::take_hook;
use std::path::PathBuf;

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
#[derive(Debug, Clone, Copy, Default)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(PartialEq)]
enum LoopAction {
    Interrupt,
    KeepOn,
    SKIP,
}

#[derive(Default)]
pub struct Editor {
    view: View,
    is_clear: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) -> Result<(), io::Error> {
        let hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            hook(panic_info);
        }));

        Terminal::initialize().unwrap();
        // Terminal::enter_alternate_screen()?;

        self.handle_args().unwrap();
        // self.view.render()?;

        self.process()?;
        // Terminal::leave_alternate_screen()?;
        Terminal::terminate().unwrap();
        Ok(())
    }

    fn handle_args(&mut self) -> Result<(), io::Error> {
        let args: Vec<String> = env::args().collect();
        // println!("print args:{:?}", args);
        let mut path = None;
        if args.len() > 1 {
            path = Some(PathBuf::from(args[1].clone()));
        }
        self.view.load(path)?;
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), io::Error> {
        if !self.view.redraw {
            Terminal::execute()?;
            return Ok(());
        }
        Terminal::move_caret_to(Position::default())?;

        self.view.render()?;
        print!("{:?}", self.location);
        Terminal::move_caret_to(Position {
            col: self.location.x as u16,
            row: self.location.y as u16,
        })?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn process(&mut self) -> Result<(), io::Error> {
        loop {
            self.refresh_screen()?;
            let _: () = match read()? {
                Event::Key(event) => {
                    let loop_action = Self::process_keyevents(event)?;
                    if LoopAction::Interrupt == loop_action {
                        break;
                    }
                    if LoopAction::KeepOn == loop_action {
                        continue;
                    }
                }
                Event::Resize(columns, rows) => {
                    self.view.resize(Position {
                        col: columns,
                        row: rows,
                    })?;

                    // todo!()
                }
                Event::FocusGained => todo!(),
                Event::FocusLost => todo!(),
                Event::Mouse(_) => todo!(),
                Event::Paste(_) => todo!(),
            };
        }
        Ok(())
    }

    fn process_keyevents(event: KeyEvent) -> Result<LoopAction, io::Error> {
        let loop_action = Self::process_shortcuts(event)?;
        Self::process_keypress(event)?;
        Ok(loop_action)
    }

    fn process_shortcuts(event: KeyEvent) -> Result<LoopAction, io::Error> {
        match event {
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
            } => Terminal::clear_screen().map(|_| LoopAction::SKIP),
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::SHIFT,
                ..
            } => Terminal::clear_line().map(|_| LoopAction::SKIP),
            _ => Ok(LoopAction::SKIP),
        }
    }

    fn process_keypress(event: KeyEvent) -> Result<(), io::Error> {
        match event.code {
            KeyCode::Enter => Terminal::draw_row()?,
            KeyCode::Backspace => {
                let (column, row) = Terminal::move_direction_to(Direction::Left, 1)?;
                if column == 0 && row >= 1 {
                    // 向上一行
                    Terminal::move_direction_to(Direction::Up, 1)?;
                    // 移动到上一行的末尾位置？
                }
            }
            KeyCode::Tab => {
                Terminal::move_direction_to(Direction::Right, 4)?;
            }
            KeyCode::Char(c) => Terminal::print(c)?,
            _ => {}
        }
        Terminal::execute()?;
        Ok(())
    }
}
