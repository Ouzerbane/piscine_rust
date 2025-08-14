pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str>  {
    // if prefix.len()>s.len(){
    //     return None ;
    // }
    if s.starts_with(prefix){
         Some(&s[prefix.len()..])
    }else{
        None
    }
   
}