use advent_of_code::CoordGeneric;
use regex::Regex;

advent_of_code::solution!(13);



#[derive(Copy, Clone, Hash, Debug)]
struct Machine{
  a:CoordGeneric<i64>,
  b:CoordGeneric<i64>,
  prize:CoordGeneric<i64>,
}

fn read_machines(input:&str) -> Vec<Machine>{
  let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\r?\n?Button B: X\+(\d+), Y\+(\d+)\r?\n?Prize: X=(\d+), Y=(\d+)").unwrap();
  let mut machines:Vec<Machine> = vec![];
  for (_, [a_x,a_y,b_x,b_y,p_x,p_y]) in re.captures_iter(input).map(|c| c.extract()) {
    machines.push(Machine{
      a:CoordGeneric::<i64>{ x: a_x.parse::<i64>().unwrap(), y:a_y.parse::<i64>().unwrap() },
      b:CoordGeneric::<i64>{ x: b_x.parse::<i64>().unwrap(), y:b_y.parse::<i64>().unwrap() },
      prize:CoordGeneric::<i64>{ x: p_x.parse::<i64>().unwrap(), y:p_y.parse::<i64>().unwrap() },
    })
  }
  machines
}



pub fn part_one(input: &str) -> Option<u64> {
  let machines = read_machines(input);
  //machines.iter().for_each(|machine| { dbg!(*machine);});
  //Some(machines.iter().map(|machine| find_cheapest_solution(machine)).sum())
  Some(machines.iter().map(|machine| calc_cheapest(machine)).sum())
}
/*
fn find_cheapest_solution(machine: &Machine) -> u32 {
  let mut current:CoordGeneric<i64> = machine.prize;
  let mut b_pushes = 0;
  let mut best = (0,0);
  while current.x > 0 && current.y > 0{
    if current.x % machine.a.x == 0 && current.y % machine.a.y == 0 && current.x / machine.a.x == current.y / machine.a.y{
      let a_pushes = current.x / machine.a.x;
      best = (a_pushes,b_pushes);
    }
    current = current.sub(machine.b);
    b_pushes+=1;
  }
  ((best.0 * 3) + best.1) as u32
}
 */
fn calc_cheapest(machine: &Machine) -> u64 {
  let top = (machine.prize.y * machine.a.x) - (machine.prize.x * machine.a.y);
  let bottom = (machine.b.y * machine.a.x) - (machine.b.x * machine.a.y);
  if top % bottom !=  0{  
    0 
  } else{
    let b = top / bottom;
    if (machine.prize.x - b * machine.b.x) % machine.a.x != 0{
      0
    }
    else{
      let a = (machine.prize.x - b * machine.b.x) / machine.a.x;
      (a * 3 + b) as u64
    }
  }
}



pub fn part_two(input: &str) -> Option<u64> {
  let machines = read_machines(input);
  //let correction = CoordGeneric::<i64>{ x:100000, y:100000};
  let correction = CoordGeneric::<i64>{ x:10000000000000, y:10000000000000};
  let corrected_machines:Vec<Machine> = machines.iter().map(|machine| Machine{ a: machine.a, b:machine.b, prize:machine.prize.add(correction)}).collect();
  //machines.iter().for_each(|machine| { dbg!(*machine);});
  Some(corrected_machines.iter().map(|machine| calc_cheapest(machine)).sum())
}

//76246593362537

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
