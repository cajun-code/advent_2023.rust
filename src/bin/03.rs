advent_of_code::solution!(3);

use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref NUMBER_REGEX: Regex = Regex::new("[0-9]{1,3}").unwrap();
    static ref SYMBLE_REGEX: Regex = Regex::new(r"[\?|#|\$|\*|@|%|!|\^|\+|=]").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut previous: Option<&str> = None;
    //let mut current: Option<&str> = None;
    let forward: Option<&str> = None;
    let mut total: u32 = 0;
    let mut current = lines.next();
    for line in lines{
        total += check_for_parts(previous, current, Some(line));
        previous = current;
        current = Some(line);
    }
    total += check_for_parts(previous, current, forward);
    Some(total)
}

fn check_for_parts(previous: Option<&str>, current: Option<&str>, forward:Option<&str>) -> u32 {
    let mut total:u32 = 0;
    for cap in NUMBER_REGEX.captures_iter(current.unwrap()){
        let part = cap.get(0).unwrap().as_str();
        let v:Vec<_> = current.unwrap().match_indices(part).collect();
        let mut pos = v.first().unwrap().0;
        pos = pos.saturating_sub(1);
        let mut len = v.first().unwrap().1.len()+2+pos;

        if len >= current.unwrap().len(){
            len = current.unwrap().len();
        }
        let mut is_previous  = false;

        let mut is_future = false;
        if previous.is_some(){
            let slice = &previous.unwrap()[pos .. len];
            println!("cutting string {} at {pos}, {len}", previous.unwrap());
            is_previous = SYMBLE_REGEX.is_match(slice.replace('.', " ").as_str());
            println!("Looking back at previous row: {slice} = {is_previous}");
        }
        let slice = &current.unwrap()[pos .. len];
        println!("cutting string {} at {pos}, {len}", current.unwrap());
        let is_current = SYMBLE_REGEX.is_match(slice.replace('.', " ").as_str());
        println!("Looking at current row: {slice} {is_current}");

        if forward.is_some() {
            let slice = &forward.unwrap()[pos .. len];
            println!("cutting string {} at {pos}, {len}", forward.unwrap());
            is_future =  SYMBLE_REGEX.is_match(slice.replace('.', " ").as_str());
            println!("Looking at forward row: {slice} {is_future}");
        }
        if is_current || is_previous || is_future {
            total += part.parse::<u32>().unwrap();
        }
        let is_part = is_previous || is_current || is_future;
        println!("part: {part} processed: {is_part} total: {total}");
    }
    
    total
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
