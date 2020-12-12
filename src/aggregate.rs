use std::sync::mpsc::{Receiver,Sender};
use std::thread::{spawn, JoinHandle};
use super::paramcli::*;
use super::lineread::*;
use super::lineaggregate::*;

pub fn start_thread_aggregate(from_read:Receiver<Lineread>, to_compute: Sender<Lineaggregate>, data: &Paramcli) -> JoinHandle<()>{
    let before = data.before;
    let after = data.after;
    spawn(move || {
        let mut file = String::new();
        file.push_str("");
        let mut res = Lineaggregate::new("",0,0);
        for l in from_read {
            //TODO: duplicate string and set them in a struct
            //struct will have lien N°, fileName,  n before lines, n afterline, line
            //line to search ....
            if l.file != file{
                res = Lineaggregate::new(&l.file,before,after);
                file.clear();
                file.push_str(&l.file);
            }
            res.addline(l.data);            
            if to_compute.send(res.clone()).is_err() {
                println!("error sending to write");
                return;
            }
        }
   })
}