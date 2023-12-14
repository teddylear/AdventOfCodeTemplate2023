pub fn day01(input_lines: &str) -> (String, String) {
    let mut answer1 = 0;
    for line in input_lines.split("\n") {
        answer1 = answer1 + calc_line_amount(line)
    }
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
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
    println!("result_string {}", result_string);
    if result_string == "aa" {
        return 0;
    }
    return result_string.parse::<u32>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(day01("").0, "0".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(day01("").1, "0".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(day01(""), ("0".to_string(), "0".to_string()))
    }
}
