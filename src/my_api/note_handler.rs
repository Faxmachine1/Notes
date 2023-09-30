use ring::digest;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::{BufReader, BufRead}, io::Write};

fn generate_sha256(data: &String) -> String {
    let mut context = digest::Context::new(&digest::SHA256);
    context.update(data.as_bytes());

    hex::encode(context.finish())
}

#[derive(Serialize, Deserialize)]
pub struct Note {
    title: String,
    body: String,
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub title: String,
    pub body: String,
}

impl Note {

    // Создание новой заметки
    pub fn create(title: String, body: String) -> Note {
        let combined = format!("{},{}", title, body);
        let key = generate_sha256(&combined);

        let object = Note {title, body, key};

        let mut file = OpenOptions::new().append(true).open("Notes.txt").unwrap();

        let json_to_string = format!("\n{}", serde_json::to_string(&object).unwrap());
        file.write_all(json_to_string.as_bytes()).unwrap();

        object
    }
    
    // Прочитать заметку по ключу
    pub fn read(key: String) -> Option<Data> {
        let file = OpenOptions::new().read(true).open("Notes.txt").unwrap();

        let reader = BufReader::new(&file);
        for line in reader.lines() {
            let str = line.unwrap();
            
            let note: Self = match serde_json::from_str(&str) {
                Ok(note) => note,
                Err(_) => continue,
            };

            if note.check_key(&key) {
                return Some(Data { 
                    title: note.title, 
                    body: note.body 
                });
            }
        }

        None
    }

    fn check_key(&self, key: &str) -> bool {
        key.eq_ignore_ascii_case(&self.key)
    }
}
