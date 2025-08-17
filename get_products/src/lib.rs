pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut vc:Vec<usize> = Vec::new();
    if arr.len()==1{
        return  vc ;
    }
    for i in 0..arr.len() {
        let mut prod = 1;
        for j in 0..arr.len() {
            if i != j {
                prod *= arr[j];
            }
        }
        vc.push(prod as usize);
    }
    vc
}
