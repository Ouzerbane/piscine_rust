use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool {
            drops : Cell::new(0),
            states :  RefCell::new(vec![]),
        }
        // todo!()
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
     let x = self.thread_len();
     self.states.borrow_mut().push(false);
    (x,Thread::new(x,c,self))
        // todo!()
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
        // todo!()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
        // todo!()
    }

    pub fn drop_thread(&self, id: usize) {
         let mut x = self.states.borrow_mut();
         match x[id] {
            false => {x[id] = true ; self.drops.set(self.drops.get() + 1)} ,
            _ => {panic!("{} is already dropped",id)},
         }
        // todo!()
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
   pub pid:usize ,
   pub cmd : String ,
   pub parent :& 'a ThreadPool
    // expected public fields
}

impl <'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Thread {
            pid:p ,
            cmd : c ,
            parent : t ,
        }
    }

    pub fn skill(self) {
        drop(self)
        // todo!()
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid)
    }
}