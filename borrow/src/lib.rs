pub fn str_len(s: &str)-> usize {
    let mut siz = 0 ;
    for i in s.chars() {
        siz +=1;
    }
    siz
}