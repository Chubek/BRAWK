use io::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::fmt;
use std::cmp::PartialEq;
use std::clone::Clone;

pub struct AwkIO {
    inputs: HashMap<String, Option<Box<dyn BufRead>>>,
    outputs: HashMap<String, Box<dyn Write>>,
    fields: Vec<String>,
    line: String,
}

impl AwkIO {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            fields: vec![],
            line: String::new(),
        }
    }

    pub fn add_input(&mut self, file_path: &str) -> Result<()> {
        if file_path == "-" {
            self.inputs.insert("STDIN".to_string(), None);
            Ok(())
        } else {
            let handle = File::open(file_path)?;
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
            let handle = File::create(file_path)?;
            let buffer = BufWriter::new(handle);
            self.outputs.insert(file_path.to_string(), Box::new(buffer));
            Ok(())
        }
    }

    pub fn read_line_from_input(
        &mut self,
        file_path: &str,
        delimiter: char,
    ) -> Result<usize> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            let line_len: usize = match input {
                Some(input) => input.read_line(&mut self.line)?,
                None => io::stdin().read_line(&mut self.line)?,
            };

            if !self.line.is_empty() {
                self.fields = self.line
                    .trim()
                    .split(delimiter)
                    .map(|s| s.to_string())
                    .collect();
                Ok(line_len)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    pub fn read_from_input(&mut self, file_path: &str, buffer: &mut String) -> Result<usize> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            match input {
                Some(input) => input.read_line(buffer),
                None => io::stdin().read_line(buffer),
            }
        } else {
            Ok(0)
        }
    }

    pub fn write_to_output(&mut self, file_path: &str, data: &[u8]) -> Result<()> {
        if let Some(output) = self.outputs.get_mut(file_path) {
            output.write_all(data)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn read_until_regex(&mut self, file_path: &str, pattern: Regex) -> Result<usize> {
        if let Some(input) = self.inputs.get_mut(file_path) {
            self.line.clear();
            let mut read_buffer = String::new();
            let mut delimiter_found = false;
            let mut bytes_read = 0;

            while !delimiter_found {
                match input {
                    Some(input) => bytes_read = input.read_line(&mut read_buffer)?,
                    None => bytes_read = io::stdin().read_line(&mut read_buffer)?,
                }

                if bytes_read == 0 {
                    break;
                }

                self.line.push_str(&read_buffer);

                if pattern.is_match(&self.line) {
                    delimiter_found = true;
                }

                self.fields = pattern
                    .split(&read_buffer)
                    .map(|s| s.to_string())
                    .collect();
            }

            Ok(bytes_read)
        } else {
            Ok(0)
        }
    }

    pub fn get_field(&self, index: usize) -> String {
        if index > 0 && index <= self.fields.len() {
            self.fields[index - 1].clone()
        } else {
            String::new()
        }
    }
}

impl fmt::Display for AwkIO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Fields: {:?}", self.fields);
        writeln!(f, "Line: {}", self.line)
    }
}

impl PartialEq for AwkIO {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
    }
}

impl fmt::Debug for AwkIO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AwkIO")
            .field("fields", &self.fields)
            .field("line", &self.line)
            .finish()
    }
}

impl Clone for AwkIO {
    fn clone(&self) -> Self {
        let mut new_instance = AwkIO::new();
        
        new_instance.fields = self.fields.clone();
        new_instance.line = self.line.clone();
        
        new_instance
    }
}
