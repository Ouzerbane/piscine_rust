pub fn rev_str(input: &str) -> String {
    let mut stock : String = String::new();
    for i in input.chars().rev() {
        stock.push(i);
    }
    stock
}
