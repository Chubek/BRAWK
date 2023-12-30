use io::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

use regex::Regex;

struct AwkIO {
    inputs: HashMap<String, Option<Box<dyn BufRead>>>,
    outputs: HashMap<String, Box<dyn Write>>,
    last_read: Vec<String>,
}

impl AwkIO {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            last_read: vec![],
        }
    }

    pub fn add_input(&mut self, file_path: &str) -> Result<()> {
        if file_path == "-" {
            self.inputs.insert("STDIN".to_string(), None);
            Ok(())
        } else {
            let handle = File::open(file_path).unwrap();
            let buffer = BufReader::new(handle);
            self.inputs
                .insert(file_path.to_string(), Some(Box::new(buffer)));
            Ok(())
        }
    }

    pub fn add_output(&mut self, file_path: &str) -> Result<()> {
        if file_path == "-" {
            self.outputs
                .insert("STDOUT".to_string(), Box::new(io::stdout()));
            Ok(())
        } else {
            let handle = File::create(file_path).unwrap();
            let buffer = BufWriter::new(handle);
            self.outputs.insert(file_path.to_string(), Box::new(buffer));
            Ok(())
        }
    }

    pub fn read_line_from_input(&mut self, file_path: &str) -> Result<Option<String>> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            if let Some(input) = input {
                let mut line = String::new();
                input.read_line(&mut line).unwrap();
                Ok(Some(line))
            } else {
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                Ok(Some(line))
            }
        } else {
            Ok(None)
        }
    }

    pub fn read_from_input(&mut self, file_path: &str, buffer: &mut String) -> Result<usize> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            if let Some(input) = input {
                input.read_line(buffer)
            } else {
                io::stdin().read_line(buffer)
            }
        } else {
            Ok(0)
        }
    }

    pub fn write_to_output(&mut self, file_path: &str, data: &[u8]) -> Result<()> {
        if let Some(output) = self.outputs.get_mut(file_path) {
            output.write_all(data).unwrap();
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn read_until_regex(&mut self, file_path: &str, pattern: Regex) -> Result<usize> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            let mut read_buffer = String::new();
            let mut total_buffer = String::new();
            let mut delimiter_found = false;
            let mut bytes_read = 0;

            while !delimiter_found {
                if let Some(input) = input {
                    bytes_read = input.read_line(&mut read_buffer)?;
                } else {
                    bytes_read = io::stdin().read_line(&mut read_buffer)?
                }

                if bytes_read == 0 {
                    break;
                }

                total_buffer.push_str(&read_buffer);

                if pattern.is_match(&total_buffer) {
                    delimiter_found = true;
                }

                self.last_read = pattern
                    .split(&total_buffer)
                    .map(|s| s.to_string())
                    .collect();
            }

            Ok(bytes_read)
        } else {
            Ok(0)
        }
    }
}
