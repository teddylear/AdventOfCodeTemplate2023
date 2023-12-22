use std::collections::HashMap;

pub fn day02(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn is_valid_set(set: HashMap<String, usize>, cubes: HashMap<String, usize>) -> bool {
    for (set_key, set_value) in set.iter(){
        if !cubes.contains_key(set_key) {
            return false;
        }

        if set_value > cubes.get(set_key).unwrap() {
            return false;
        }
    }
    return true;
}

fn set_str_to_map(set_str: &str) -> HashMap<String, usize> {
    let mut game_set = HashMap::new();
    let mut index: usize;
    let mut num: usize;
    let mut color: String;

    for num_color_combo in set_str.split(", ") {
        index = 0;
        num = 0;
        color = "".to_string();
        for piece in num_color_combo.split(" ") {
            if index == 0 {
                num = piece.parse::<usize>().unwrap();
            } else {
                color = piece.to_string();
            }
            index = index + 1;
        }
        game_set.insert(color, num);
    }

    return game_set;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_set_from_string_1() {
        assert_eq!(set_str_to_map("3 blue, 4 red"), HashMap::from([
            ("blue".to_string(), 3),
            ("red".to_string(), 4),
        ]))
    }

    #[test]
    fn check_day01_set_from_string_2() {
        assert_eq!(set_str_to_map("3 blue, 4 red"), HashMap::from([
            ("red".to_string(), 4),
            ("blue".to_string(), 3),
        ]))
    }

    #[test]
    fn check_day01_is_valid_set_1() {
        assert!(is_valid_set(set_str_to_map("3 blue, 4 red"), set_str_to_map("4 blue, 5 red")))
    }

    #[test]
    fn check_day01_is_valid_set_2() {
        assert!(is_valid_set(set_str_to_map("3 blue, 4 red"), set_str_to_map("3 blue, 4 red")))
    }

    #[test]
    fn check_day01_is_valid_set_3() {
        assert!(!is_valid_set(set_str_to_map("3 blue, 4 red"), set_str_to_map("4 blue, 3 red")))
    }

    #[test]
    fn check_day01_is_valid_set_4() {
        assert!(!is_valid_set(set_str_to_map("3 blue, 4 red, 7 green"), set_str_to_map("4 blue, 3 red")))
    }

    // #[test]
    // fn check_day02_part1_case1() {
        // assert_eq!(day02("").0, "0".to_string())
    // }
    //

    // #[test]
    // fn check_day02_part2_case1() {
        // assert_eq!(day02("").1, "0".to_string())
    // }

    // #[test]
    // fn check_day02_both_case1() {
        // assert_eq!(day02(""), ("0".to_string(), "0".to_string()))
    // }
}
