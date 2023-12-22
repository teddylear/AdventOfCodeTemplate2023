use std::collections::HashMap;

pub fn day02(input_lines: &str) -> (String, String) {
    let _ = input_lines;
    let answer1 = day02_part1(input_lines, HashMap::from([
        ("red".to_string(), 12),
        ("blue".to_string(), 14),
        ("green".to_string(), 13),
    ]));
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

fn day02_part1(input_lines: &str, cubes: HashMap<String, usize>) -> usize {
    let mut result = 0;
    for line in input_lines.split("\n") {
        result = result + value_per_game(line, cubes.clone())
    }
    return result;
}

fn value_per_game(line: &str, cubes: HashMap<String, usize>) -> usize {
    let mut game_num = 0;
    let mut index = 0;
    for piece in line.split(":") {
        if index == 0 {
            game_num = game_value_in_line(piece);
        } else {
            if !is_valid_game(piece, cubes.clone()) {
                return 0;
            }
        }
        index = index + 1;
    }
    return game_num;
}

fn game_value_in_line(game_num_text: &str) -> usize {
    let game_num_str = game_num_text.strip_prefix("Game ").unwrap();
    return game_num_str.parse::<usize>().unwrap();
}

fn is_valid_game(game_text: &str, cubes: HashMap<String, usize>) -> bool {
    for game_set in game_text.split(";") {
        if !is_valid_set(set_str_to_map(game_set), cubes.clone()) {
            return false;
        }
    }
    return true;
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
        println!("{}", num_color_combo);
        index = 0;
        num = 0;
        color = "".to_string();
        for piece in num_color_combo.split(" ") {
            println!("{}", piece);
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

    #[test]
    fn check_day01_get_game_number() {
        assert_eq!(game_value_in_line("Game 11"), 11)
    }
}
