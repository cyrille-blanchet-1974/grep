pub struct Lineread {
    pub data: String, //the line
    pub pos: u32,     //line position in the file
    pub file: String, //filename where the line come from
}

impl Default for Lineread {
    fn default() -> Self {
        Lineread::new("", 0, "")
    }
}

impl Lineread {
    pub fn new(file: &str, pos: u32, data: &str) -> Lineread {
        let mut f = String::new();
        let mut d = String::new();
        f.push_str(&file);
        d.push_str(&data);
        Lineread {
            file: f,
            pos,
            data: d,
        }
    }
}
