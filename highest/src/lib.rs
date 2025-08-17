#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a[u32]) -> Self {
        Numbers{numbers}
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.len() ==0 {
            return None;
        } 
        Some(self.numbers[self.numbers.len()-1])
    }

    pub fn highest(&self) -> Option<u32> {
       let  x = self.numbers.iter().max()?;
    //    println!("{:?}",x);
       Some(*x)
    }

    pub fn highest_three(&self) -> Vec<u32> {
        if self.numbers.len()==0 {
            return  vec![];
        }
        let mut vec = self.numbers.to_vec();
        vec.sort();
        vec.reverse();
        if self.numbers.len() < 3 {
            return  vec;
        }
        vec[0..3].to_vec()

    }
}