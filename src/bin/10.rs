use std::collections::HashSet;

use advent_of_code::Coord;

advent_of_code::solution!(10);

struct Map{
  points:Vec<Vec<u8>>,
  size:Coord,
  starts:Vec<Coord>
}

fn build_map(input: &str) -> Map{
  let mut starts:Vec<Coord> = vec![];
  let map:Vec<Vec<u8>> = input.lines().enumerate().map(|(y,line)| line.chars().enumerate().map(|(x,c)| {
      let val = c.to_digit(10 ).unwrap();
      if val == 0{
        starts.push(Coord{x:x as i32, y:y as i32});
      }
      val as u8
    }).collect()).collect();
  let size = Coord{ y: map.len() as i32, x: map[0].len() as i32};
  Map{ 
    starts: starts,
    points: map,
    size: size,
  }
}


fn get_point(map:&Map, point:Coord)->u8{
  if point.x >= 0 && point.y >=0 && point.x < map.size.x && point.y < map.size.y{
    map.points[point.y as usize][point.x as usize]
  }
  else{
    255
  }
}


pub fn part_one(input: &str) -> Option<u32> {
  let directions:Vec<Coord> = vec![Coord{ x:1,y:0}, Coord{ x:-1,y:0},Coord{ x:0,y:1}, Coord{ x:0,y:-1}];
  let map = build_map(input);
  Some(map.starts.iter().map(|start| {
    let start_rating = (1..10).fold(HashSet::from([*start]), |acc, step|{
      let mut next:HashSet<Coord> = HashSet::new();
      acc.iter().for_each(|point| {
        directions.iter().for_each(|dir| {
          let test_point = point.add(*dir);
          if get_point(&map,test_point) == step{
            next.insert(test_point);
          }
        });
      });
      //dbg!(step,next.clone());
    next
  }).len() as u32;
  //dbg!(start_rating);
  start_rating
}).sum() )
}

pub fn part_two(input: &str) -> Option<u32> {
  let directions:Vec<Coord> = vec![Coord{ x:1,y:0}, Coord{ x:-1,y:0},Coord{ x:0,y:1}, Coord{ x:0,y:-1}];
  let map = build_map(input);
  Some(map.starts.iter().map(|start| {
    let start_rating = (1..10).fold(vec![*start], |acc, step|{
      let mut next:Vec<Coord> = vec![];
      acc.iter().for_each(|point| {
        directions.iter().for_each(|dir| {
          let test_point = point.add(*dir);
          if get_point(&map,test_point) == step{
            next.push(test_point);
          }
        });
      });
        
      next
    }).len() as u32;
    //dbg!(start_rating);
    start_rating
  }).sum() )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
