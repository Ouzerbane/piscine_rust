use chrono::Local;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
   pub form_values : (&'static str,String) ,
   pub date : String ,
   pub err : &'static str ,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError {
            form_values : (field_name , field_value),
            date : Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err : err
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
   pub name : String ,
   pub password : String 
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.len()== 0 {
            return Err(FormError::new("name" , self.name.clone() , "Username is empty" ))
        };
          if self.password.len()<8 {
           return Err(FormError::new("password" , self.password.clone() , "Password should be at least 8 characters long"));   
        };
        let mut nember = false ;
        let mut alpha = false ;
        let mut symbol = false ;
        for i in self.password.chars(){
            if i.is_alphabetic(){
                alpha = true ;
                continue ;
            }
            if i.is_numeric() {
                nember = true ;
                continue ;
            }
            if !i.is_alphanumeric(){
                symbol = true ;
            }
        }
        if !nember || !alpha || !symbol {
           return Err(FormError::new("password" , self.password.clone() , "Password should be a combination of ASCII numbers, letters and symbols"));
        }

        Ok(())
    }
      
     
        
}