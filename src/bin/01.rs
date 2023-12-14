

use lazy_static::lazy_static;
use regex::Regex;
advent_of_code::solution!(1);

lazy_static!{
    static ref NUMBER_WORDS: Regex = Regex::new("(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    static ref REVERSE_NUMERIC_WORDS: Regex = Regex::new("(1|2|3|4|5|6|7|8|9|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total:u32 = 0;
    //println!("{}", input);
    //let contents = fs::read_to_string(input).expect("Could not read input file");
    let lines = input.split('\n');
    for line in lines{
        let forward_numbers: Vec<&str> = line.matches(char::is_numeric).collect();
        let rev_numbers: Vec<&str> = line.rmatches(char::is_numeric).collect();
        if forward_numbers.is_empty() && rev_numbers.is_empty() {
            continue;
        }
        let value = format!("{}{}", forward_numbers[0], rev_numbers[0]);
        //println!("{line} = {:?}+{:?} = {value}", forward_numbers, rev_numbers);
        total += value.parse::<u32>().unwrap();
    }
    Some(total)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut total:u32 = 0;
    //println!("{}", input);
    //let contents = fs::read_to_string(input).expect("Could not read input file");
    let lines = input.split('\n');
    
    for line in lines{
        if line.is_empty(){
            continue;
        }
        let rev_line = line.chars().rev().collect::<String>();
        let forward_number = NUMBER_WORDS.find(line).unwrap().as_str();//line.matches(char::is_numeric).collect();
        let rev_number = REVERSE_NUMERIC_WORDS.find(rev_line.as_str()).unwrap().as_str();
        let back_number = rev_number.chars().rev().collect::<String>();
        let value = format!("{}{}", word_to_digit(forward_number), word_to_digit(back_number.as_str()));
        //println!("{line} = {:?}+{:?} = {value}", forward_number, back_number);
        total += value.parse::<u32>().unwrap();
    }
    Some(total)
}

fn word_to_digit(word:  &str) -> &str {
    match word.to_lowercase().as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven"=> "7",
        "eight"=> "8",
        "nine" => "9",
        _ => word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_part2", DAY));
        assert_eq!(result, Some(281));
    }
}
