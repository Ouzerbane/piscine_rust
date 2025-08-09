pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Warning => {
            match server {
                Ok(sta) =>  sta.to_string(),
                Err(_) =>  "WARNING: check the server".to_string(),
            }
        },
        Security::Unknown => {
            server.unwrap().to_string()
        },
        Security::Message => {
            match server {
                Ok(sta) =>  sta.to_string(),
                Err(_) => panic!("ERROR: program stops"),
            }
        },

        Security::NotFound => {
            match server {
                Ok(sta) =>  sta.to_string(),
                Err(sta) =>  format!("Not found: {}",sta),
            }
        },

            Security::UnexpectedUrl => {
            match server {
               Err(err_msg) => err_msg.to_string(),
                Ok(url) => panic!("Unexpected URL: {}", url),
            }
        },
       
    }
}


// use unwrap_or_expect::*;

// fn main() {
//     println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
//     println!("{}", fetch_data(Err("server.com"), Security::Warning));
//     println!("{}", fetch_data(Err("server2.com"), Security::NotFound));

//     // Panics with no custom message
//     // fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

//     // Panics with the message "ERROR: program stops"
//     // fetch_data(Err("server.com"), Security::Message);

//     // Panics with the message "malicious_server.com"
//     // fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
// }