use serde::{Deserialize, Serialize};
use serde_json::{Error, Result};
use std::fs::File;

enum NoteType {
    Light,
    Middle,
    Large,
}

#[derive(Serialize, Deserialize)]
struct Note {
    contents: String,
    note_type: NoteType,
}

pub fn partition(s: String, ind: usize) -> Vec<String> {
    let mut rst = Vec::new();
    let mut tmp = String::from("");
    for (c, i) in s.chars().enumerate() {
        tmp += &c.to_string();
        if (i as usize) > 0 && (i as usize) % ind == 0 {
            rst.push(tmp);
            tmp = String::from("");
        }
    }
    if tmp.len() > 0 {
        rst.push(tmp);
    }
    rst
}

impl Note {
    // 自動で改行をいれる。
    fn insert_newline(&self, width: usize) -> String {
        match self.note_type {
            Light => self.contents,
            NoteType::Large | NoteType::Middle => partition(self.contents, width).join("\n"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Entity {
    name: String,
    description: Note,
}

#[derive(Serialize, Deserialize)]
struct EntityRelation {
    from: Entity,
    to: Entity,
}

#[derive(Serialize, Deserialize)]
struct EntityDB {
    entity_list: [Entity],
    entity_relationship_list: [EntityRelation],
}

fn main() {
    let cmd = std::env::args().nth(1).expect("no path given");
    println!("{}", cmd);
}

fn entity_list() -> Result<()> {
    let file = File::open("foo.txt");
    serde_json::from_reader(file.expect("file cannot read."))
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn entity_add() {}
    #[test]
    fn entity_del() {}
    #[test]
    fn entity_list() {}
    #[test]
    fn entity_relationship() {}
}
