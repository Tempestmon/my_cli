use std::fs;
use std::path::Path;
use std::str::FromStr;
use clap::{Parser};

#[derive(Debug)]
enum Command {
    Ls(Ls),
    Cat(Cat),
    Find(Find),
    Cd(Cd),
    Rm(Rm),
    Touch(Touch),
}

trait Executable {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), String>;
}

impl Executable for Command {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), String> {
        match self {
            Command::Ls(c) => {c.execute(args)}
            Command::Cat(c) => {c.execute(args)}
            Command::Find(c) => {c.execute(args)}
            Command::Cd(c) => {c.execute(args)}
            Command::Rm(c) => {c.execute(args)}
            Command::Touch(c) => {c.execute(args)}
        }
    }
}

#[derive(Debug)]
struct Ls {}

impl Executable for Ls {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), String> {

        match args {
            None => {
                let paths = fs::read_dir("./").unwrap();

                for path in paths {
                    print!("{}\t", path.unwrap().path().display())
                }
            }
            Some(a) => {
                todo!()
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Cat {}

impl Executable for Cat {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), String> {
        match args {
            Some(a) => {
                let path = Path::new(a.get(0).unwrap());
                let content = fs::read_to_string(path)
                    .expect("Should have been able to read the file");
                println!("{content}");
                Ok(())
            }
            None => {
                Err("Got no arguments".parse().unwrap())
            }
        }
    }
}

#[derive(Debug)]
struct Find {}

impl Executable for Find {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), String> {
        todo!()
    }
}

#[derive(Debug)]
struct Cd {}

impl Executable for Cd {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), String> {
        todo!()
    }
}

#[derive(Debug)]
struct Rm {}

impl Executable for Rm {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), String> {
        todo!()
    }
}

#[derive(Debug)]
struct Touch {}

impl Executable for Touch {
    fn execute(&self, args: Option<Vec<String>>) -> Result<(), String> {
        todo!()
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "ls" => {
                Ok(Command::Ls(Ls {}))
            }
            "cd" => {
                Ok(Command::Cd(Cd {}))
            }
            "find" => {
                Ok(Command::Find(Find {}))
            }
            "cat" => {
                Ok(Command::Cat(Cat {}))
            }
            "rm" => {
                Ok(Command::Rm(Rm {}))
            }
            "touch" => {
                Ok(Command::Touch(Touch {}))
            }
            _ => { Err(()) }
        }
    }
}

#[derive(Parser)]
struct Cli {
    command: Option<String>,
    argument: Option<String>,
}

fn main() {
    let parsed = Cli::parse();
    let command = parsed.command;
    let arguments = parsed.argument;
    let arguments = match arguments {
        None => None,
        Some(a) => Some(vec![a])
    };

    let command = Command::from_str(command.expect("Got no command").as_str()).expect("Command is not supported");

    command.execute(arguments).unwrap();
}