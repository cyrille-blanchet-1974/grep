use std::sync::mpsc::Receiver;
use std::thread::{spawn, JoinHandle};

pub fn start_thread_grep(from_aggregate: Receiver<String>, count_line: &bool) -> JoinHandle<()> {
    let countline = *count_line;
    spawn(move || {
        let mut lc = 0;
        let mut wc = 0;
        for l in from_aggregate {
            lc += 1;
            if !countline {
                //compter les mots
                let v: Vec<&str> = l.rsplit(' ').collect();
                wc += v.len();
            }
        }
        if countline {
            println!("{}", lc);
        } else {
            println!("word:{}, line:{}", wc, lc);
        }
    })
}
