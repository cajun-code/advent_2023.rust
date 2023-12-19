advent_of_code::solution!(3);

use std::collections::{HashMap, HashSet};

pub struct Map{
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}
#[derive(Debug, PartialEq)]
pub enum EngineSymbolType{
    Number(u32),
    Empty,
    Symbol(char),
}

pub struct EngineSymbol{
    pub symbol_type: EngineSymbolType,
    pub id: usize,
}

pub fn get_engine_symbol_map(map: &Map) -> HashMap<(usize, usize), EngineSymbol>{
    let mut engine_symbols: HashMap<(usize, usize), EngineSymbol> = HashMap::new();
    let mut id = 0;

    for y in 0..map.height{
        let mut visited: HashSet<u32> = HashSet::new();
        for x in 0..map.width{
            if visited.contains(&(x as u32)){
                continue;
            }
            let c = map.map[y][x];
            if c.is_digit(10){
                let num: u32 = c.to_digit(10).unwrap();
                let mut x2 = x + 1;
                let mut digits: Vec<u32> = vec![num];
                while x2 < map.width {
                    let c2 = map.map[y][x2];
                    if c2.is_digit(10){
                        visited.insert(x2 as u32);
                        let num = c2.to_digit(10).unwrap();
                        digits.push(num);
                        x2 += 1;
                    }else{
                        break;
                    }
                }
                let num = digits.iter().fold(0, |acc, x| acc* 10 + x);
                for(i, _num) in digits.iter().enumerate(){
                    engine_symbols.insert((x+i, y), EngineSymbol{symbol_type: EngineSymbolType::Number(num), id,} );
                }
                engine_symbols.insert((x,y), EngineSymbol{
                    symbol_type: EngineSymbolType::Number(num),
                    id,
                });
            }else if c == '.'{
                engine_symbols.insert((x,y), EngineSymbol{
                    symbol_type: EngineSymbolType::Empty,
                    id,
                });
            }else {
                engine_symbols.insert((x,y), EngineSymbol{
                    symbol_type: EngineSymbolType::Symbol(c),
                    id,
                });
            }
            id += 1;
        }
    }
    engine_symbols
}
pub fn read_to_map (input: &str) -> Map{
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in input.lines(){
        let mut row: Vec<char> = Vec::new();
        for c in line.chars(){
            row.push(c);
        }
        width = row.len();
        map.push(row);
        height += 1;
    }
    Map{
        map, width, height,
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let map = read_to_map(input);

    let engine_symbols = get_engine_symbol_map(&map);

    let sum = sum_adjacent_numbers(&map, &engine_symbols);

    Some(sum)
}

fn sum_adjacent_numbers(map: &Map, map_lookup: &HashMap<(usize, usize), EngineSymbol>) -> u32 {
    let mut sum: u32 = 0;
    let mut visited_ids: HashSet<usize> = HashSet::new();
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (0,1),
        (1,0),
        (1,1),
        (-1,0),
        (0,-1),
        (-1,-1),
        (1,-1),
        (-1,1),
    ];
    for y in 0..map.height{
        for x in 0..map.width{
            let symbol = map_lookup.get(&(x,y)).unwrap();
            if let EngineSymbolType::Number(num) = symbol.symbol_type{
                if visited_ids.contains(&symbol.id){
                    continue;
                }
                let mut adjacent = false;
                for (x2, y2) in adjacent_neighbors.iter(){
                    let x2 = x2 + x as i32;
                    let y2 = y2 + y as i32;
                    if x2 <0 || y2 < 0 {
                        continue;
                    }
                    if x2 >map.width as i32 || y2 > map.height as i32{
                        continue;
                    }
                    if let Some(symbol) = map_lookup.get(&(x2 as usize, y2 as usize)){
                        if let EngineSymbolType::Symbol(c) = symbol.symbol_type{
                            println!("{num} at ({x},{y}) is adjacent to {c}");
                            adjacent = true;
                            break;
                        }
                    }
                }
                if adjacent {
                    sum += num;
                    visited_ids.insert(symbol.id);
                }
            }
        }
    }
    sum
}


pub fn part_two(input: &str) -> Option<u32> {
    let map = read_to_map(input);

    let engine_symbols = get_engine_symbol_map(&map);

    let sum:u32 = sum_gear_ratios(&map, &engine_symbols);

    Some(sum)
}

fn sum_gear_ratios(map: &Map, map_lookup: &HashMap<(usize, usize), EngineSymbol>) -> u32 {
    let mut gear_ratio_sum:u32 = 0;
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (0,1),
        (1,0),
        (1,1),
        (-1,0),
        (0,-1),
        (-1,-1),
        (1,-1),
        (-1,1),
    ];
    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(symbol) = map_lookup.get(&(x, y)) {
                if let EngineSymbolType::Symbol('*') = symbol.symbol_type {
                    let mut visited_ids: HashSet<usize> = HashSet::new();
                    let mut adjacent_numbers = 0;
                    let mut product = 1;
                    let mut ratio: Vec<usize> = Vec::new();
                    'next_neighbor: for (x2, y2) in adjacent_neighbors.iter() {
                        let x3 = x2 + x as i32;
                        let y3 = y2 + y as i32;
                        if x3 >= 0 && y3 >= 0 && x3 < map.width as i32 && y3 < map.height as i32 {
                            if let Some(adjacent_symbol) = map_lookup.get(&(x3 as usize, y3 as usize)) {
                                if visited_ids.contains(&adjacent_symbol.id) {
                                    continue 'next_neighbor;
                                }
                                if let EngineSymbolType::Number(num) = adjacent_symbol.symbol_type {
                                    adjacent_numbers += 1;
                                    ratio.push(num.clone() as usize);
                                    product *= num;
                                    visited_ids.insert(adjacent_symbol.id);

                                }
                            }
                        }
                    }
                    if adjacent_numbers == 2 {
                        gear_ratio_sum += product;
                        println!("{:?}", ratio)
                    }
                }
            }
        }
    }
    gear_ratio_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }
    // 527369
    #[test]
    fn test_part_one_input() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(512794));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
