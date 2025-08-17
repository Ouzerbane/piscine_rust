#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub item :Vec<(String, f32)> ,
    pub receipt : Vec<f32>
    // expected public fields
}
impl Cart {
    pub fn new() -> Cart {
        Cart {item:vec![],receipt:vec![]}
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let pri = s.products.iter().find(|x|ele==x.0);
        let pr = pri.unwrap();
        self.item.push(((&pr.0).to_string(),pr.1));
        self.receipt.push(pr.1);
        
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // let mut retun :Vec<f32> = Vec::new();
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let ind = self.receipt.len()/3 ;
        let k =self.receipt.iter().take(ind);
        let sum : f32 = k.sum();
        let total:f32 = self.receipt.iter().sum();
        let prosontage = (total-sum)/total ;
        let new_v: Vec<f32> = self.receipt.iter().map(|x| ((x * prosontage)*100.0).round()/100.).collect();
         self.receipt=new_v.clone() ;
       
        new_v
    }
}