
advent_of_code::solution!(1);

fn build_lists(input: &str) -> (Vec<i32>, Vec<i32>){
  input.lines().map(|line| {
    let mut items = line.split_ascii_whitespace();
    let left = items.next().unwrap().parse::<i32>().unwrap();
    let right = items.next().unwrap().parse::<i32>().unwrap();
    (left,right)
  }).unzip()
}



pub fn part_one(input: &str) -> Option<i32> {

  let (mut left, mut right) = build_lists(input);
  left.sort();
  right.sort();
  Some(std::iter::zip(left,right)
      .map(|(l,r)| (l-r).abs())
      .sum() )
}

pub fn part_two(input: &str) -> Option<i32> {
  let (left, mut right) = build_lists(input);
  Some(left.into_iter().map(|val| {
    let count = right.iter().filter(|&v| *v == val).count() as i32;
    (count * val)
  }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
