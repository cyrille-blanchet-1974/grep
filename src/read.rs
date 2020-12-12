use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

//TODO: add only one entry point 
//it receive input struct
// either it read from stdi
//or from a file
//or from a path
//or recursively
//in either case it return thread pointer
//can be a dedicated thread for path 
//and for recurse
pub fn start_thread_read_file(to_compute: Sender<String>, fic: &str) -> JoinHandle<()> {
    let file = String::from(fic);
    spawn(move || {
        let input = File::open(&file);
        match input {
            Err(e) => {
                println!("Error reading file {} => {}", &file, e);
            }
            Ok(f) => {
                let buffered = BufReader::new(f);
                for line in buffered.lines() {
                    if let Ok(l) = line {
                        if to_compute.send(l).is_err() {
                            println!("error sending to compute");
                            return;
                        }
                    }
                }
            }
        }
    })
}

pub fn start_thread_read_stdin(to_compute: Sender<String>) -> JoinHandle<()> {
    let stdin = io::stdin(); // We get `Stdin` here.
    spawn(move || {
        let buffered = BufReader::new(stdin);
        for line in buffered.lines() {
            if let Ok(l) = line {
                if to_compute.send(l).is_err() {
                    println!("error sending to compute");
                    return;
                }
            }
        }
    })
}
