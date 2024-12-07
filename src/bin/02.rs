advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
  let reports = input.lines().map(|line| {
    let figs:Vec<i32>= line.split_ascii_whitespace().map(|val| val.parse::<i32>().unwrap()).collect();
    let second = &figs[1..];
    let diffs:Vec<i32> = figs.iter().zip(second.iter()).map(|(l,r)| (l-r)).collect();
    diffs.iter().all(|&v| v > 0 && v < 4) || diffs.iter().clone().all(|&v| v < 0 && v > -4)
  });
  //reports.clone().enumerate().for_each(|(i, r)| { dbg!(i, r);});
  Some(reports.filter(|v| *v == true ).count() as u32)  
}


pub fn part_two(input: &str) -> Option<u32> {
  let reports = input.lines().map(|line| {
    let figs:Vec<i32>= line.split_ascii_whitespace().map(|val| val.parse::<i32>().unwrap()).collect();
    figs.iter().enumerate().any(|(i,_)| {
      let mut subset = figs.clone();
      subset.remove(i);
      let second = &subset[1..];
      let diffs:Vec<i32> = subset.iter().zip(second.iter()).map(|(l,r)| (l-r)).collect();
      diffs.iter().all(|&v| v > 0 && v < 4) || diffs.iter().clone().all(|&v| v < 0 && v > -4)
    })
  });
  Some(reports.filter(|v| *v == true ).count() as u32)  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
