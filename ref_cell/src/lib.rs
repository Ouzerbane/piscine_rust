use std::rc::Rc;
use std::cell::RefCell;

pub struct Tracker {
   pub messages:  RefCell<Vec<String>>,
    value: usize,       
    max: usize,
}


impl Tracker {
    pub fn new(max: usize) -> Self {
        Tracker {
            messages: RefCell::new(Vec::new()) ,
            value: 0,
            max,
        }
    }

    pub fn set_value(&self , value : &Rc<usize>){
        let reff = Rc::strong_count(value); 
        if reff > self.max {
            self.messages.borrow_mut().push(format!("Error: You can't go over your quota!"));
        }else {
            let x = (reff * 100) / self.max;
            if x > 70 {
                self.messages.borrow_mut().push(format!("Warning: You have used up over {}% of your quota!",x));
            }
           
        }
    }

    pub fn peek(&self , value : &Rc<usize>){
        let reff = Rc::strong_count(value); 
        let x = (reff * 100) / self.max;
        self.messages.borrow_mut().push(format!("Info: This value would use {}% of your quota",x));

    }
}