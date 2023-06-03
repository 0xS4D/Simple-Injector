use console::Term;
use injrs::inject_windows::*;
use injrs::process_windows::*;

use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn clear_c(c_move: u16) {
    if c_move == 0 {
        execute!(
            stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine),
        )
        .unwrap();
        return ();
    }

    execute!(
        stdout(),
        cursor::MoveUp(c_move),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    )
    .unwrap();
}

fn main() {

    let term = Term::stdout();
    let title = "x0S4D - owo";
    term.set_title(title);

    let name = "nice_.exe";
    let dll = "noname.dll";
    let mut infinite = false;

    let animation = ["-", "/", "|", "\\"];
    let mut counter = 4;

    if !std::path::Path::new(dll).exists() {
        println!(
            "Current directory: {}",
            std::env::current_dir().unwrap().display()
        );
        println!("Error: {} not found", dll);
        std::process::exit(exitcode::DATAERR);
    }

    println!("Do you want an infinite loop after injection? (y/n) - Default: n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "y" {
        infinite = true;
    }

    clear_c(2);

    loop {
        thread::sleep(Duration::from_secs(1));
        let p = Process::find_first_by_name(name);
        match p {
            None => {
                counter -= 1;
                if counter == 0 {
                    counter = 4;
                }
                clear_c(0);

                print!(
                    "{} segs waiting game and trying again... {}",
                    counter,
                    animation[counter - 1]
                );
                stdout().flush().unwrap();
            }

            Some(p) => {
                clear_c(0);
                println!("Process found!");
                thread::sleep(Duration::from_secs(3));
                match p.inject(dll) {
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                    Ok(_) => {
                        println!("Sucessfully injected into process, PID: {}", p.pid);
                        loop {
                            thread::sleep(Duration::from_secs(1));
                            match Process::find_first_by_name(name) {
                                Some(_) => {}
                                None => {
                                    println!("Process closed, exiting...");
                                    break;
                                }
                            }
                        }
                    }
                }
                if infinite {
                    clear_c(3);
                } else {
                    break;
                }
            }
        }
    }
}
