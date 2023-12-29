use std::io::{self, BufRead, Write};
use std::fs::File;

struct AwkIO {
    input: Option<Box<dyn BufRead>>,
    output: Box<dyn Write>,
}

impl AwkIO {
    fn standard_io() -> Self {
        Self {
            input: None,
            output: Box::new(io::stdout()),
        }
    }

    fn from_file(file_path: &str) -> io::Result<Self> {
        let file = File::open(file_path)?;
        Ok(Self {
            input: Some(Box::new(io::BufReader::new(file))),
            output: Box::new(io::stdout()),
        })
    }

    fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();
        let Self { input, .. } = self;

        if input.is_none() {
            io::stdin()
                .read_line(&mut line)?;
        } else {
            self.input
                .as_mut()
                .expect("Error getting input lock")
                .read_line(&mut line)?;
        }

        Ok(line)
    }

    fn write(&mut self, s: &str) -> io::Result<()> {
        self.output.write_all(s.as_bytes())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}


