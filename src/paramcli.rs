use std::env;
use std::fs::File;

#[derive(Debug)]
pub struct Paramcli
//parameters from command line and/or confFile
{
    pub input: String,      //a file, a path, or stdin  
    pub search: String,     //what to search
    pub before:u8,          //number of line before what found do we output
    pub after:u8,           //same but after
    pub case_sensitive:bool,
    pub inverse_search:bool, //return line that do not contain searche string  
    pub recurse:bool,
}

impl Default for Paramcli {
    fn default() -> Self {
        Paramcli::new()
    }
}

impl Paramcli {
    pub fn new() -> Paramcli {
        let mut input = String::new();
        let mut search= String::new();
        let mut before =0;
        let mut after=0;
        let mut case_sensitive=false;
        let mut inverse_search=false;  
        let mut recurse=false;
        let args: Vec<String> = env::args().skip(1).collect();
        let name = env::args()
            .take(1)
            .next()
            .unwrap_or_else(|| String::from("grep"));
        for arg in args {
            if arg == "-?"
                || arg.to_lowercase() == "-help"
            {
                help(&name);
            }
            if arg.to_lowercase() == "-recurse" {
                recurse = true;
                continue;
            }
            //TODO : complete
        }
        //checks
        /*if !fic.is_empty() {
            //check if file exists
            if File::open(&fic).is_err() {
                println!("Error file {} doesn't exists or unereadable", &fic);
                help(&name);
            };
        }*/
        Paramcli { 
            after,
            before,
            case_sensitive,
            input,
            inverse_search,
            search,
            recurse,
         }
    }
}

fn help(name: &str) {
    println!("{} 1.0 (2020)", name);
    println!("syntax : {} 'what to search' [where to search] [-aftern] [-beforen] [-case_sensitive] [-inverse] [-recurse] [-help]", name);
    println!("parameters between [] are optionnals");
    println!("'what to search': string to search"); 
    println!("                  must be the first parameter not starting with '-'");
    println!("[where to search]: if present give the file, the file or the path where to search");
    println!("                   must be the second parameter not stating with '-'");
    println!("                   if not present search will be on stdin");
    println!("------------------------------------");
    println!("-aftern: output will display n lines after what we found");
    println!("-beforen: output will display n lines before what we found");
    println!("-case_sensitive: search take account of case");
    println!("-inverse: search line that do not contain 'what to search'");
    println!("-recurse: search also on sub folders");
    std::process::exit(0);
}
