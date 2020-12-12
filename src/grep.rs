use std::sync::mpsc::Receiver;
use std::thread::{spawn, JoinHandle};
use super::lineaggregate::*;
use super::paramcli::*;

pub fn start_thread_grep(from_aggregate: Receiver<Lineaggregate>, data: &Paramcli) -> JoinHandle<()> {
    let mut to_search = String::new();
    to_search.push_str(&data.search);
    spawn(move || {
        for l in from_aggregate {
            if l.data.contains(&to_search){
                println!("Trouvé {}",l.pos);
            }
        }
    })
}
