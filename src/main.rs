pub mod live;

use rustyline::DefaultEditor;
use live::Env;

fn main() {
    let mut env = Env::new();
    env.run(&mut DefaultEditor::new().expect("editor"));
}
