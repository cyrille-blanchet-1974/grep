use super::lineaggregate::*;
use super::lineread::*;
use super::paramcli::*;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{spawn, JoinHandle};

pub fn start_thread_aggregate(
    from_read: Receiver<Lineread>,
    to_compute: Sender<Simplelineaggregate>,
    data: &Paramcli,
) -> JoinHandle<()> {
    let before = data.before;
    let after = data.after;
    let case_sensitive = data.case_sensitive;
    spawn(move || {
        let mut file = String::new();
        file.push_str("");
        let mut res = Lineaggregate::new("", 0, 0, case_sensitive);
        for l in from_read {
            if l.file != file {
                //if not 1st file
                if !file.is_empty() {
                    //continue to advanced
                    loop {
                        res.forward();
                        match res.get() {
                            None => {
                                break;
                            }
                            Some(x) => {
                                if to_compute.send(x).is_err() {
                                    println!("error sending to write");
                                    return;
                                }
                            }
                        }
                    }
                }
                res = Lineaggregate::new(&l.file, before, after, case_sensitive);
                file.clear();
                file.push_str(&l.file);
            }
            res.add(l.data);
            match res.get() {
                None => {
                    //break;
                }
                Some(x) => {
                    if to_compute.send(x).is_err() {
                        println!("error sending to write");
                        return;
                    }
                }
            }
        } //loop reads
          //continue to advanced
        loop {
            res.forward();
            match res.get() {
                None => {
                    break;
                }
                Some(x) => {
                    if to_compute.send(x).is_err() {
                        println!("error sending to write");
                        return;
                    }
                }
            }
        }
    })
}
