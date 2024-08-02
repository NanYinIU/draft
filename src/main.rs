mod editor;
fn main() {
    println!("Hello, world!");
    match editor::run() {
        Ok(()) => (),
        Err(e) => panic!("execute error,cause{e}"),
    };
}
