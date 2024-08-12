use std::{env, path::PathBuf};

mod editor;
mod terminal;
fn main() {
    // println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    // println!("print args:{:?}", args);
    let mut path = None;
    if args.len() > 1 {
        path = Some(PathBuf::from(args[1].clone()));
    }

    match editor::run(path) {
        Ok(()) => (),
        Err(e) => panic!("execute error,cause{e}"),
    };
}

#[cfg(test)]
pub mod test {
    #[test]
    fn test_life() {
        let x = String::from("x");
        let y = String::from("y");
        // longestValue(x, y);
        longest(&x, &y);
        longest(&x, &y);
    }

    /*
    rustc: missing lifetime specifier
    this function's return type contains a borrowed value,
    but the signature does not say whether it is borrowed from `x` or `y`

    需要声明生命周期
    带有生命周期的函数格式：fn f<'a>(t: &'a T)
    */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
