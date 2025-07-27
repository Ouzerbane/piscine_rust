pub fn capitalize_first(input: &str) -> String {
        let mut chars = input.chars();
        match chars.next(){
            Some(f)=> f.to_uppercase().collect::<String>()+chars.as_str(),
            None => String::new(),
        }

}

pub fn title_case(input: &str) -> String {
    let mut splt : Vec<&str> = input.split(" ").collect();
    let mut stock : String = String::new();
    for  i in input.chars() {
        if stock == ""{
            stock.push_str(&capitalize_first(&i.to_string()));
            continue ;
        }
         if last_index(&stock) && i != ' '{
            stock.push_str(&capitalize_first(&i.to_string()));
            continue;

         }
        stock.push(i);
    }
    stock

}

pub fn last_index(s: &str)-> bool {
    match s.chars().last() {
        Some(c) => c == ' ' || c == '\t' ,
        None => false,
    }
}

pub fn change_case(input: &str) -> String {
    let mut strock : String = String::new();
    for i in input.chars() {
        if i == ' '{
            strock.push(i);
            continue ;
        }
        if i.is_uppercase(){
            strock.push_str(&i.to_lowercase().to_string());
            continue ;
        };
        if i.is_lowercase(){
            strock.push_str(&i.to_uppercase().to_string());
            continue;
        };

    }
    strock

}