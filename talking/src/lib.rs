pub fn talking(text: &str) -> &str {
    if text.trim() == "" {
        return "Just say something!"
    }
    
    let last = text.chars().last().unwrap();
    if last == '?'{
        let s = text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
        let l = text.chars().any(|c| c.is_alphabetic());
        if s && l{
            return "Quiet, I am thinking!" ;
        }else {
            return "Sure.";
        }
    
    } 
    let s = text.chars().all(|c| !c.is_alphabetic()|| c.is_uppercase());
    if s {
        return "There is no need to yell, calm down!";
    }
    return "Interesting";
}