pub fn arrange_phrase(phrase: &str) -> String {
     let st = phrase.split(' ') ;

    let mut v : Vec<String> = Vec::new();
    let mut stock : String = String::new();
    for i in st {
        v.push(i.to_string());
    }
    let mut nb = 0 ;
    let mut nember_in_st = 0 ;
    let mut index = 0 ;
    let mut postion = 0 ;
    while v.len() > 0 {
        let ele = &v[nb];
        for (x , v )in ele.char_indices() {
            if v.is_numeric() {
                let  digit_i32: i32 = (v as i32) - ('0' as i32);
                if nember_in_st == 0 {
                    nember_in_st = digit_i32;
                    index = nb;
                    postion = x ;
                }
                if nember_in_st > digit_i32 {
                    nember_in_st = digit_i32 ;
                    index = nb;
                    postion = x ;
                } 
                break ;
            }
        }
        if nb ==v.len()-1 {
            let part1 = &v[index][0..postion];
            let part2 = &v[index][postion+1..];
            stock.push_str(&format!("{}{} ",part1,part2));
            v.remove(index) ;
            nb = 0 ;
            nember_in_st = 0 ;
            continue;
        }
        nb += 1 ;
       
    }

   
  stock.trim().to_string()
}