pub fn score(letter : &str)->u64 {
    let mut cont : u64= 0;
    // let arr1 = ["A", "E", "I", "O", "U", "L", "N", "R", "S", "T"];
    // let arr2 = ["D", "G"];
    // let arr3 = ["B", "C", "M", "P"];
    // let arr4 = ["J", "X"];
    // let arr5 = ["Q", "Z"];

    for x in letter.chars() {
      let value  = match x.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0, 
        };
        cont += value as u64;
    }
    cont
}