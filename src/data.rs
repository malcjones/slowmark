use colored::Colorize;

pub struct Bookmark {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}

impl Bookmark {
    pub fn deserialize(line: &str) -> Option<Bookmark> {
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let mut split = line.splitn(3, '|').map(|s| s.to_owned());
        Some(Bookmark {
            name: split.next()?,
            url: split.next()?,
            tags: split
                .next()?
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        })
    }

    pub fn tag_str(&self) -> String {
        self.tags.join(",")
    }

    pub fn pretty(&self) -> String {
        format!(
            "{} ({}) {}",
            self.name.bold(),
            self.url.blue(),
            self.tags
                .iter()
                .map(|s| s.bold().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    // hehe
    pub fn pp(&self) {
        println!("{}", self.pretty());
    }

    pub fn serialize(&self) -> String {
        format!("{}|{}|{}", self.name, self.url, self.tag_str())
    }
}
