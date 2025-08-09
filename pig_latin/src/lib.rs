pub fn pig_latin(text: &str) -> String {
     let alpha = "aeiou";
    let first = &text[0..1];
    let mut rt : String = String::new();
    if !alpha.contains(&first){
        if text.len()>2{
            if &text[1..3]== "qu"{
                rt.push_str(&text[3..]);
                rt.push_str(first);
                rt.push_str(&text[1..3]);
                rt.push_str("ay");
                return rt ;
            }
        }
    }else {
        rt.push_str(&text);
        rt.push_str("ay");
        return rt ;
    }
    let mut x : String = String::new();
    let mut y : String = String::new();

    for (i,v) in text.char_indices(){
        if alpha.contains(v) {
            rt.push_str(&text[i..]);
            rt.push_str(&x);
            rt.push_str("ay");
            return rt ;
        }
        x.push(v);
    }
    rt
}
