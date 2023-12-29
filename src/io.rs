use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use std::collections::HashMap;
use std::result::{Result, Result::Err};

struct AwkIO {
    inputs: HashMap<String, Option<Box<dyn BufRead>>>,
    outputs: HashMap<String, Box<dyn Write>>,
}

impl AwkIO {
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn add_input(&mut self, file_path: &String) -> Result<(), Err> {
        if file_path == "-" {
            self.inputs.insert("STDIN".to_string(), None);
            Ok(())
        } else {
            let handle = File::open(file_path.clone())?;
            let buffer = BufReader::new(handle);
            self.inputs.insert(file_path.clone(), Some(Box::new(buffer)));
            Ok(())
        }
    }

    pub fn add_output(&mut self, file_path: &String) -> Result<(), Err> {
        if file_path == "-" {
            self.outputs.insert("STDOUT".to_string(), Box::new(io::stdout()));
            Ok(())
        } else {
            let handle = File::open(file_path.clone())?;
            let buffer = BufWriter::new(handle);
            self.outputs.insert(file_path.clone(), Box::new(buffer));
        }
    }

}


