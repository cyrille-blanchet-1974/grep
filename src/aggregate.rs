use std::sync::mpsc::{Receiver,Sender};
use std::thread::{spawn, JoinHandle};
use super::paramcli::*;
use super::lineread::*;
use super::lineaggregate::*;

pub fn start_thread_aggregate(from_read:Receiver<Lineread>, to_compute: Sender<Simplelineaggregate>, data: &Paramcli) -> JoinHandle<()>{
    let before = data.before;
    let after = data.after;
    spawn(move || {
        let mut file = String::new();
        file.push_str("");
        let mut res = Lineaggregate::new("",0,0);
        for l in from_read {
            if l.file != file{
                //if not 1st file
                if file!=""{
                    //continue to advanced
                    loop {
                        res.forward();
                        match res.get(){
                            None => {
                                break;
                                },
                            Some(x)=>{
                                if to_compute.send(x).is_err() {
                                    println!("error sending to write");
                                    return;
                                }
                            }
                        }
                    } 
                }
                res = Lineaggregate::new(&l.file,before,after);
                file.clear();
                file.push_str(&l.file);
            }
            res.add(l.data); 
            match res.get(){
                None => {
                    //break;
                    },
                Some(x)=>{
                    if to_compute.send(x).is_err() {
                        println!("error sending to write");
                        return;
                    }
                }
            }
        }//loop reads
        //continue to advanced
        loop {
            res.forward();
            match res.get(){
                None => {
                    break;
                    },
                Some(x)=>{
                    if to_compute.send(x).is_err() {
                        println!("error sending to write");
                        return;
                    }
                }
            }
        }        
   })
}