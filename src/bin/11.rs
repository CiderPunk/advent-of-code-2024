use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
  let mut cache:StoneCache = HashMap::new();
  let stones:Vec<u64> = input.split_ascii_whitespace().map(|v| v.parse::<u64>().unwrap()).collect();
  Some(stones.iter().map(|stone| get_stone_count(25,*stone, &mut cache)).sum::<u64>())
}

type StoneCache = HashMap<(i32, u64), u64>;

pub fn part_two(input: &str) -> Option<u64> {

  let mut cache:StoneCache = HashMap::new();
  let stones:Vec<u64> = input.split_ascii_whitespace().map(|v| v.parse::<u64>().unwrap()).collect();
  Some(stones.iter().map(|stone| get_stone_count(75,*stone, &mut cache)).sum::<u64>())
}

fn get_stone_count(step: i32, stone: u64, cache: &mut StoneCache) -> u64 {
  if step == 0{
    return 1 as u64;
  }
  else{
    let v = cache.get(&(step,stone));
    match v{
      Some(result) => return *result,
      None=>{
        let result = get_next_stone(stone).iter().map(|next_stone| get_stone_count(step - 1, *next_stone, cache)).sum::<u64>();
        cache.insert((step,stone), result);
        return result;
      }
    }
  }
}

fn get_next_stone(stone:u64)->Vec<u64>{
  if stone == 0{
    return vec![1];
  }
  let stone_str = stone.to_string();
  if stone_str.len() % 2 == 0 {
    return vec![stone_str[0..stone_str.len() / 2].parse::<u64>().unwrap(),stone_str[stone_str.len() / 2..].parse::<u64>().unwrap()];
  }
  return vec![stone * 2024];
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
