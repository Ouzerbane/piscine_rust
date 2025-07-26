pub fn first_subword(mut s: String) -> String {
    let mut st :String = String::new();
    for (i,ch ) in s.char_indices() {
        if (ch == '_' || ch.is_uppercase()) && i != 0 {
            return st
        }  
        st.push(ch)
    }
     st
}