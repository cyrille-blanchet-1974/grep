use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::{Receiver,Sender};
use std::thread::{spawn, JoinHandle};
use super::paramcli::*;

pub fn start_thread_aggregate(from_read:Receiver<String>, to_compute: Sender<String>, data: &Paramcli) -> JoinHandle<()>{
    spawn(move || {
        for l in from_read {
            //TODO: duplicate string and set them in a struct
            //struct will have lien N°, fileName,  n before lines, n afterline, line
            //line to search ....
            let mut res= String::new();
            if to_compute.send(res).is_err() {
                println!("error sending to write");
                return;
            }
        }
   })
}