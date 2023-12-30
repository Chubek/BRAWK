use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::result::{Result, Result::Err};

use regex::Regex;

struct AwkIO {
    inputs: HashMap<String, Option<Box<dyn BufRead>>>,
    outputs: HashMap<String, Box<dyn Write>>,
    tokenized: Vec<String>,
}

impl AwkIO {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn add_input(&mut self, file_path: &str) -> Result<(), Err> {
        if file_path == "-" {
            self.inputs.insert("STDIN".to_string(), None);
            Ok(())
        } else {
            let handle = File::open(file_path).map_err(|_| Err)?;
            let buffer = BufReader::new(handle);
            self.inputs
                .insert(file_path.to_string(), Some(Box::new(buffer)));
            Ok(())
        }
    }

    pub fn add_output(&mut self, file_path: &str) -> Result<(), Err> {
        if file_path == "-" {
            self.outputs
                .insert("STDOUT".to_string(), Box::new(io::stdout()));
            Ok(())
        } else {
            let handle = File::create(file_path).map_err(|_| Err)?;
            let buffer = BufWriter::new(handle);
            self.outputs.insert(file_path.to_string(), Box::new(buffer));
            Ok(())
        }
    }

    pub fn read_line_from_input(&mut self, file_path: &str) -> Result<Option<String>, Err> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            if let Some(input) = input {
                let mut line = String::new();
                input.read_line(&mut line).map_err(|_| Err)?;
                Ok(Some(line))
            } else {
                let mut line = String::new();
                io::stdin().read_line(&mut line).map_err(|_| Err)?;
                Ok(Some(line))
            }
        } else {
            Ok(None)
        }
    }

    pub fn read_from_input(&mut self, file_path: &str, buffer: &mut Vec<u8>) -> Result<usize, Err> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            if let Some(input) = input {
                input.read(buffer).map_err(|_| Err)
            } else {
                io::stdin().read(buffer).map_err(|_| Err)
            }
        } else {
            Ok(0)
        }
    }

    pub fn write_to_output(&mut self, file_path: &str, data: &[u8]) -> Result<(), Err> {
        if let Some(output) = self.outputs.get_mut(file_path) {
            output.write_all(data).map_err(|_| Err)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn read_until_regex(
        &mut self,
        file_path: &str,
        pattern: Regex,
        buffer: &mut Vec<u8>,
    ) -> Result<usize, Err> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            if let Some(input) = input {
                let mut read_buffer = Vec::new();
                let mut delimiter_found = false;

                while !delimiter_found {
                    buffer.clear();
                    let bytes_read = input.read_to_end(buffer).map_err(|_| Err)?;

                    if bytes_read == 0 {
                        break; // Reached the end of the file
                    }

                    read_buffer.extend_from_slice(buffer);

                    // Convert the read data to a string for regex matching
                    let content_str = String::from_utf8_lossy(&read_buffer);

                    // Check if the delimiter is found
                    if pattern.is_match(&content_str) {
                        delimiter_found = true;
                    }
                }

                self.tokenized = pattern.split(&buffer).collect();
                Ok(read_buffer.len())
            } else {
                // STDIN
                let mut delimiter_found = false;

                while !delimiter_found {
                    buffer.clear();
                    let bytes_read = io::stdin().read_to_end(buffer).map_err(|_| Err)?;

                    if bytes_read == 0 {
                        break; // Reached the end of input
                    }

                    // Convert the read data to a string for regex matching
                    let content_str = String::from_utf8_lossy(buffer);

                    // Check if the delimiter is found
                    if pattern.is_match(&content_str) {
                        delimiter_found = true;
                    }
                }

                self.tokenized = pattern.split(&buffer).collect();
                Ok(buffer.len())
            }
        } else {
            Ok(0)
        }
    }
}
