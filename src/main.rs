use khat::{ Result, get_file_and_print };
use clap::Parser;

#[derive(Parser)]
struct Args {

    /// The file to the path
    file: String,

    #[arg(long, short = 'f')]
    /// Displays the file content reversing it entirely
    full_rev: bool,
    #[arg(long, short = 'l')]
    /// Displays the file content reversing only the lines
    line_rev: bool,
    #[arg(long, short = 'c')]
    /// Displays the file content reversing only the characters within the lines
    chars_rev: bool,
}




fn main() -> Result<()>  {
    let args = Args::parse();
    let content = get_file_and_print((args.file, args.full_rev, args.line_rev, args.chars_rev)).unwrap_or_else(|err| panic!("{}", err));
    println!("{}", content);
    Ok(())
}
