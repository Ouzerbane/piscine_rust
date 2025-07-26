pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut vec : Vec <String> = Vec::new();
    for i in names {
        let mut st : String = String::from(i.chars().next().expect("walo"));
        let mut index = i.find(" ").expect("ta hiya walo almoxkil");
        st.push('.');
        st.push_str(&i[index..index+2].to_string());
        st.push('.');
        vec.push(st);
    }
    vec
}