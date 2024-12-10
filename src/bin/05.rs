

advent_of_code::solution!(5);

fn build_rules_updates(input: &str) -> ( Vec<(u32,u32)>, Vec<Vec<u32>> ){

  let mut rules: Vec<(u32,u32)> = vec![];
  let mut updates: Vec<Vec<u32>> = vec![];
  let mut rules_mode = true;
  input.lines().for_each(|line| {
    if line.len() == 0{
      rules_mode = false;
      return;
    }
    if rules_mode{
      let mut items = line.split("|");
      let left = items.next().unwrap().parse::<u32>().unwrap();
      let right = items.next().unwrap().parse::<u32>().unwrap();
      rules.push((left, right));
    }
    else{
      let items: Vec<u32> = line.split(",").map(|item|  item.parse::<u32>().unwrap()).collect();
      updates.push(items);
    }
  });
  (rules, updates)
}


pub fn part_one(input: &str) -> Option<u32> {
  let (rules, updates) = build_rules_updates(input);
  Some(updates.iter().map(|update| {
    if rules.iter().all(|(l,r)| {
      let left_pos = update.iter().position(|v| v == l);
      let right_pos = update.iter().position(|v| v == r); 
      left_pos == None || right_pos == None || left_pos < right_pos
    }) {
      update[update.len()/2]
    }
    else{
      0
    }
  }).sum())
}

struct Update{
  order:Vec<u32>,
  rules:Vec<(u32,u32)>,
}

fn fix_update(update:Update) -> u32{
  let mut fixed = false;
  let mut list = update.order.clone();
  while !fixed{
    fixed = update.rules.iter().all(|(l,r)| {
      let left_pos = list.iter().position(|v| v == l).unwrap();
      let right_pos = list.iter().position(|v| v == r).unwrap();
      if left_pos > right_pos{
        list.swap(left_pos, right_pos);
        return false
      }
      true
    });
  }
  list[list.len() / 2]
}

pub fn part_two(input: &str) -> Option<u32> {
  let (rules, lists) = build_rules_updates(input);
  //build list of updates and their related rules
  let mut updates:Vec<Update> = lists.iter().map(|list|
    Update{
      order:list.to_vec(),
      rules:rules.clone().into_iter().filter(|(l,r)| list.contains(l) && list.contains(r)).collect(),
    }
  ).collect();
  //remove any that are already ordered
  updates = updates.into_iter().filter(|u| u.rules.iter().any(|(l,r)| u.order.iter().position(|v| v == l) > u.order.iter().position(|v| v == r))).collect();
  //fix and add up everything
  Some( updates.into_iter().map(|u| fix_update(u)).sum())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
