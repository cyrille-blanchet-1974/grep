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
        let mut pos=1;
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
                pos=1;
            }
            res.addline(l.data);            
            res.compute_data(pos);
            if to_compute.send(res.clone()).is_err() {
                println!("error sending to write");
                return;
            }
            pos+=1;
        }
   })
}