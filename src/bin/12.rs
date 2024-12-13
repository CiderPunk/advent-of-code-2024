use advent_of_code::Coord;

advent_of_code::solution!(12);

struct Map{
  points:Vec<Vec<char>>,
  size:Coord,
}
#[derive(Copy, Clone, Hash, Debug)]
enum Direction{
  North,
  East,
  South,
  West,
}

const DIRECTIONS:[Coord;4] = [Coord{ x:1,y:0}, Coord{ x:-1,y:0},Coord{ x:0,y:1}, Coord{ x:0,y:-1}];
const DIRECTIONS2:[(Direction,Coord);4] = [(Direction::North,Coord{ x:0,y:-1}),(Direction::South,Coord{ x:0,y:1}),(Direction::East,Coord{ x:1,y:0}),(Direction::West,Coord{ x:-1,y:0}),];

//const DIRECTIONS2:[Directions;4] = [Directions::North(Coord{ x:0,y:1}),Directions::East(Coord{ x:1,y:0}),Directions::South(Coord{ x:0,y:1}),Directions::West(Coord{ x:-1,y:0})];

struct Edge{
  point:Coord,
  dir:Direction,
}


fn get_point(map:&Map, point:Coord)->char{
  map.points[point.y as usize][point.x as usize]
}

fn in_bounds(map:&Map, point:Coord)->bool{
  point.x >= 0 && point.y >=0 && point.x < map.size.x && point.y < map.size.y
}


fn build_map(input: &str) -> Map{
  let map:Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let size = Coord{ y: map.len() as i32, x: map[0].len() as i32};
  Map{ 
    points: map,
    size: size,
  }
}


fn expand(crop:char, point:Coord, map:&Map, done:&mut Vec<bool>)->(u64,u64){
  //dbg!(crop, point);
  //check for outside map
  if !in_bounds(map, point){
    return (0,1)
  }

  //check same crop
  if crop != get_point(map, point){
    return (0,1)
  }
  //avoid going in circles
  if done[((point.y * map.size.x) + point.x) as usize]{
    return (0,0)
  }
  //mark this point checked
  done[((point.y * map.size.x) + point.x) as usize] = true;
  //iterate through ordinals recursively
  DIRECTIONS.iter()
    .map(|dir| expand(crop, point.add(*dir), map, done))
    .fold((1,0), |acc,item| {
      (acc.0+item.0,acc.1+item.1)})
}

pub fn part_one(input: &str) -> Option<u64> {
  let map = build_map(input);

  let capacity = (map.size.x * map.size.y) as usize;
  let mut done:Vec<bool> = vec![false; capacity];
  
  Some(
    (0 .. map.size.y).map(|y| {
      (0 .. map.size.x).map(|x| {
        if done[((y * map.size.x) + x) as usize]{
          0 as u64
        }
        else{
          let start = Coord{x:x,y:y};
          let crop = get_point(&map, start);
          let (area, perimeter) = expand(crop, start, &map, &mut done);
          //dbg!(crop, area,  perimeter);
          area * perimeter
        }
      }).sum::<u64>()
    }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
  let map = build_map(input);

  let capacity = (map.size.x * map.size.y) as usize;
  let mut done:Vec<bool> = vec![false; capacity];
  
  Some(
    (0 .. map.size.y).map(|y| {
      (0 .. map.size.x).map(|x| {
        if done[((y * map.size.x) + x) as usize]{
          0 as u64
        }
        else{
          let start = Coord{x:x,y:y};
          let crop = get_point(&map, start);
          let mut fences:Vec<Edge> = vec![];
          let area = expand_fences(crop, Direction::North, start,start, &map, &mut done, &mut fences);
          let sides = count_sides(&mut fences);
          //dbg!(crop, area, sides);
          area * sides
        }
      }).sum::<u64>()
    }).sum())
}


fn count_sides(fences:&mut Vec<Edge>)->u64{
  let mut north_fences:Vec<Coord> = vec![];
  let mut south_fences:Vec<Coord> = vec![];
  let mut east_fences:Vec<Coord> = vec![];
  let mut west_fences:Vec<Coord> = vec![];
  fences.iter().for_each(|edge| {
    match edge.dir {
      Direction::North =>north_fences.push(edge.point),
      Direction::South =>south_fences.push(edge.point),
      Direction::East =>east_fences.push(edge.point),
      Direction::West =>west_fences.push(edge.point),
    }
  });
  count_vertical_fences(&mut east_fences) +  count_vertical_fences( &mut west_fences)  + count_horizontal_fences(&mut north_fences) + count_horizontal_fences(&mut south_fences)
}

fn count_vertical_fences(fences: &mut Vec<Coord>)->u64{
  let mut last:Coord = Coord{x:-1000, y:-1000};
  fences.sort_by(|a,b| ((a.x * 100000) + a.y).cmp(&((b.x * 100000) + b.y)));
  fences.iter().fold(0, |acc, c| { 
    if c.x == last.x && (c.y-last.y).abs() == 1 { 
      last = *c;
      acc 
    } else { 
      last = *c;
      acc + 1
    } 
  })
}

fn count_horizontal_fences(fences: &mut Vec<Coord>)->u64{
  let mut last:Coord = Coord{x:-1000, y:-1000};
  fences.sort_by(|a,b| ((a.y * 100000) + a.x).cmp(&((b.y * 100000) + b.x)));
  fences.iter().fold(0, |acc, c| { 
    if c.y == last.y && (c.x-last.x).abs() == 1 { 
      last = *c;
      acc 
    } else { 
      last = *c;
      acc + 1
    } 
  })
}


fn expand_fences(crop:char, dir:Direction, prev:Coord, point:Coord, map:&Map, done:&mut Vec<bool>, fences:&mut Vec<Edge>)->u64{
  //dbg!(crop, point);
  //check for outside map or same crop
  if !in_bounds(map, point) || crop != get_point(map, point) {
    fences.push(Edge{ dir:dir, point:prev});
    return 0;
  }

  //avoid going in circles
  if done[((point.y * map.size.x) + point.x) as usize]{
    return 0
  }
  //mark this point checked
  done[((point.y * map.size.x) + point.x) as usize] = true;
  //iterate through ordinals recursively
  DIRECTIONS2.iter()
    .map(|(dir, vector)| expand_fences(crop, *dir, point, point.add(*vector), map, done, fences))
    .sum::<u64>() + 1
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
