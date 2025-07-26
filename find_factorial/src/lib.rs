pub fn factorial(num: u64)->u64{
    let mut stck = 1 ;
   
        for i in 1..num+1 {
            stck = stck*i
        }
   stck
}