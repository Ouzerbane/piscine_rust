#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{
            head : None,
        }
    }

    pub fn push(&mut self, value: T) {
        let x = self.head.take();
        if x.is_some(){
           let y = x.unwrap();
            let newlist = Node  {
            value: value,
            next: Some(Box::new(y)) ,
            };
            self.head = Some(newlist) ;
        }else {
            let newlist = Node  {
            value: value,
            next: None ,
            };
            self.head = Some(newlist) ;           
        }
        
    }

   pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take()?;
        self.head = head.next.map(|boxed| *boxed);
        Some(head.value)
    }

   pub fn len(&self) -> usize {
    let mut count = 0;
    let mut current = self.head.as_ref();

    while let Some(node) = current {
        count += 1;
        current = node.next.as_deref();
    }

    count
}

}
