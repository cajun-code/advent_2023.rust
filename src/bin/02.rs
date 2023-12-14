use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
advent_of_code::solution!(2);

lazy_static!{
    static ref NUMBER_REGEX: Regex = Regex::new("[0-9]{1,3}").unwrap();
    static ref COLOR_REGEX: Regex = Regex::new("(blue|red|green)").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    //println!("{}", input);
    let mut total: u32 = 0;
    let required: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let lines = input.split('\n');
    for line in lines{
        if line.is_empty(){
            continue;
        }
        let mut games = line.split(':');
        let id = games.next().unwrap();
        let game_id = NUMBER_REGEX.find(id).unwrap().as_str();
        let game = games.next().unwrap();
        let bags = game.split(';');
        let mut playable: bool = true;
        for bag in bags{
            if !playable{
                continue;
            }
            let mut cubes:HashMap<&str, u32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            for dice in bag.split(','){
                let number = NUMBER_REGEX.find(dice).unwrap().as_str();
                let color  = COLOR_REGEX.find(dice).unwrap().as_str();
                cubes.insert(color, number.parse::<u32>().unwrap());
            }
            //println!("{:?} required {:?}", cubes, required);
            playable = cubes["blue"] <= required["blue"]
                && cubes["red"] <= required["red"]
                && cubes["green"] <= required["green"];

        }
        if playable{
            //println!("adding game Id: {game_id} to total: {total}");
            total += game_id.parse::<u32>().unwrap();
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let lines = input.split('\n');
    for line in lines{
        if line.is_empty(){
            continue;
        }
        let mut games = line.split(':');
        let _id = games.next().unwrap();
        let game = games.next().unwrap();
        let bags = game.split(';');
        let mut cubes:HashMap<&str, u32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for bag in bags{
            for dice in bag.split(','){
                let number = NUMBER_REGEX.find(dice).unwrap().as_str().parse::<u32>().unwrap();
                let color  = COLOR_REGEX.find(dice).unwrap().as_str();
                if cubes[color] < number {
                    cubes.insert(color, number);
                }
            }
        }
        let power: u32 = cubes.values().product();
        total += power;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let expected: u32 = 8;
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

}
