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
    /// Creates a new File
    pub fn new() -> Self {
        File { path: None, content: None }
    }
    /// Same as new but with a path setted from the beginning
    pub fn from_path(path: String) -> Self {
        File { path: Some(path), content: None }
    }
    /// Gets the content from the specified file and introduces it as a String in the content attribute
    /// Returns a result in case the file name doesn't exist or the path attribute is empty
    pub fn get_content(&mut self) -> Result<()> {
        match self.path.clone() {
            Some(path) => {match  fs::read_to_string(path.as_str()) {
                    Ok(content) => {
                        self.content = Some(content);
                        Ok(())
                    },
                    Err(_) => Err(clap::Error::new(ErrorKind::InvalidValue))
                }
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))

        }
        
    }
    /// Prints the content as is
    pub fn print_content(&self) -> Result<String> {
        match self.content.as_deref() {
            Some(cont) => {
                Ok(cont.to_string())
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
    /// Prints the content reversing both lines and characters
    pub fn print_reverse(&self) -> Result<String> {
        match self.content.as_deref() {
            Some(cont) => { 
                Ok(cont.chars().rev().collect::<String>())
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
    /// Prints the content reversing only the lines
    pub fn print_lines_reverse(&self) -> Result<String> {
        match self.content.as_deref() {
            Some(cont) => {
                Ok(cont.split('\n').collect::<Vec<&str>>().iter().rev().map(|line| {
                    *line
                }).collect::<Vec<&str>>().join("\n"))
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
    /// Prints the content reversing only the characters within the lines
    pub fn print_chars_reverse(&self) -> Result<String> {
        match self.content.as_deref() {
            Some(cont) => {
                Ok(cont.split('\n').collect::<Vec<&str>>().iter().map(|line| {
                    line.chars().rev().collect::<String>()
                }).collect::<Vec<String>>().join("\n"))
            },
            None => Err(clap::Error::new(ErrorKind::InvalidValue))
        }
    }
}
pub fn get_file_and_print(args: (String, bool, bool, bool)) -> Result<String> {
    let mut file = File::from_path(args.0);
    file.get_content()?;
    let mut content_to_print = String::new();
    match (args.1, args.2, args.3) {
        (false, false, false) => content_to_print = file.print_content()?,
        (true, false, false) => content_to_print = file.print_reverse()?,
        (false, true, false) => content_to_print = file.print_lines_reverse()?,
        (false, false, true) => content_to_print = file.print_chars_reverse()?,
        _ => Err(clap::Error::new(ErrorKind::InvalidSubcommand))?,
    }
    Ok(content_to_print)
}

#[cfg(test)]
mod tests {
    use crate::File;
    #[test]
    #[should_panic]
    fn print_content_test_empty_content() {
        let file = File::new();
        file.print_content().unwrap();
    }
    #[test]
    fn print_content_with_content() {
        let file = File {
            path: Some(String::new()),
            content: Some(String::from("Hello there!"))
        };
        assert_eq!(file.print_content().unwrap(), String::from("Hello there!"))
    }
    #[test]
    fn print_content_with_content_intros() {
        let file = File {
            path: Some(String::new()),
            content: Some(String::from("Hello there!\n!ereht olleH"))
        };
        assert_eq!(file.print_content().unwrap(), String::from("Hello there!\n!ereht olleH"))
    }
    #[test]
    #[should_panic]
    fn print_reverse_empty() {
        let file = File::new();
        file.print_reverse().unwrap();
    }
    #[test]
    #[should_panic]
    fn print_reverse_panic() {
        let file = File {
            path: Some(String::new()),
            content: Some(String::from("Hello there!\nGeneral Kenobi"))
        };
        assert_eq!(file.print_reverse().unwrap(), String::from("Hello there!\nGeneral Kenobi"));
    }
    #[test]
    fn print_reverse() {
        let file = File {
            path: Some(String::new()),
            content: Some(String::from("Hello there!\nGeneral Kenobi"))
        };
        assert_eq!(file.print_reverse().unwrap(), String::from("iboneK lareneG\n!ereht olleH"));
    }
    #[test]
    #[should_panic]
    fn print_lines_reverse_empty() {
        let file = File::new();
        file.print_lines_reverse().unwrap();
    }
    #[test]
    #[should_panic]
    fn print_lines_reverse_panic() {
        let file = File {
            path: Some(String::new()),
            content: Some(String::from("Hello there!\nGeneral Kenobi"))
        };
        assert_eq!(file.print_lines_reverse().unwrap(), String::from("Hello there!\nGeneral Kenobi"));
    }
    #[test]
    fn print_lines_reverse() {
        let file = File {
            path: Some(String::new()),
            content: Some(String::from("Hello there!\nGeneral Kenobi"))
        };
        assert_eq!(file.print_lines_reverse().unwrap(), String::from("General Kenobi\nHello there!"));
    }
    #[test]
    #[should_panic]
    fn print_characters_reverse_empty() {
        let file = File::new();
        file.print_chars_reverse().unwrap();
    }
    #[test]
    #[should_panic]
    fn print_characters_reverse_panic() {
        let file = File {
            path: Some(String::new()),
            content: Some(String::from("Hello there!\nGeneral Kenobi"))
        };
        assert_eq!(file.print_chars_reverse().unwrap(), String::from("Hello there!\nGeneral Kenobi"));
    }
    #[test]
    fn print_chars_reverse() {
        let file = File {
            path: Some(String::new()),
            content: Some(String::from("Hello there!\nGeneral Kenobi"))
        };
        assert_eq!(file.print_chars_reverse().unwrap(), String::from("!ereht olleH\niboneK lareneG"));
    }
}