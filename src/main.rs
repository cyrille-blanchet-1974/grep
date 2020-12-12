mod paramcli;
mod read;
mod grep;

use paramcli::*;
use read::*;
use std::sync::mpsc::channel;
use grep::*;

pub fn traitement(p: &Paramcli) {
    //MPSC chanels
    let (to_compute, from_read) = channel();

    let hread = //if !p.fic.is_empty() {
        //start_thread_read_file(to_compute, &p.fic)
    //} else {
        start_thread_read_stdin(to_compute);
    //};
    let hcompute = start_thread_grep(from_read, &true);

    //wait for threads to stop
    if hread.join().is_err() {
        println!("Thread read finished with error");
    }
    if hcompute.join().is_err() {
        println!("Thread compute finished with error");
    }
}

fn main() {
    let param = Paramcli::new();
    traitement(&param);
}
