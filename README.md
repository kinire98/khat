# khat
A cat clone implemented in Rust written in the most idiomatic way I know
# How to use it
First install the app: 
```bash
cargo intsall khat
```
## See help
If you want to get help for the subcommands:
```bash
khat -h
```
You will receive this output:
```bash
Usage: khat.exe [OPTIONS] <FILE>

Arguments:
  <FILE>  The file to the path

Options:
  -f, --full-rev   Displays the file content reversing it entirely
  -l, --line-rev   Displays the file content reversing only the lines
  -c, --chars-rev  Displays the file content reversing only the characters within the lines
  -h, --help       Print help
```
## Normal use
The normal way to use this will be to print the file as is:
```bash
khat Cargo.toml
```
Output: 
```bash
[package]
name = "khat"
version = "0.1.0"
edition = "2021"
license = "MIT"
description = "A cat clone, nothing more nothing less"
authors = ["kinire98"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
clap = { version = "4.4.10", features = ["derive"]}
```
##  Reversing lines
You can also reverse the lines of the document:
```bash
khat -l Cargo.toml
```
Output:
```bash
clap = { version = "4.4.10", features = ["derive"]}
[dependencies]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

authors = ["kinire98"]
description = "A cat clone, nothing more nothing less"
license = "MIT"
edition = "2021"
version = "0.1.0"
name = "khat"
[package]
```
## Reversing characters
Another option is reversing the characters within the line
```bash
khat -c Cargo.toml
```
Output:
```
]egakcap[
"tahk" = eman
"0.1.0" = noisrev
"1202" = noitide
"TIM" = esnecil
"ssel gnihton erom gnihton ,enolc tac A" = noitpircsed
]"89erinik"[ = srohtua

lmth.tsefinam/ecnerefer/ograc/gro.gnal-tsur.cod//:sptth ta snoitinifed rieht dna syek erom eeS #

]seicnedneped[
}]"evired"[ = serutaef ,"01.4.4" = noisrev { = palc
```
## Full reverse
As a final option you can combine both:
```bash
khat -r Cargo.toml
```