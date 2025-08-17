#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            return None;
        }
        let s =*self;
        // println!("{}",self.v);
        if self.v % 2 != 0 {
            let x =(3 * self.v) +1;
            self.v = x ;
            return  Some(s);
        }else {
            let x =self.v/2 ;
            self.v = x ;
           return  Some(s);
        }
    }
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz {v:n}
    }
}

pub fn collatz(n: u64) -> usize {
   Collatz::new(n).count()
}