use std::convert::TryInto;

pub struct Lineaggregate{
    pub data:String,             //the line
    pub pos:u32,                 //line position in the file
    pub file:String,             //filename where the line come from
    pub before:usize,              //nb of line before to display
    pub before_data:Vec<String>, //the 0-before line before the one where we search
    pub after:usize,               //nb of line after to display
    pub after_data:Vec<String>,  //the 0-after line before the one where we search
}

impl Lineaggregate {
    pub fn new(file:&str,before:u8,after:u8) -> Lineaggregate {
        let mut f = String::new();
        f.push_str(&file);
        Lineaggregate{
            data:String::new(),
            pos:0,
            file:f,
            before:before.try_into().unwrap(),
            before_data:Vec::new(),
            after:after.try_into().unwrap(),
            after_data:Vec::new(),
        }
    }

    pub fn addline(&mut self,new_data:String){
        //case no before/after
        if self.before==0 && self.after==0{
            self.data=new_data;
            return
        }
        //TODO: other case 
        //case only after
        if self.before==0 && self.after>0{
            //if after_data is full,
            if self.after == self.after_data.len(){
                //its first element goes in data
                self.data = self.after_data.remove(0);
            }
            //add ne data at the end of
            self.after_data.push(new_data);
            return;
        }
        //case only before
        if self.before>0 && self.after==0{
            //if before_data is full,
            if self.before == self.before_data.len(){
                //remove its first element
                self.after_data.remove(0);
                let mut tmp = String::new();
                tmp.push_str(&self.data);
                //push data at the end
                self.after_data.push(tmp);
            }
            //new_data is the new data
            self.data = new_data;
        }
        //case before and after
        if self.before>0 && self.after>0{
        }
    }
}

impl Clone for Lineaggregate {
    fn clone(&self) -> Lineaggregate {
        let mut data = String::new();
        data.push_str(&self.data);
    
        let mut file = String::new();
        file.push_str(&self.file);

        let mut before_data = Vec::new();
        for v in &self.before_data {
            before_data.push(v.clone());
        }
        let mut after_data = Vec::new();
        for v in &self.after_data {
            after_data.push(v.clone());
        }
        Lineaggregate {
            data,
            pos:self.pos,
            file,
            before:self.before,
            before_data,
            after:self.after,
            after_data,
        }
    }
}
