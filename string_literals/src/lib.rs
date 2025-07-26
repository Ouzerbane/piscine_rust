pub fn is_empty(v: &str) -> bool {
    v == ""
}

pub fn is_ascii(v: &str) -> bool {
    for i in v.chars(){
        if !i.is_ascii(){
            return false
        }
    }
    return true
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let mut x = (&v[0..index],&v[index..]) ;
    x
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("REASON")
}