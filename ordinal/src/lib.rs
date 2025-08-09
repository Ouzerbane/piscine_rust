pub fn num_to_ordinal(x: u32) -> String {
    // let st = x.to_string();
    // let index = st.chars().last().unwrap();
    if x % 100 ==11 || x % 100 ==12 || x % 100 ==13 {
        return format!("{}th",x);
    }

    match x % 10 {
        1=> return format!("{}st",x) ,
        2=> return format!("{}nd",x) ,
        3=> return format!("{}rd",x) ,
        _=> return format!("{}th",x) ,
    }
    // println!("{}",index);
    // st
}