use std::convert::TryInto;

#[derive(Debug)]
pub struct Lineaggregate{
    pub data_before:Vec<String>,   //the lines before the one where to search
    pub data:Vec<String>,          //the line where to search (Vec but size should always by 1 max)
    pub data_after:Vec<String>,    //the lines after the one where to search
    pub before:u32,                //nb of line max to keep in data_before
    pub after:u32,                 //nb of line max to keep in data_after
    pub pos:u32,                   //line position in the file
    pub file:String,               //filename where the line come from
    pub idx:u32,                   //+1 for each add() -1 for each get()
}

#[derive(Debug)]
pub struct Simplelineaggregate{
    pub file:String,               //filename where the line come from
    pub where_to_search:String,    //the line where to search (Vec but size should always by 1 max)
    pub to_display:Vec<String>,    //the lines before the one where to search
    pub position:u32,
}

impl Lineaggregate {
    pub fn new(file:&str,before:u8,after:u8) -> Lineaggregate {
        let mut f = String::new();
        f.push_str(&file);
        Lineaggregate{
            data_before:Vec::new(),
            data:Vec::new(),
            data_after:Vec::new(),
            before:before.try_into().unwrap(),
            after:after.try_into().unwrap(),
            pos:0,
            file:f,
            idx:0,
        }
    }

    /**
     * advande without adding a line
     */
    pub fn forward(&mut self){
        //if something in after push it in data
        if !self.data_after.is_empty() {
            self.data.push(self.data_after.remove(0));
        }
        //if data contains more than one push to before
        if !self.data.is_empty() {
            self.data_before.push(self.data.remove(0));
        }
        //cleaning
        if !self.data_before.is_empty() {
            self.data_before.remove(0);
        }
        //self.pos+=1;
        self.idx+=1;
    }

    /**
     * a a line to the struct
     */
    pub fn add(&mut self,new_data:String){
        //1st line goes in after
        self.data_after.push(new_data);
        //if after is full then use data
        if self.data_after.len() > self.after.try_into().unwrap() {
            self.data.push(self.data_after.remove(0));
        }
        //if data contains more than one push to before
        if self.data.len() > 1 {
            self.data_before.push(self.data.remove(0));
        }
        //cleaning
        if self.data_before.len() > self.before.try_into().unwrap() {
            self.data_before.remove(0);
        }
        //self.pos+=1;
        self.idx+=1;
    }
    /**
     * get 
     *      -filename
     *      -a line
     *      -line num of this line
     *      -a vec to display (line + n before + m after)
     * It also update cursors
     */
    pub fn get(&mut self)->Option<Simplelineaggregate>{
        let size = self.data_before.len() + self.data.len() + self.data_after.len();
        if size==0 || self.idx==0 || self.data.is_empty(){
            return None;
        } 
        self.idx-=1;
        self.pos +=1;
        let mut file = String::new();
        let mut data = String::new();
        let pos:u32=self.pos;
        let mut to_display= Vec::new();
        for d in &self.data_before{
            to_display.push(d.clone());
        }
        for d in &self.data{
            to_display.push(d.clone());
        }
        for d in &self.data_after{
            to_display.push(d.clone());
        }
        data.push_str(&self.data[0]);
        file.push_str(&self.file);

        Some(Simplelineaggregate{
            file,
            where_to_search:data,
            to_display,
            position:pos,                   
        })
    }
}

impl Clone for Lineaggregate {
    fn clone(&self) -> Lineaggregate {
        let mut file = String::new();
        file.push_str(&self.file);

        let mut data = Vec::new();
        for v in &self.data {
            data.push(v.clone());
        }

        let mut abuff = Vec::new();
        for v in &self.data_after {
            abuff.push(v.clone());
        }

        let mut bbuff = Vec::new();
        for v in &self.data_after {
            bbuff.push(v.clone());
        }
        Lineaggregate {
            data_before:bbuff,
            data,
            data_after:abuff,
            before:self.before,
            after:self.after,
            pos:self.pos,
            file,
            idx:self.idx,
        }
    }
}

#[test]
fn test_b0_a0() {
    let datas = vec!["A","B","C","D","E","F","G"];
    let expected_data = vec!["A","B","C","D","E","F","G"];
    let expected_datas = vec![vec!["A"], vec!["B"],vec!["C"],
                              vec!["D"],vec!["E"],
                              vec!["F"],vec!["G"]];
    let mut d = Lineaggregate::new("Test", 0, 0);
    let mut pos=0;
    for data in datas{
        d.add(data.to_string());
        println!("  add {:?}",&d);
        pos+=1;        
        let p:usize = (pos-1).try_into().unwrap();
        let res = d.get().unwrap();
        println!("      get {:?}\n      ->{:?}",&d,&res);                
        assert_eq!(expected_data[p], res.where_to_search);
        assert_eq!(pos,res.position);
        assert_eq!(expected_datas[p],res.to_display);
    }
    assert_eq!(false, match d.get(){None=>false,Some(_)=>true,});
}


#[test]
fn test_b1_a0() {
    let datas = vec!["A","B","C","D","E","F","G"];
    let expected_data = vec!["A","B","C","D","E","F","G"];
    let expected_datas = vec![vec!["A"], vec!["A","B"],vec!["B","C"],
                              vec!["C","D"],vec!["D","E"],
                              vec!["E","F"],vec!["F","G"]];
    let mut d = Lineaggregate::new("Test", 1, 0);
    let mut pos=0;
    for data in datas{
        d.add(data.to_string());
        println!("  add {:?}",&d);
        pos+=1;
        let p:usize = (pos-1).try_into().unwrap();
        let res = d.get().unwrap();
        println!("      get {:?}\n      ->{:?}",&d,&res);        
        assert_eq!(expected_data[p], res.where_to_search);
        assert_eq!(pos,res.position);
        assert_eq!(expected_datas[p],res.to_display);
    }
    assert_eq!(false, match d.get(){None=>false,Some(_)=>true,});
}

#[test]
fn test_b0_a1() {
    let datas = vec!["A","B","C","D","E","F","G"];
    let expected_data = vec!["A","B","C","D","E","F","G"];
    let expected_datas = vec![vec!["A","B"],vec!["B","C"],vec!["C","D"],
                              vec!["D","E"],vec!["E","F"],
                              vec!["F","G"],vec!["G"]];
    let mut d = Lineaggregate::new("Test", 0, 1);
    let mut pos:i32=-1;
    for data in datas{
        d.add(data.to_string());
        println!("  add {:?}",&d);
        pos+=1;
        if pos >0{
            let p:usize = (pos-1).try_into().unwrap();
            let res = d.get().unwrap();
            println!("      get {:?}\n      ->{:?}",&d,&res);        
            assert_eq!(expected_data[p], res.where_to_search);
            let p2:u32 = pos.try_into().unwrap();
            assert_eq!(p2,res.position);
            assert_eq!(expected_datas[p],res.to_display);
        }
    }
    let p:usize = 6;//datas.len()-1;
    d.forward();
    let res = d.get().unwrap();
    println!("      get {:?}->{:?}",&d,&res);        
    assert_eq!(expected_data[p], res.where_to_search);
    assert_eq!(7,res.position);
    assert_eq!(expected_datas[p],res.to_display);
    d.forward();
    assert_eq!(false, match d.get(){None=>false,Some(_)=>true,});
}

#[test]
fn test_b1_a1() {
    let datas = vec!["A","B","C","D","E","F","G"];
    let expected_data = vec!["A","B","C","D","E","F","G"];
    let expected_datas = vec![vec!["A","B"],vec!["A","B","C"],vec!["B","C","D"],
                              vec!["C","D","E"],vec!["D","E","F"],
                              vec!["E","F","G"],vec!["F","G"]];
    let mut d = Lineaggregate::new("Test", 1, 1);
    let mut pos:i32=-1;
    for data in datas{
        d.add(data.to_string());
        println!("  add {:?}",&d);
        pos+=1;
        if pos >0{
            let p:usize = (pos-1).try_into().unwrap();
            let res = d.get().unwrap();
            println!("      get {:?}\n      ->{:?}",&d,&res);        
            assert_eq!(expected_data[p], res.where_to_search);
            let p2:u32 = pos.try_into().unwrap();
            assert_eq!(p2,res.position);
            assert_eq!(expected_datas[p],res.to_display);
        }
    }
    let p:usize = 6;//datas.len()-1;
    d.forward();
    let res = d.get().unwrap();
    println!("      get {:?}->{:?}",&d,&res);        
    assert_eq!(expected_data[p], res.where_to_search);
    assert_eq!(7,res.position);
    assert_eq!(expected_datas[p],res.to_display);
    d.forward();
    assert_eq!(false, match d.get(){None=>false,Some(_)=>true,});
}


#[test]
fn test_b3_a3() {
    let datas = vec!["A","B","C","D","E","F","G"];
    let expected_data = vec!["A","B","C","D","E","F","G"];
    let expected_datas = vec![vec!["A","B","C","D"],vec!["A","B","C","D","E"],vec!["A","B","C","D","E","F"],
                              vec!["A","B","C","D","E","F","G"],vec!["B","C","D","E","F","G"],
                              vec!["C","D","E","F","G"],vec!["D","E","F","G"]];
    let mut d = Lineaggregate::new("Test", 3, 3);
    let mut pos:i32=-3;
    for data in datas{
        d.add(data.to_string());
        println!("  add {:?}",&d);
        pos+=1;
        if pos >0{
            let p:usize = (pos-1).try_into().unwrap();
            let res = d.get().unwrap();
            println!("      get {:?}\n      ->{:?}",&d,&res);        
            assert_eq!(expected_data[p], res.where_to_search);
            let p2:u32 = pos.try_into().unwrap();
            assert_eq!(p2,res.position);
            assert_eq!(expected_datas[p],res.to_display);
        }
    }
    let p:usize = 4;//datas.len()-1;
    d.forward();
    let res = d.get().unwrap();
    println!("      get {:?}->{:?}",&d,&res);        
    assert_eq!(expected_data[p], res.where_to_search);
    assert_eq!(5,res.position);
    assert_eq!(expected_datas[p],res.to_display);
    let p:usize = 5;//datas.len()-1;
    d.forward();
    let res = d.get().unwrap();
    println!("      get {:?}->{:?}",&d,&res);        
    assert_eq!(expected_data[p], res.where_to_search);
    assert_eq!(6,res.position);
    assert_eq!(expected_datas[p],res.to_display);
    let p:usize = 6;//datas.len()-1;
    d.forward();
    let res = d.get().unwrap();
    println!("      get {:?}->{:?}",&d,&res);        
    assert_eq!(expected_data[p], res.where_to_search);
    assert_eq!(7,res.position);
    assert_eq!(expected_datas[p],res.to_display);
    d.forward();
    assert_eq!(false, match d.get(){None=>false,Some(_)=>true,});
}