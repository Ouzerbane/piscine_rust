pub fn stars(n: u32) -> String {
    let mut stor : String = String::new();
    let ex = 2_i32.pow(n);
    for i in 0..ex {
        stor.push('*') ;
    } 
    stor

}