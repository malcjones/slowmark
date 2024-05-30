pub mod builtin;

use std::path::Path;

use rustyline::{error::ReadlineError, DefaultEditor};
use slowmark::data::{fs::load, Bookmark};

pub struct Env {
    pub error: Result<(), String>,
    pub builtin: Vec<Command>,
    pub bookmarks: Vec<Bookmark>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            error: Ok(()),
            builtin: Vec::new(),
            bookmarks: Vec::new(),
        }
    }

    pub fn with_builtin(mut self) -> Self {
        self.builtin = builtin::cmds();
        self
    }

    pub fn motd(&self) {
        println!(
            r#"     s
    s s
    s    
     s   lowmark {}
      s   - bookmarks: {}
    s s   - cmds: {}
     s"#,
            env!("CARGO_PKG_VERSION"),
            self.bookmarks.len(),
            self.builtin.len()
        );
        println!("type 'help' for more information");
    }

    pub fn prompt(&self) -> String {
        if self.error.is_ok() {
            ">> ".to_string()
        } else {
            "!> ".to_string()
        }
    }

    pub fn find_command(&self, name: &str) -> Option<&Command> {
        self.builtin.iter().find(|cmd| cmd.name == name)
    }

    pub fn handle(&mut self, line: String) -> Result<(), String> {
        let name = Context::parse(&line).ok_or(format!("empty command"))?.0;
        let cmd = self
            .find_command(name)
            .ok_or(format!("unknown command: {name}"))?;
        (cmd.action)(&mut Context::new(self, line))
    }

    pub fn startup(&mut self) {
        self.autoload();
        self.motd();
    }

    pub fn autoload(&mut self) {
        if Path::new("bookmarks.qm").exists() {
            self.bookmarks = load(Some("bookmarks.qm")).expect("couldn't autoload bookmarks.qm");
        }
    }

    pub fn run(&mut self, editor: &mut DefaultEditor) {
        self.startup();

        loop {
            if let Err(ref e) = self.error {
                eprintln!("error: {}", e);
            }
            let result = self.handle(self.take(editor));
            self.error = result;
        }
    }

    pub fn take(&self, editor: &mut DefaultEditor) -> String {
        match editor.readline(&self.prompt()) {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {
                eprintln!("CTRL-C");
                std::process::exit(0);
            }
            Err(ReadlineError::Eof) => {
                eprintln!("CTRL-D");
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                std::process::exit(1);
            }
        }
    }
}

pub struct Context<'a> {
    pub env: &'a mut Env,
    pub line: String,
}

impl<'a> Context<'a> {
    pub fn new(env: &'a mut Env, line: String) -> Self {
        Context { env, line }
    }

    pub fn args(&self) -> Vec<&str> {
        Self::parse(&self.line).map_or(Vec::new(), |(_, args)| args)
    }

    pub fn parse(line: &str) -> Option<(&str, Vec<&str>)> {
        let mut split = line.trim_end().split_whitespace();
        Some((split.next()?, split.collect()))
    }
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub action: fn(&mut Context) -> Result<(), String>,
}

pub struct CmdBuilder(Command);

impl CmdBuilder {
    pub fn new(name: &str) -> Self {
        CmdBuilder(Command {
            name: name.to_string(),
            description: "".to_string(),
            usage: "".to_string(),
            action: |_: &mut Context| Ok(()),
        })
    }

    pub fn description(mut self, description: &str) -> Self {
        self.0.description = description.to_string();
        self
    }

    pub fn usage(mut self, usage: &str) -> Self {
        self.0.usage = usage.to_string();
        self
    }

    pub fn action(mut self, action: fn(&mut Context) -> Result<(), String>) -> Self {
        self.0.action = action;
        self
    }

    pub fn build(self) -> Command {
        self.0
    }
}
