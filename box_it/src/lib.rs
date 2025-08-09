pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut vc : Vec::<Box<u32>> = Vec::new();

    for i in s.split_whitespace() {
        let last_str = i.chars().last().unwrap();
        if last_str == 'k' {
            let nmber = &i[0..i.len()-1];
            let pars :f64= nmber.parse().unwrap(); 
            vc.push(Box::new((pars * 1000.0)  as u32 ))
        }else {
            let pars :f64= i.parse().unwrap(); 
            vc.push(Box::new(pars as u32))
        }
    }
    
   vc
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut vc : Vec<u32> = Vec::new();
    for i in a {
        vc.push(*i);
    }
    vc
}