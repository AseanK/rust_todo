use std::{
    fs::{self, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Write},
};

pub struct Task {
    pub title: Vec<String>,
    pub path: String,
}

pub const FILE_PATH: &str = "/Users/krean/Projects/Todos/todo.txt";

impl Task {
    pub fn new() -> Result<Self, String> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&FILE_PATH)
            .expect("Couldn't open the todofile");

        let mut buf_reader = BufReader::new(&file);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let todo: Vec<String> = contents.lines().map(str::to_string).collect();

        Ok(Self {
            title: todo,
            path: FILE_PATH.to_string(),
        })
    }

    pub fn add_task(&self, inp: &[String]) {
        let file = OpenOptions::new()
            .create(true) // a) create the file if it does not exist
            .append(true) // b) append a line to it
            .open(&self.path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(file);
        let line = format!("[ ] {}\n", inp.join(" "));

        buffer
            .write_all(line.as_bytes())
            .expect("Failed to write data");
    }

    pub fn list_task(&self) {
        let stdout = io::stdout();

        let mut writer = BufWriter::new(stdout);

        for (i, task) in self.title.iter().enumerate() {
            let data = format!("{i} {task}\n");
            writer
                .write_all(data.as_bytes())
                .expect("Failed to read data");
        }
    }

    pub fn mark_task(&self, index: usize) {
        let file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(file);

        for task in self.title.iter() {
            if task.contains(&self.title[index]) {
                if &task[..3] == "[ ]" {
                    let data = format!("[X] {}\n", &task[4..]);
                    buffer
                        .write_all(data.as_bytes())
                        .expect("Failed to write data");
                } else if &task[..3] == "[X]" {
                    let data = format!("[ ] {}\n", &task[4..]);
                    buffer
                        .write_all(data.as_bytes())
                        .expect("Failed to write data");
                }
            } else {
                let data = format!("{}\n", &task);
                buffer
                    .write_all(data.as_bytes())
                    .expect("Failed to write data");
            }
        }
    }

    pub fn edit_task(&self, index: usize, inp: &[String]) {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .expect("Failed to open file");

        let mut buffer = BufWriter::new(file);

        for task in self.title.iter() {
            if task.contains(&self.title[index]) {
                let data = format!("[ ] {}\n", inp.join(" "));
                buffer.write_all(data.as_bytes()).expect("Failed to update");
            } else {
                let data = format!("{}\n", &task);
                buffer
                    .write_all(data.as_bytes())
                    .expect("Failed to write data");
            }
        }
    }

    pub fn delete_task(&self, index: usize) {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(file);

        for task in self.title.iter() {
            if !task.contains(&self.title[index]) {
                let data = format!("{}\n", &task);
                buffer
                    .write_all(data.as_bytes())
                    .expect("Failed to write data");
            }
        }
    }

    pub fn reset_task(&self) {
        match fs::remove_file(&self.path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while clearing todo file: {}", e)
            }
        }
    }
}
