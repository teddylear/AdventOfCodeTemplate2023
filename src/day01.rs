pub fn day01(input_lines: &str) -> (String, String) {
    let mut answer1 = 0;
    for line in input_lines.split("\n") {
        answer1 = answer1 + calc_line_amount(line)
    }
    let mut answer2 = 0;

    for line in input_lines.split("\n") {
        answer2 = answer2 + calc_line_amount_with_num(line)
    }

    (format!("{}", answer1), format!("{}", answer2))
}

fn calc_line_amount_with_num(line: &str) -> u32 {
    let mut first_char = 'a';
    let mut last_char = 'a';
    let mut temp_char;

    let mut index = 0;
    for c in line.chars() {
        if c == 'o' && is_match_num_word(&index, line, "one") {
           temp_char = get_number_from_word("one");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c == 't' && is_match_num_word(&index, line, "two") {
           temp_char = get_number_from_word("two");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c == 't' && is_match_num_word(&index, line, "three") {
           temp_char = get_number_from_word("three");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c == 'f' && is_match_num_word(&index, line, "four") {
           temp_char = get_number_from_word("four");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c == 'f' && is_match_num_word(&index, line, "five") {
           temp_char = get_number_from_word("five");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c == 's' && is_match_num_word(&index, line, "six") {
           temp_char = get_number_from_word("six");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c == 's' && is_match_num_word(&index, line, "seven") {
           temp_char = get_number_from_word("seven");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c == 'e' && is_match_num_word(&index, line, "eight") {
           temp_char = get_number_from_word("eight");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c == 'n' && is_match_num_word(&index, line, "nine") {
           temp_char = get_number_from_word("nine");
           if first_char == 'a' {
               first_char = temp_char;
           }
           last_char = temp_char;
        } else if c.is_digit(10) {
           if first_char == 'a' {
               first_char = c;
           }
           last_char = c;
        }

        index = index + 1;
    }

    let result_string = format!("{first_char}{last_char}");
    if result_string == "aa" {
        return 0;
    }
    return result_string.parse::<u32>().unwrap();
}

fn is_match_num_word(index: &usize, line: &str, word: &str) -> bool {
    let temp_line = line;
    if !temp_line.contains(word) {
        return false;
    }

    let first_option = temp_line.find(word).unwrap();
    if first_option == *index {
        return true;
    }

    let last_option = temp_line.rfind(word).unwrap();
    if last_option == *index {
        return true;
    }

    return false;
}

fn get_number_from_word(str_num: &str) -> char {
    match str_num {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '0',
    }
}

fn calc_line_amount(line: &str) -> u32 {
    let mut first_char = 'a';
    let mut last_char = 'a';

    for c in line.chars() {
       if c.is_digit(10) {
           if first_char == 'a' {
               first_char = c;
           }
           last_char = c;
       }
    }

    let result_string = format!("{first_char}{last_char}");
    if result_string == "aa" {
        return 0;
    }
    return result_string.parse::<u32>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(calc_line_amount_with_num("9eightone"), 91)
    }

    #[test]
    fn check_day01_part2_case2() {
        assert_eq!(calc_line_amount_with_num("9twopjqkghmbone"), 91)
    }

    #[test]
    fn check_day01_part2_case3() {
        assert_eq!(calc_line_amount_with_num("hczsqfour3nxm5seven4"), 44)
    }

    #[test]
    fn check_day01_part2_case4() {
        assert_eq!(calc_line_amount_with_num("hczsqfour3fourseven4"), 44)
    }
}
