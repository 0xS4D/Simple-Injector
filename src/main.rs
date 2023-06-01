use console::Term;
use injrs::inject_windows::*;
use injrs::process_windows::*;

use crossterm::{cursor, execute, terminal};
use std::io;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    
    // Set console title
    let term = Term::stdout();
    let title = "x0S4D - owo";
    term.set_title(title);
    
    let name = "nice_game.exe";
    let dll = "nice_dll.dll";

    let animation = ["-", "/", "|", "\\"];
    let mut counter = 4;

    if std::path::Path::new(dll).exists() {
        loop {
            thread::sleep(Duration::from_secs(1));
            let p = Process::find_first_by_name(name);
            match p {
                Some(p) => {
                    execute!(
                        stdout(),
                        cursor::MoveToColumn(0),
                        terminal::Clear(terminal::ClearType::CurrentLine)
                    )
                    .unwrap();
                    println!("Injecting DLL into process...");
                    // wait 3 seconds
                    thread::sleep(Duration::from_secs(5));
                    match p.inject(dll) {
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                        Ok(_) => {
                            println!("Success!");
                        }
                    }
                    break;
                }
                None => {
                    counter -= 1;
                    if counter == 0 {
                        counter = 4;
                    }
                    execute!(
                        stdout(),
                        cursor::MoveToColumn(0),
                        terminal::Clear(terminal::ClearType::CurrentLine)
                    )
                    .unwrap();

                    print!(
                        "{} segs waiting game and trying again... {}",
                        counter,
                        animation[counter - 1]
                    );
                    stdout().flush().unwrap();
                }
            }
        }
    } else {
        println!(
            "Current directory: {}",
            std::env::current_dir().unwrap().display()
        );
        println!("Error: {} not found", dll);
    }

    println!("You can close me uwu");
    io::stdin().read_line(&mut String::new()).unwrap();

    // TODO: Ask for unload dll, close or start again.
}
