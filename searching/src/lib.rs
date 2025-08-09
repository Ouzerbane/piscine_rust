pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let seveindex: usize = 0 ;
    for (i,v) in array.iter().enumerate().rev(){
        if v == &key {
            return Some(i as usize)
        }
    }
    return None ;
    // return Some(seveindex)
}