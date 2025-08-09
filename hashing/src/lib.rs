pub fn mean(list: &[i32]) -> f64 {
    let sum : i32 = list.iter().sum() ;
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut x : Vec<i32> = list.to_vec();
    x.sort();
    if x.len()%2 == 1 {
        x[x.len()/2]
    }else {
       ( x[x.len()/2]+x[x.len()/2 - 1]) / 2
    }

}

pub fn mode(list: &[i32]) -> i32 {
    let mut k = 0 ;
    let mut y : i32 = 0 ;
    for i in list {
        let mut j = 0 ;
        for x in list {
            if i == x {
                j += 1 ;
            };
        }
        if j > k {
            y = *i as i32  ;
            k = j ;
        } 
    }
    y
}