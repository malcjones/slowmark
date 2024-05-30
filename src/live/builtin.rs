use super::{CmdBuilder, Command};

pub fn cmds() -> Vec<Command> {
    vec![echo(), help()]
        .into_iter()
        .chain(bookmark::cmds().into_iter())
        .collect()
}

pub fn echo() -> Command {
    CmdBuilder::new("echo")
        .description("echo arguments")
        .usage("echo [args...]")
        .action(|ctx| {
            println!("{}", ctx.args().join(" "));
            Ok(())
        })
        .build()
}

pub fn help() -> Command {
    CmdBuilder::new("help")
        .description("show help")
        .usage("help [command]")
        .action(|ctx| {
            if let Some(cmd) = ctx.args().get(0) {
                if let Some(cmd) = ctx.env.find_command(cmd) {
                    println!("{} - {}", cmd.name, cmd.description);
                    println!("usage: {}", cmd.usage);
                } else {
                    println!("unknown command: {}", cmd);
                }
            } else {
                for cmd in &ctx.env.builtin {
                    println!("{} - {}", cmd.name, cmd.description);
                }
            }
            Ok(())
        })
        .build()
}

mod bookmark {
    use super::{CmdBuilder, Command};
    use slowmark::data::Bookmark;

    pub fn cmds() -> Vec<Command> {
        vec![add(), list(), search(), remove(), load(), save(), reset()]
    }

    fn add() -> Command {
        CmdBuilder::new("add")
            .description("add a bookmark")
            .usage("add <name> <url> [tags...]")
            .action(|ctx| {
                let mut args = ctx.args().into_iter();
                let name = args.next().ok_or("missing name")?.to_string();
                let url = args.next().ok_or("missing url")?.to_string();
                let tags = args.map(|s| s.to_string()).collect();
                ctx.env.bookmarks.push(Bookmark::new(name, url, tags));
                println!("added bookmark");
                Ok(())
            })
            .build()
    }

    fn list() -> Command {
        CmdBuilder::new("list")
            .description("list all bookmarks")
            .usage("list")
            .action(|ctx| {
                for (i, b) in ctx.env.bookmarks.iter().enumerate() {
                    println!("{}. {}", i, b.pretty());
                }
                Ok(())
            })
            .build()
    }

    fn search() -> Command {
        CmdBuilder::new("search")
            .description("search bookmarks")
            .usage("search <query>")
            .action(|ctx| {
                let query = ctx.args().join(" ");
                for b in ctx.env.bookmarks.iter() {
                    if b.name.contains(&query) || b.tag_str().contains(&query) {
                        println!("{}", b.pretty());
                    }
                }
                Ok(())
            })
            .build()
    }

    fn remove() -> Command {
        CmdBuilder::new("remove")
            .description("remove a bookmark")
            .usage("remove <index>")
            .action(|ctx| {
                let index = ctx
                    .args()
                    .get(0)
                    .ok_or("missing index")?
                    .parse::<usize>()
                    .map_err(|_| "invalid index")?;
                ctx.env.bookmarks.remove(index);
                println!("removed bookmark");
                Ok(())
            })
            .build()
    }

    fn reset() -> Command {
        CmdBuilder::new("reset")
            .description("reset all bookmarks")
            .usage("reset")
            .action(|ctx| {
                ctx.env.bookmarks.clear();
                println!("reset bookmarks");
                Ok(())
            })
            .build()
    }

    fn load() -> Command {
        CmdBuilder::new("load")
            .description("load bookmarks from file")
            .usage("load [filename]")
            .action(|ctx| {
                let filename = ctx.args().get(0).map(|s| s.to_string());
                ctx.env
                    .bookmarks
                    .extend(slowmark::data::fs::load(filename.as_deref())?);
                Ok(())
            })
            .build()
    }

    fn save() -> Command {
        CmdBuilder::new("save")
            .description("save bookmarks to file")
            .usage("save [filename]")
            .action(|ctx| {
                let filename = ctx.args().get(0).map(|s| s.to_string());
                slowmark::data::fs::save(&ctx.env.bookmarks, filename.as_deref())?;
                Ok(())
            })
            .build()
    }
}
