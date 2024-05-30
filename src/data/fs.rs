use super::Bookmark;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

pub fn save(bookmarks: &[Bookmark], filename: Option<&str>) -> Result<(), String> {
    let filename = filename.unwrap_or("bookmarks.qm");
    let mut file = std::fs::File::create(filename).map_err(|e| e.to_string())?;
    for b in bookmarks {
        writeln!(file, "{}", b.serialize()).map_err(|e| e.to_string())?;
    }
    println!("saved {} bookmarks", bookmarks.len());
    Ok(())
}

pub fn load(filename: Option<&str>) -> Result<Vec<Bookmark>, String> {
    let filename = filename.unwrap_or("bookmarks.qm");
    let file = std::fs::File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut bookmarks = Vec::new();
    for line in reader.lines() {
        if let Some(b) = Bookmark::deserialize(&line.map_err(|e| e.to_string())?) {
            bookmarks.push(b);
        }
    }
    println!("loaded {} bookmarks", bookmarks.len());
    Ok(bookmarks)
}
