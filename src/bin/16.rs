use advent_of_code::Coord;

advent_of_code::solution!(16);


struct ScoreMap{
  score_map:Vec<u32>,
  width:i32,
  length:usize,
}




impl ScoreMap{


  fn coord_to_score(map:&ScoreMap, point:Coord, dir:Direction)->usize{
    let ix = (((point.y/2) * map.width) + (point.x / 2)) as usize; 
    match dir{
      Direction::North => ix,
      Direction::East => ix + map.length * 2,
      Direction::South => ix + map.length * 3,
      Direction::West => ix + map.length * 4,
    }
  }


  pub fn new(size:Coord) -> ScoreMap{
    ScoreMap{
      score_map: vec![0; ((size.x/2) * (size.y/2) * 4) as usize],
      length: ((size.x/2) * (size.y/2)) as usize,
      width: size.x/2,
    }
  }

  pub fn get_score(&self, point:Coord, dir:Direction) -> u32{
    self.score_map[ScoreMap::coord_to_score(self, point, dir)]
  }  
  pub fn set_score(&mut self, point:Coord, value:u32, dir:Direction) {
    self.score_map[ScoreMap::coord_to_score(self, point, dir)] =  value;
  }  
}  

struct Map{
  points:Vec<Vec<bool>>,
  start:Coord,
  end:Coord,
}

impl Map{
  pub fn get_point(&self, point:Coord) -> bool{
    self.points[point.y as usize][point.x as usize]
  }  
}  

enum Direction{
  North,
  East, 
  South,
  West,
}

fn parse_map(input: &str) -> Map{
  let mut start = Coord{x:0, y:0};
  let mut end = Coord{x:0, y:0};

  let points:Vec<Vec<bool>>  = input.lines().enumerate().map(|(y,line)| { 
    line.chars().enumerate().map(|(x,ch)| 
      match ch{
        'S' =>{ start = Coord{x:x as i32,y:y  as i32}; false }
        'E' =>{ end = Coord{ x:x as i32,y:y  as i32}; false }
        '.' =>{ false }
        _ => { true }
      }
    ).collect()
  }).collect();
  Map{ points: points, start:start, end:end } 
}

const FACINGS:[[(Direction,u32,Coord);3]; 4] = [ 
    [
      (Direction::North, 2, Coord{ x:0,y:-1}),
      (Direction::East, 1002, Coord{ x:1,y:0}),
      (Direction::West, 1002, Coord{ x:-1,y:0}),
    ],
    [
      (Direction::East, 2, Coord{ x:1,y:0}),
      (Direction::North, 1002, Coord{ x:0,y:-1}),
      (Direction::South, 1002, Coord{ x:0,y:1}),
    ],
    [
      (Direction::South, 2, Coord{ x:0,y:-1}),
      (Direction::East, 1002, Coord{ x:1,y:0}),
      (Direction::West, 1002, Coord{ x:-1,y:0}),
    ],
    [
      (Direction::East, 2, Coord{ x:-1,y:0}),
      (Direction::North, 1002, Coord{ x:0,y:-1}),
      (Direction::South, 1002, Coord{ x:0,y:1}),
    ],
 ];





fn find_best_route(map:&Map, scores:&mut ScoreMap, loc:Coord, facing:Direction, score:u32 )->u32{

  let current_score = scores.get_score(loc, facing);
  if score < current_score && current_score != 0 { return u32::max_value(); }
  scores.set_score(loc, score, facing);

  let tests = match facing {
    Direction::North => FACINGS[0],
    Direction::East => FACINGS[1],
    Direction::South => FACINGS[2],
    Direction::West => FACINGS[3], 
  };

  tests.iter().for_each(|(dir, score, coord)| {
    


  });

  if loc == map.end{
    return score
  }


  if (facing)

  0
}


pub fn part_one(input: &str) -> Option<u32> {
  let map = parse_map(input);
  let mut scores = ScoreMap::new(Coord{ x:map.points[0].len() as i32, y:map.points.len() as i32 });
  //let mut scores:Vec<u32> = vec![0; height/2 * width / 2];
  let mut loc = map.start;
  let mut facing = Direction::East;
  Some(find_best_route(&map, &mut scores, loc, facing, 0))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
