mod paramcli;
mod read;
mod aggregate;
mod grep;
mod lineread;
mod lineaggregate;

use paramcli::*;
use read::*;
use aggregate::*;
use std::sync::mpsc::channel;
use grep::*;

pub fn traitement(p: &Paramcli) {
    //MPSC chanels
    let (to_aggregate, from_read) = channel();
    let (to_compute, from_aggregate) = channel();

    let hread = start_thread_read(to_aggregate,p);
    let haggregate = start_thread_aggregate(from_read,to_compute, &p);
    let hcompute = start_thread_grep(from_aggregate, &p);

    //wait for threads to stop
    if hread.join().is_err() {
        println!("Thread read finished with error");
    }
    if haggregate.join().is_err() {
        println!("Thread aggregate finished with error");
    }
    if hcompute.join().is_err() {
        println!("Thread compute finished with error");
    }
}

fn main() {
    let param = Paramcli::new();
    traitement(&param);
}
