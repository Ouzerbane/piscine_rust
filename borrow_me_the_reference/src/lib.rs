pub fn delete_and_backspace(s: &mut String) {
    let mut stock : String = String::new();
    let mut bo = 0 ;
    for i in s.chars(){
        if i == '-'{
            stock.pop();
            continue ;
        }
        if i == '+'{
            bo += 1 ;
            continue ;
        }
        if bo != 0 {
            bo -= 1 ;
            continue ;
        }
        stock.push(i)

    }
    *s = stock

}

pub fn do_operations(v: &mut [String]) {
    for i in 0..v.len(){
        let exp = &v[i];
        let mut nb1 : i32 = 0;
        let mut nb2 : i32 = 0 ;
        if exp.contains('+'){
            let el = exp.split('+');
            let mut vc : Vec<String> = Vec::new();
            for p in el {
                vc.push(p.to_string())
            }
            nb1 = vc[0].parse().expect("makaynx almoxkil");
            nb2 = vc[1].parse().expect("makaynx almoxkil") ;
            v[i]=  (nb1+nb2).to_string();
            continue;
           
        }
         if exp.contains('-'){
             let el = exp.split('-');
             let mut vc : Vec<String> = Vec::new();
            for p in el {
                vc.push(p.to_string())
            }
            nb1 = vc[0].parse().expect("makaynx almoxkil");
            nb2 = vc[1].parse().expect("makaynx almoxkil") ;
              v[i]=  (nb1-nb2).to_string();
              continue ;
        }
   
        
    }

}