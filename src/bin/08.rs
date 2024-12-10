use std::collections::{HashMap, HashSet};

use advent_of_code::Coord;

advent_of_code::solution!(8);

struct AntennaMap{
  size:Coord,
  antenna:HashMap<char, Vec<Coord>>,
  //antenna:Vec<AntennaCollection>
}


fn build_map(input: &str) -> AntennaMap{
  let mut antenna: HashMap<char, Vec<Coord>> = HashMap::new();

  let lines:Vec<&str> = input.lines().collect();

  lines.iter().enumerate().for_each(|(y, line)| {
    line.chars().enumerate().for_each(|(x, cr)| {
      if cr != '.'{
        let frequency_collection = antenna.get_mut(&cr);
        if frequency_collection == None{
          antenna.insert(cr, vec![ Coord{x:x as i32,y:y as i32} ]);
        }
        else{
          frequency_collection.unwrap().push(Coord{x:x as i32,y:y as i32});
        }
      }
    });
  });

  AntennaMap{
    size: Coord{x:lines[0].len() as i32, y:lines.len() as i32},
    antenna:antenna,
  }
}

fn inside_map(map:&AntennaMap, pos:Coord)->bool{
  pos.x >= 0 && pos.y >=0 && pos.x < map.size.x && pos.y < map.size.y
}

pub fn part_one(input: &str) -> Option<u32> {
  let map = build_map(input);
  let mut result:HashSet<Coord> = HashSet::with_capacity(map.antenna.len() * 2);
  map.antenna.iter().for_each(|(_,list)|{

    //dbg!(list);
    list.iter().for_each(|current|{  
      list.iter().for_each(|compare|{

        if current != compare {
          let diff = current.sub(*compare); 
          let antinode = current.add(diff);
          if inside_map(&map, antinode){
            result.insert(antinode);

          }
        }
      });
    });
  });
  //dbg!(&result);
  Some(result.iter().len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
  let map = build_map(input);
  let mut result:HashSet<Coord> = HashSet::with_capacity(map.antenna.len() * 10);
  map.antenna.iter().for_each(|(_,list)|{
    //dbg!(list);
    list.iter().for_each(|current|{  
      list.iter().for_each(|compare|{
        if current == compare {
          return;
        }
        let diff = current.sub(*compare); 
        let mut count = 0; 
        while inside_map(&map, current.add_scale(diff, count)){
          let antinode = current.add_scale(diff, count);
          result.insert(antinode);
          count +=1;
        }
      
      });
    });
  });
  //dbg!(&result);
  Some(result.iter().len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
