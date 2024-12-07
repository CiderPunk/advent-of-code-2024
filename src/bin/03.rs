use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
  let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
  //let mut results = vec![];
  let mut result:u32 = 0;
  for (_, [left,right]) in re.captures_iter(input).map(|c| c.extract()) {
    result += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
    //results.push((left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()));
  }
  //Some( results.iter().map(|(l,r)| l * r ).sum())
  Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
  let re = Regex::new(r"(d)(o)\(\)|(d)(o)n't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
  let mut result:u32 = 0;
  let mut capture = true;
  for (command, [left,right]) in re.captures_iter(input).map(|c| c.extract()) {
    if command == "do()"{
      capture = true;
    }
    else if command == "don't()"{
      capture = false;
    }
    else{
      if capture{
        result += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
      }
    }
  }
  Some( result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
