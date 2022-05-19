use crate::helper_functions;

pub fn pig_latin_main() {
    helper_functions::flush_print("\n\tEnter any valid string : ");
    let input : String = helper_functions::read_line_self();
    let mut pig_latin_output = String::new();

    for i in input.trim().split_whitespace(){
        //Indexing bad idea, hence .chas() method 
        let word_char_iter : Vec<char> = i.chars().collect();
        if ['a','e','i','o','u','A','E','I','O','U'].contains(&word_char_iter[0]){
            let pig_latin_word = format!("{}-{} ",&i,"hay");
            pig_latin_output.push_str(&pig_latin_word);
        }else{
            let mut iter = i.chars();
            iter.next();//Skip first letter as it is not vowel.
            for j in iter{
                pig_latin_output.push(j);
            }
            let pig_latin_append = format!("-{}{} ",word_char_iter[0],"ay");
            pig_latin_output.push_str(&pig_latin_append)
        }
    }
    println!("\tGiven string in pig latin : {}",pig_latin_output);
    
}
