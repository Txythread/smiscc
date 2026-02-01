/// ### Removes comments, leading and trailing whitespaces.
///
/// **For example:**
/// `"\t let a /* new value*/ = 3  "` => `"let a  = 3"`
///
/// To remove multi-line block comments, the argument in_block_comment needs to be set to a false value initially and then be re-used every time a new line is passed to the function.
pub fn trim(line: &str, in_block_comment: &mut bool) -> String {
    // In this function, the romanian flag is used for 
    // remains that should be removed in the end but make
    // the functions' syntax easier (to write, not to read :) )



    let mut output = String::new();
    let mut last_character = "🇷🇴".to_string();

    for character in line.chars(){
        if last_character == "/" && character == '/' {
            last_character = "🇷🇴".to_string();
            break;
        }

        if last_character == "/" && character == '*' {
            last_character = "🇷🇴".to_string();
            *in_block_comment = true;
        }else if last_character == "*" && character == '/' {
            *in_block_comment = false;
            last_character = "🇷🇴".to_string();
            continue;
        }

        if *in_block_comment {
            last_character = character.to_string();
            continue;
        }

        output += last_character.as_str();

        last_character = format!("{}", character);
    }

    if !*in_block_comment{
        output += last_character.as_str();
    }


    output = output.replace("🇷🇴", " ");


    output.trim().to_string()
}


#[cfg(test)]
mod tests {
    use crate::compiler::trimmer::trim;


    #[test]
    pub fn test_trim(){
        let lines = vec![
            "/*",
            "This is in a block comment",
            "And so is this!*/",
            "let a = 10 // This sets a to 10",
            "let b = a /* 10 */ + 3",
        ];

        let expected = vec![
            "",
            "",
            "",
            "let a = 10",
            "let b = a   + 3",
        ];

        let mut output: Vec<String> = vec![];


        let mut in_block_comment = false;

        for line in lines {
            output.push(trim(line, &mut in_block_comment));
        }

        assert_eq!(expected.len(), output.len());


        for i in 0..output.len() {
            assert_eq!(expected[i], output[i]);
        }
    }
}
