pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message == "" {
        return "".to_string() ;
    }
    let mut lop = message.len()/i ;
    if  message.len()%i !=0 {
        lop +=  1 ;
    }
    let mut stock : String = String::new();
    let mut vec : Vec<Vec<char>> = vec![vec![' ';i];lop];
    let mut st : String = String::from(message);
    for id in 0..lop {
        vec[id]=vec![' ';i];
        let mut cont = 0 ;
        for (ind , v) in st.char_indices() {
            cont+=1 ;
            vec[id][ind]=v;
            if ind == i-1  {
                break ;
            }
        }
        st = format!("{}",&st[cont..]);
    }
    for ind in 0..i {
        for index in 0..lop{
            stock.push(vec[index][ind])
        }
    } 
    stock.trim().to_string()
    // println!("{:?}",vec);
    // "ffd".to_string()
}