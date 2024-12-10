advent_of_code::solution!(7);

struct Equation{
  total:u64,
  components:Vec<u64>,
}


pub fn part_one(input: &str) -> Option<u64> {
  let calibration:Vec<Equation> = input.lines().map(|line| {
    let mut parts = line.split(':');
    let total:u64 = parts.next().unwrap().parse().unwrap();
    let components:Vec<u64> = parts.next().unwrap().split_ascii_whitespace().map(|comp| comp.parse::<u64>().unwrap()).collect();
    Equation{
      total: total,
      components: components,
    }
  }).collect();
  Some(calibration.iter().filter(|equation| is_valid_part1(&equation)).map(|equation| equation.total as u64).sum())
}

fn is_valid_part1(equation: &Equation) -> bool {
  equation.components.iter().rev().fold(vec![equation.total], |acc, value| {   
    let mut res:Vec<u64> = vec![];
    acc.iter().for_each(|v| { 

      if value > v {
        return;
      }
      else{
        //addition
        res.push(v-value);
        //multiplaction
        if v % value == 0{
          res.push(v/value);
        }
      }
    }); 
    res
  }).iter().any(|res| *res == 0)
}


fn is_valid_part2(equation: &Equation) -> bool {
  equation.components.iter().rev().fold(vec![equation.total], |acc, value| {   
    let mut res:Vec<u64> = vec![];
    acc.iter().for_each(|v| { 

      if value > v {
        return;
      }
      else{
        //addition
        res.push(v-value);
        //multiplaction
        if v % value == 0{
          res.push(v/value);
        }
        //concatenation
        let mut pow = 1;
        while (10 as u64).pow(pow) <= *value{
          pow+=1;
        } 
        let factor = (10 as u64).pow(pow);
        if v % factor == *value{
          let next = (v - value) / factor;
          //dbg!("w00p",next,v,value);
          res.push(next);
        }
      }
    }); 
    res
  }).iter().any(|res| *res == 0)
}

pub fn part_two(input: &str) -> Option<u64> {
  let calibration:Vec<Equation> = input.lines().map(|line| {
    let mut parts = line.split(':');
    let total:u64 = parts.next().unwrap().parse().unwrap();
    let components:Vec<u64> = parts.next().unwrap().split_ascii_whitespace().map(|comp| comp.parse::<u64>().unwrap()).collect();
    Equation{
      total: total,
      components: components,
    }
  }).collect();
  Some(calibration.iter().filter(|equation| is_valid_part2(&equation)).map(|equation| equation.total as u64).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
