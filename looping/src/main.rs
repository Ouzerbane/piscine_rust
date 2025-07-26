use std::io;

fn main() {
    let mut cont = 0 ;
    let riddle : String =String::from("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    loop {
        let mut input:String = String::new();
        cont +=1 ;
        println!("{}",riddle);
        io::stdin().read_line(&mut input).expect("moxkil input");
        if input.trim() == "The letter e" {
            println!("Number of trials: {}",cont);
            break;
        };
        

    }
}
