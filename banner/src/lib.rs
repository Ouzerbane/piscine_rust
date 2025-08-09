use std::{collections::HashMap, num::ParseFloatError};

pub struct  Flag {
   pub short_hand: String ,
    pub long_hand: String ,
    pub desc: String
}
impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
         Flag {
        short_hand : format!("-{}",name.chars().next().expect("ff").to_string()),
         long_hand : format!("--{}",name) ,
         desc : d.to_string()
    }
    }
}


pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;


pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand,func);
        self.flags.insert(flag.long_hand,func);

        
        // todo!()
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
    if argv.len() < 2 {
        return Err("Not enough arguments".to_string());
    }

    let callback = match self.flags.get(input) {
        Some(cb) => cb,
        None => return Err(format!("No flag found for '{}'", input)),
    };

     let result = callback(argv[0], argv[1]);
         match  result  {
            Ok(val)=>return Ok(val) ,
            Err(er)=>return Err(er.to_string()) ,
         }
}

}


pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
     let a1 = a.parse::<f64>();
   let a11 = match a1 {
        Ok(mb)=> mb,
        Err(e)=>return Err(e),
    };

    let b1 = b.parse::<f64>();
   let b11=  match b1 {
        Ok(mb)=> mb,
        Err(e)=>return Err(e),
    };
    Ok((a11 / b11).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a1 = a.parse::<f64>();
   let a11 = match a1 {
        Ok(mb)=> mb,
        Err(e)=>return Err(e),
    };

    let b1 = b.parse::<f64>();
   let b11=  match b1 {
        Ok(mb)=> mb,
        Err(e)=>return Err(e),
    };
   Ok((a11 % b11).to_string())
    // todo!()
}