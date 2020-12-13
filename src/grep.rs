use std::sync::mpsc::Receiver;
use std::thread::{spawn, JoinHandle};
use super::paramcli::*;
use super::lineaggregate::*;

pub fn start_thread_grep(from_aggregate: Receiver<Simplelineaggregate>, data: &Paramcli) -> JoinHandle<()> {
    let mut to_search = String::new();
    to_search.push_str(&data.search);
    spawn(move || {
        for l in from_aggregate {
            if l.where_to_search.contains(&to_search){
                println!("{}({})==>{}",&l.file,l.position,&l.where_to_search);
            }
        }
    })
}
