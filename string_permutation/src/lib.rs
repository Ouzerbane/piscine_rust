use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false 
    };
    let mut has  = HashMap::new();
    let mut count = 0 ; 
      for i in s1.chars() {
        for j in s1.chars(){
            if i== j{
                count+=1;
            }

        }
        has.insert(i,count);
        count = 0
    }

    for i in s2.chars() {
        for j in s2.chars(){
            if i== j{
                count+=1;
            }

        }
        if Some(&count) != has.get(&i) {
         return false;
        };
        count = 0 ;
    }
    true

}