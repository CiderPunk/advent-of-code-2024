use std::collections::HashSet;

use advent_of_code::Coord;


advent_of_code::solution!(6);


#[derive(Copy, Clone, Eq, Hash)]
pub struct Visited{
  pos:Coord,
  dir:Coord,
}

impl PartialEq for Visited {
  fn eq(&self, other: &Self) -> bool {
    self.pos == other.pos && self.dir == other.dir
  }
}



#[derive(Clone)]
pub struct Map{
  grid:Vec<Vec<bool>>,
  start:Coord,
  width:usize,
  height:usize,
}


pub fn inside_map(map:&Map, coord:Coord) -> bool{
  coord.x >= 0 && coord.y >= 0 && (coord.x as usize) < map.width && (coord.y as usize) < map.height
}

pub fn map_collision_test(map:&Map, pos:Coord) -> bool{
  map.grid[pos.y as usize][pos.x as usize]
}

pub fn map_collision_test_block(map:&Map, pos:Coord, block:Coord) -> bool{
  pos == block || map.grid[pos.y as usize][pos.x as usize]
}

/*
impl Map{
  pub fn inside(self, coord:Coord) -> bool{
    coord.x >= 0 && coord.y >= 0 && (coord.x as usize) < self.width && (coord.y as usize) < self.height
  }
  pub fn check_collision(self, pos:Coord) -> bool{
    self.grid[pos.y as usize][pos.x as usize]
  }
}
*/

fn parse_map(input: &str) -> Map{
  let mut start:Coord = Coord{ x:0, y:0};
  let grid: Vec<Vec<bool>> = input.lines().enumerate().map(|(y,line)| line.chars().enumerate().map(|(x, c)| {
    if c=='^'{
      start = Coord{ x:x as i32,y:y as i32};
    }
    c == '#'
  }).collect()).collect();
  let width = grid[0].len();
  let height = grid.len();
  Map {
    grid: grid,
    start: start,
    width: width,
    height: height,
  }
}


pub fn part_one(input: &str) -> Option<u32> {
  let map = parse_map(input);
  let mut dir = Coord{ 
    x:0,
    y:-1,
  };
  let mut exited = false;
  let mut current = map.start;
  let mut positions:HashSet<Coord> = HashSet::with_capacity(10000);
  positions.insert(current);

  while !exited{
    let next = current.add(dir);

    if !inside_map(&map, next){
      exited = true;
    }
    else{
      if map_collision_test(&map, next){
        dir.rotate_ccw();
      }
      else{
        positions.insert(next);
        current = next;
      }
    }
  }
  Some(positions.iter().count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
  let map = parse_map(input);
  let mut dir = Coord{ 
    x:0,
    y:-1,
  };
  let mut exited = false;
  let mut current = map.start;
  let mut block_list:HashSet<Coord> = HashSet::with_capacity(1000);
  let mut visited_list:HashSet<Coord> = HashSet::with_capacity(1000);
  while !exited{
    let next = current.add(dir);
    if !inside_map(&map, next){
      exited = true;
    }
    else{
      if map_collision_test(&map, next){
        dir.rotate_ccw();
      }
      else{
        //test block
        if next != map.start && !visited_list.contains(&next) && !block_list.contains(&next) && loop_check(&map, current, dir){
          block_list.insert(next);
        }
        visited_list.insert(current);
        current = next;
        
      }
    }
  }
  Some(block_list.iter().count() as u32)
}

fn loop_check(map: &Map, start: Coord, start_dir: Coord) -> bool {
  let block = start.add(start_dir);
  let mut current = start;
  let mut dir = start_dir;
  let mut visited:Vec<Visited> = vec![];
  loop{
    let next = current.add(dir);
    if !inside_map(&map, next){
      return false;
    }
    else{
      if map_collision_test_block(&map, next, block){
        let visit = Visited{pos: current, dir:dir};
        if visited.iter().any(|&v| v == visit){
          return true
        }
        else{
          visited.push(visit);
        }
        dir.rotate_ccw();
      }
      else{
        current = next;
      }
    }
  }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
