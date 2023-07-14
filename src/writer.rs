use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct Writer {
    filename: Option<String>,
    file: Option<std::fs::File>,
}

impl Writer {
    pub fn new(filename: Option<String>) -> Self {
        let file = match &filename {
            Some(name) => {
                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(name);

                match file {
                    Ok(f) => Some(f),
                    Err(_) => {
                        eprintln!("Error opening file: {}", name);
                        None
                    }
                }
            }
            None => None,
        };

        Writer {
            filename,
            file,
        }
    }

    pub fn println(&mut self, text: &str) {
        if let Some(file) = &mut self.file {
            if let Err(_) = writeln!(file, "{}", text) {
                eprintln!("Error writing to file");
            }
        } else {
            println!("{}", text);
        }
    }
}
