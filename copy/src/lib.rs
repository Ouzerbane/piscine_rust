pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let cc = c as f64 ;
    (c , cc.exp() ,cc.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let allnb : Vec<&str> = a.split(' ').collect();
    let mut stock : String = String::new();
    for (inedx ,val) in allnb.iter().enumerate() {
        let nb_float : f64 = val.parse().unwrap();
        stock.push_str(&nb_float.exp().to_string());
        if inedx != allnb.len()-1 {
        stock.push(' ');
        }
        
    }
    (a,stock)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut ner_vec : Vec<f64> = Vec::new();
    for i in &b {
        ner_vec.push((*i as f64).abs().ln())
    }
    (b,ner_vec)
}