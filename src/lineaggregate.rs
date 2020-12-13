use std::convert::TryInto;

pub struct Lineaggregate{
    pub data:String,               //the line
    pub pos:u32,                   //line position in the file
    pub file:String,               //filename where the line come from
    pub display_size:usize,        //size of display buffer
    pub display_data:Vec<String>,  //the lines to display
    pub display_data_start:usize,  //num line of the first position in display_data
}

impl Lineaggregate {
    pub fn new(file:&str,before:u8,after:u8) -> Lineaggregate {
        let mut f = String::new();
        f.push_str(&file);
        let display_size = (before+1+after).try_into().unwrap();
        Lineaggregate{
            data:String::new(),
            pos:0,
            file:f,
            display_size,
            display_data:Vec::new(),
            display_data_start:0,
        }
    }

    pub fn compute_data(&mut self,pos:u32){
        let mut res=String::new();
        //compute position in buffer
        let t:usize=pos.try_into().unwrap();
        let p:usize = t - self.display_data_start;
        res.push_str(&self.display_data[p -1]);
        self.data = res;
        self.pos=pos;
    }

    pub fn addline(&mut self,new_data:String){
        //case of first use
        if self.display_data.len() == 0 {
            self.pos=1;
        }
        //add the new data at the end of the buffer
        self.display_data.push(new_data);
        //check if buffer exceed max size
        if self.display_data.len() > self.display_size{
            //in this case remove the first line
            self.display_data.remove(0);
            //and change the position of the buffer
            self.display_data_start+=1;
        }
    }
}

impl Clone for Lineaggregate {
    fn clone(&self) -> Lineaggregate {
        let mut data = String::new();
        data.push_str(&self.data);
    
        let mut file = String::new();
        file.push_str(&self.file);

        let mut buff = Vec::new();
        for v in &self.display_data {
            buff.push(v.clone());
        }
        Lineaggregate {
            data,
            pos:self.pos,
            file,
            display_size:self.display_size,
            display_data:buff,
            display_data_start:self.display_data_start,
        }
    }
}

#[test]
fn test_b0_a0() {
    let datas = vec!["A","B","C","D","E","F","G"];
    let mut d = Lineaggregate::new("Test", 0, 0);
    let mut pos=0;
    for data in datas{
        d.addline(data.to_string());
        pos+=1;
        d.compute_data(pos);
        assert_eq!(data, d.data);
        if pos == 3{
            assert_eq!(1, d.display_data.len());
            assert_eq!(2,d.display_data_start);
            assert_eq!("C",d.data);
        }    
    }
    assert_eq!(1, d.display_data.len());
    assert_eq!(6,d.display_data_start);
    assert_eq!("G",d.data);
}

#[test]
fn test_b1_a0() {
    let datas = vec!["A","B","C","D","E","F","G"];
    let mut d = Lineaggregate::new("Test", 1, 0);
    let mut pos=0;
    for data in datas{
        d.addline(data.to_string());
        pos+=1;
        d.compute_data(pos);
        assert_eq!(data, d.data);
        if pos == 3{
            assert_eq!(2, d.display_data.len());
            assert_eq!(1,d.display_data_start);
            assert_eq!("C",d.data);
        }    
    }
   /* pos+=1;
    d.compute_data(pos);

    assert_eq!(2, d.display_data.len());
    assert_eq!(6,d.display_data_start);
    assert_eq!("G",d.data);*/
}