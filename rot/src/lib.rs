pub fn rotate(input: &str, key: i8) -> String {
    let mut stro: String = String::new();

    for i in input.chars() {
        if i.is_ascii_alphabetic() {
            let key = key.rem_euclid(26);
            if i.is_uppercase() {
                let mut tt = i as i16 + key as i16;
                if tt > 90 {
                    let mut yy = tt - 90;
                    yy = yy % 26;
                    yy -= 1;
                    yy = 'A' as i16 + yy;
                    stro.push(yy as u8 as char);
                    continue;
                }
                stro.push(tt as u8 as char);
            } else {
                let mut tt = i as i16 + key as i16;
                if tt > 122 {
                    let mut yy = tt - 122;
                    yy = yy % 26;
                    yy -= 1;
                    yy = 'a' as i16 + yy;
                    stro.push(yy as u8 as char);
                    continue;
                }
                stro.push(tt as u8 as char);
            }
            continue;
        }
        stro.push(i);
    }

    stro
}
