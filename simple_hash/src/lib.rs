use std::collections::HashMap;
pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    // let spl = words.split_ascii_whitespace(" ");
    let mut has : HashMap<&str, usize> = HashMap::new();
    let mut count= 0;
    for i in words {
        for j in words{
            if i==j{
                count+=1;
            }

        }
        has.insert(i,count);
        count = 0
    }
    has
      

}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
   let mut dd = 0;
   
    for i in frequency_count.keys(){
        dd+=1
    }
dd
}