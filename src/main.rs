use std::{env, path::PathBuf};

mod editor;
mod terminal;
fn main() {
    // println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // println!("print args:{:?}", args);
    let mut path = PathBuf::new();
    if args.len() > 0 {
        path = PathBuf::from(args[1].clone());
    }

    match editor::run(&path) {
        Ok(()) => (),
        Err(e) => panic!("execute error,cause{e}"),
    };
}
