use std::sync::mpsc::Receiver;
use std::thread::{spawn, JoinHandle};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use super::paramcli::*;
use super::lineaggregate::*;

pub fn start_thread_grep(from_aggregate: Receiver<Simplelineaggregate>, data: &Paramcli) -> JoinHandle<()> {
    let mut to_search = String::new();
    to_search.push_str(&data.search);
    spawn(move || {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let mut founds=0;
        for l in from_aggregate {
            if l.where_to_search.contains(&to_search){
                founds+=1;
                for td in l.to_display{
                    write!(&mut stdout, "N°{} File:{}(line:{})==>", founds,&l.file,l.position).unwrap();               
                    if td == l.where_to_search{
                        stdout.set_color(ColorSpec::new().set_bg(Some(Color::Black)).set_fg(Some(Color::Green)),).unwrap();
                    }
                    writeln!(&mut stdout, "{}", &td).unwrap();               
                    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Black)).set_fg(Some(Color::White)),).unwrap();                                
                    }
                //println!("{}({})==>{}",&l.file,l.position,&l.where_to_search);
                println!();
            }
        }
    })
}
