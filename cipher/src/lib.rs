#[derive(Debug, PartialEq)]
pub struct CipherError {
    // expected public fields
    pub expected : String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut cip = CipherError {expected:String::new()};
    for c in original.chars(){

        if !c.is_alphabetic() {
            cip.expected.push(c);
            continue ;
        }

        if c.is_lowercase() {
            let ciphra:u8 = 'z' as u8 - (c as u8 - 'a' as u8);
            cip.expected.push(ciphra as char );
            continue ;
        }

         if c.is_uppercase() {
            let ciphra:u8 = 'Z' as u8 - (c as u8 - 'A' as u8);
            cip.expected.push(ciphra as char );
            continue ;
        }

    }
    if cip.expected == ciphered {
        return Ok(());
    }else {
        return Err(cip)
    }

    // todo!()

}