pub fn number_logic(num: u32) -> bool {
    let len = num.to_string().len();
    let st =  num.to_string() ;
    let mut all:u32 = 0 ;
    for i in st.chars() {
        let n : u32 = i.to_digit(10).expect("f") ;
        all += n.pow(len as u32) ;
    } 
    if all == num {
        return true
    }
    false
}