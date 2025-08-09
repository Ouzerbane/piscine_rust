 use std::collections::HashMap;
pub fn is_pangram(s: &str) -> bool {
    let mut map =HashMap::new();
    for x in s.chars(){
        
        if x.is_ascii_alphabetic(){
            let va = x.to_lowercase().next().unwrap() ;
            map.insert( va,va);

        };
    }
    println!("{}",map.len());
    map.len()==26
    // todo!()
}