use std::error;
use std::fmt::{Display, Debug};
use std::fs;
pub type Result<T> = std::result::Result<T, self::Error>;



#[derive(Clone)]
pub struct Error {
    kind: ErrorKind,
    error: String
}
#[derive(Clone)]
pub enum ErrorKind {
    PathNotSpecified,
    FileNotFound,
    EmptyContent,
    MultipleFlags
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::PathNotSpecified => write!(f, "khat error.\nThe path attribute is empty!\n{}", self.error),
            ErrorKind::FileNotFound => write!(f, "khat error.\nDidn't found any file\n{}", self.error), 
            ErrorKind::EmptyContent => write!(f, "khat error.\nYou didn't get the content from the file, invoque the `get_content()` method\n{}", self.error),
            ErrorKind::MultipleFlags => write!(f, "khat error.\nCan't use more than one flag\n{}", self.error),
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::PathNotSpecified => write!(f, "The path attribute is empty!"),
            ErrorKind::FileNotFound => write!(f, "Didn't found any file"), 
            ErrorKind::EmptyContent => write!(f, "You didn't get the content from the file, invoque the `get_content()` method"),
            ErrorKind::MultipleFlags => write!(f, "Can't use more than one flag"),
        }
    }
}
impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}



#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct File {
    path: Option<String>,
    content: Option<String>
}
impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.clone();
        if let Some(path) = file.path {
            if let Some(content) = file.content {
                return write!(f, "\nThe file {} has the following content:\n\n {}", path, content);
            }
            return write!(f, "\nThe file has the {} path, but has no content", path);
        }
        write!(f, "\nThe file has no path")
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
    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
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
                    Err(_)=> Err(Error { kind: ErrorKind::FileNotFound, error: "Ensure you introduce a correct path. If you use Tab it can help you autocomplete the name of the file.".to_string() }),
                }
            },
            None => Err(Error { kind: ErrorKind::PathNotSpecified, error: "You must specify a path in order to get the content.\nUse the `set_path()` method or create with the `from_path()` method.".to_string()})

        }
        
    }
    /// Prints the content as is
    pub fn print_content(&self) -> Result<String> {
        match self.content.as_deref() {
            Some(cont) => {
                Ok(cont.to_string())
            },
            None => Err(Error { kind: ErrorKind::EmptyContent, error: "The file has no content. Execute the `get_content()` method first.".to_string()})
        }
    }
    /// Prints the content reversing both lines and characters
    pub fn print_reverse(&self) -> Result<String> {
        match self.content.as_deref() {
            Some(cont) => { 
                Ok(cont.chars().rev().collect::<String>())
            },
            None => Err(Error { kind: ErrorKind::EmptyContent, error: "The file has no content. Execute the `get_content()` method first.".to_string() })
        }
    }
    /// Prints the content reversing only the lines
    pub fn print_lines_reverse(&self) -> Result<String> {
        match self.content.as_deref() {
            Some(cont) => {
                Ok(cont.split('\n').collect::<Vec<&str>>().iter().rev().copied().collect::<Vec<&str>>().join("\n"))
            },
            None => Err(Error { kind: ErrorKind::EmptyContent, error: "The file has no content. Execute the `get_content()` method first.".to_string() })
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
            None => Err(Error { kind: ErrorKind::EmptyContent, error: "The file has no content. Execute the `get_content()` method first.".to_string() })
        }
    }
}
pub fn get_file_and_print(args: (String, bool, bool, bool)) -> Result<File> {
    let mut file = File::from_path(args.0);
    file.get_content()?;
    let mut content_to_print = String::new();
    match (args.1, args.2, args.3) {
        (false, false, false) => content_to_print = file.print_content()?,
        (true, false, false) => content_to_print = file.print_reverse()?,
        (false, true, false) => content_to_print = file.print_lines_reverse()?,
        (false, false, true) => content_to_print = file.print_chars_reverse()?,
        _ => Err(Error  { kind: ErrorKind::MultipleFlags, error: "Don't use multiple flags. It doesn't make sense.".to_string()})?,
    }
    println!("{}", content_to_print);
    Ok(file)
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