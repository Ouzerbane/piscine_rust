pub fn get_diamond(c: char) -> Vec<String> {
    let mut space = (c as u8 - b'A') as i32; 
    let mut spaceend = 0; 
    
    let mut ve: Vec<String> = Vec::new();

    for cha in 'A'..=c {
        let mut stock = String::new();

        if cha == 'A' {
            let spc = " ".repeat(space.try_into().unwrap());
            stock.push_str(&format!("{}A{}", spc, spc));
            space -= 1;
            spaceend +=1;
            ve.push(stock);
            continue;
        }

        let spc = " ".repeat(space.try_into().unwrap());
        let spcend = " ".repeat(spaceend);
        stock.push_str(&format!("{}{}{}{}{}", spc, cha, spcend, cha,spc));
        ve.push(stock);
        if cha != c {
            space -= 1;
            spaceend += 2;
        }
    }
    for i in (0..ve.len()-1).rev(){
        if i == ve.len()-1 {
            break
        }
        ve.push((&ve[i]).to_string())
    }
    ve
}