use std::fs;
use clap::error::ErrorKind;
pub type Result<T> = std::result::Result<T, clap::Error>;


pub struct File {
    path: Option<String>,
    content: Option<String>
}
impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
impl File {
    pub fn new() -> Self {
        File { path: None, content: None }
    }
    pub fn from_path(path: String) -> Self {
        File { path: Some(path), content: None }
    }
    pub fn get_content(&mut self) -> Result<()> {
        match  fs::read_to_string(self.path.clone().unwrap().as_str()) {
            Ok(content) => {
                self.content = Some(content);
                Ok(())
            },
            Err(_) => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
    pub fn print_content(&self) -> Result<()> {
        match self.content.as_deref() {
            Some(cont) => {
                println!("{}", cont);
                Ok(())
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
    pub fn print_reverse(&self) -> Result<()> {
        match self.content.as_deref() {
            Some(cont) => {
                println!("{}", cont.chars().rev().collect::<String>());
                Ok(())
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
    pub fn print_lines_reverse(&self) -> Result<()> {
        match self.content.as_deref() {
            Some(cont) => {
                cont.split('\n').collect::<Vec<&str>>().iter().rev().for_each(|line| {
                    println!("{}", line);
                });
                Ok(())
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
    pub fn print_chars_reverse(&self) -> Result<()> {
        match self.content.as_deref() {
            Some(cont) => {
                dbg!("here");
                cont.split('\n').collect::<Vec<&str>>().iter().for_each(|line| {
                    println!("{}", line.chars().rev().collect::<String>());
                });
                Ok(())
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
}
pub fn get_file_and_print(args: (String, bool, bool, bool)) -> Result<()> {
    let mut file = File::from_path(args.0);
    file.get_content()?;
    match (args.1, args.2, args.3) {
        (false, false, false) => file.print_content()?,
        (true, false, false) => file.print_reverse()?,
        (false, true, false) => file.print_lines_reverse()?,
        (false, false, true) => file.print_chars_reverse()?,
        _ => Err(clap::Error::new(ErrorKind::InvalidSubcommand))?,
    }
    Ok(())
}