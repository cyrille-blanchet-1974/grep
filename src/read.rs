use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};
use std::convert::TryInto;
use super::paramcli::*;
use super::lineread::*;

pub fn start_thread_read(to_aggregate: Sender<Lineread>, data: &Paramcli) -> JoinHandle<()>{
    if data.input.is_empty(){
        return start_thread_read_stdin(to_aggregate); 
    }
    //check if file exists
    if File::open(&data.input).is_ok() {
        //input is a file
        return start_thread_read_file(to_aggregate, &data.input);
    };
    start_thread_read_files(to_aggregate, data)
}

fn start_thread_read_files(to_aggregate: Sender<Lineread>, data: &Paramcli) -> JoinHandle<()> {
    //TODO:input is a path either get all files or  recurse in it
    println!("multi-file and recurse grep not yep implemented !!");
    //TODO
    let file = String::from(&data.input);
    spawn(move || {
        let input = File::open(&file);
        match input {
            Err(e) => {
                println!("Error reading file {} => {}", &file, e);
            }
            Ok(f) => {
                let buffered = BufReader::new(f);
                for (pos, line) in buffered.lines().enumerate(){
                    if let Ok(l) = line {
                        let p:u32=pos.try_into().unwrap();
                        let lr = Lineread::new(&file,p,&l);
                        if to_aggregate.send(lr).is_err() {
                            println!("error sending to compute");
                            return;
                        }
                    }
                }
            }
        }
    })
}




fn start_thread_read_file(to_aggregate: Sender<Lineread>, fic: &str) -> JoinHandle<()> {
    let file = String::from(fic);
    spawn(move || {
        let input = File::open(&file);
        match input {
            Err(e) => {
                println!("Error reading file {} => {}", &file, e);
            }
            Ok(f) => {
                let buffered = BufReader::new(f);
                for (pos, line) in buffered.lines().enumerate(){
                    if let Ok(l) = line {
                        let p:u32=pos.try_into().unwrap();
                        let lr = Lineread::new(&file,p,&l);
                        if to_aggregate.send(lr).is_err() {
                            println!("error sending to compute");
                            return;
                        }
                    }
                }
            }
        }
    })
}

fn start_thread_read_stdin(to_aggregate: Sender<Lineread>) -> JoinHandle<()> {
    let stdin = io::stdin(); // We get `Stdin` here.
    spawn(move || {
        let buffered = BufReader::new(stdin);
        for (pos, line) in buffered.lines().enumerate(){
            if let Ok(l) = line {
                let p:u32=pos.try_into().unwrap();
                let lr = Lineread::new("",p,&l);
                if to_aggregate.send(lr).is_err() {
                    println!("error sending to compute");
                    return;
                }                
            }
        }
    })
}
