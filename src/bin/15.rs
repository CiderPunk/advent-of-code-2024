use std::collections::HashSet;

use advent_of_code::Coord;

advent_of_code::solution!(15);

struct MapData{
  map:Map,
  start:Coord,
  instructions:Vec<Coord>,
}

struct Map{
  width:i32,
  points:Vec<char>,
}


fn parse_input2(input: &str)->MapData{
  let mut map_mode = true;
  let mut points:Vec<char> = vec![];
  let mut instructions:Vec<Coord> = vec![];
  let mut width = 0;
  let mut start = Coord{x:0,y:0};
  input.lines().enumerate().for_each(|(y,line)| {
    if line.len() == 0{
      map_mode = false;
    }
    else if map_mode {
      width = line.len() * 2;
      line.chars().enumerate().for_each(|(x, ch)| {
        match ch {
          '@'=> {start = Coord{ x:(x * 2) as i32, y:y as i32};
              points.push('@');
              points.push('.'); 
            }
          'O'=>{
            points.push('[');
            points.push(']'); 
          }
          _ => {
            points.push(ch);
            points.push(ch);
          }
        }
      });
    }
    else {
      line.chars().for_each(|dir|{
        match dir{
          '>' => instructions.push(Coord{x:1,y:0}),
          '<' => instructions.push(Coord{x:-1,y:0}),
          '^' => instructions.push(Coord{x:0,y:-1}),
          'v' => instructions.push(Coord{x:0,y:1}),
          _ =>{}
        }
      });
    }
  });
  MapData { 
    map:Map{
      width: width as i32,
      points: points,
    },
    start: start,
    instructions: instructions,
  }
}


fn parse_input(input: &str)->MapData{
  let mut map_mode = true;
  let mut points:Vec<char> = vec![];
  let mut instructions:Vec<Coord> = vec![];
  let mut width = 0;
  let mut start = Coord{x:0,y:0};
  input.lines().enumerate().for_each(|(y,line)| {
    if line.len() == 0{
      map_mode = false;
    }
    else if map_mode {
      width = line.len();
      line.chars().enumerate().for_each(|(x, ch)| {
        match ch {
          '@'=> {start = Coord{ x:x as i32, y:y as i32};
          points.push('@') }
          _ => points.push(ch),
        }
      });
    }
    else {
      line.chars().for_each(|dir|{
        match dir{
          '>' => instructions.push(Coord{x:1,y:0}),
          '<' => instructions.push(Coord{x:-1,y:0}),
          '^' => instructions.push(Coord{x:0,y:-1}),
          'v' => instructions.push(Coord{x:0,y:1}),
          _ =>{}
        }
      });
    }
  });
  MapData { 
    map:Map{
      width: width as i32,
      points: points,
    },
    start: start,
    instructions: instructions,
  }
}


fn swap_point(map:&mut Map, source:Coord, dest:Coord){
  let source_ix = ((source.y * map.width) + source.x) as usize;
  let dest_ix = ((dest.y * map.width) + dest.x) as usize;
  let temp = map.points[dest_ix];
  map.points[dest_ix] = map.points[source_ix];
  map.points[source_ix] = temp;
} 

fn get_point(map:&Map, coord:Coord) -> char{
  map.points[((coord.y * map.width) + coord.x) as usize]
}

fn try_move(map:&mut Map, point:Coord, dir:Coord)-> bool{
  let dest = point.add(dir);
  let dest_char = get_point(&map, dest);
  match  dest_char {
    '#' => false,
    'O' => 
        if try_move(map, dest, dir){
          swap_point(map, point, dest);
          true
        }
        else{
          false
        },   
    _ => {
      swap_point(map, point, dest);
      true  
    }
  }
}


fn do_move(map:&mut Map, move_list:&mut HashSet<Coord>, dir:Coord){
  let mut moves:Vec<Coord> = move_list.clone().into_iter().collect();
  if dir.x > 0{
    moves.sort_by(|a,b| b.x.cmp(&a.x));
  }
  else if dir.x < 0 {
    moves.sort_by(|a,b| a.x.cmp(&b.x));
  }
  if dir.y > 0{
    moves.sort_by(|a,b| b.y.cmp(&a.y));
  }
  else if dir.y < 0 {
    moves.sort_by(|a,b| a.y.cmp(&b.y));
  }
  moves.iter().for_each(|coord| {
    swap_point(map, *coord, coord.add(dir));
  });
  
}


fn can_move(map:&mut Map, move_list:&mut HashSet<Coord>, point:Coord, dir:Coord )-> bool{
  move_list.insert(point);
  let dest = point.add(dir);
  let dest_char = get_point(&map, dest);
  match  dest_char {
    '#' => false,
    'O' => { can_move(map, move_list, dest, dir )},
    '[' => {
      if dir.y == 0{
        //horizontal
        can_move(map, move_list, dest, dir )
      }
      else{
        can_move(map, move_list, dest, dir ) && can_move(map, move_list, dest.add(Coord{x:1,y:0}), dir )
      }}
      ,   
    ']' => {
        if dir.y == 0{
          //horizontal
          can_move(map, move_list, dest, dir )
        }
        else{
          can_move(map, move_list, dest, dir ) && can_move(map, move_list, dest.add(Coord{x:-1,y:0}), dir )
        }
      },
    _ => {
      true  
    }
  }
}



fn calc_gps(map:Map) -> u32{
  map.points.iter().enumerate().fold(0,|acc,(i,p)|{
    if *p == 'O' || *p =='['{
      let y = i as u32 / map.width as u32;
      let x = i as u32 % map.width as u32;
      acc + (100 * y) + x
    }
    else{
      acc    }
  })
}

fn dump_map(map:&Map){
  map.points.clone()[0..].chunks(map.width as usize).for_each(|line| {
    let s:String = String::from_iter(line);
    println!("{}",s);
  });
}

pub fn part_one(input: &str) -> Option<u32> {
  let mut map_data = parse_input(input);
  let mut robot = map_data.start;

  map_data.instructions.iter().for_each(|dir|{
    if try_move(&mut map_data.map, robot, *dir){
      robot = robot.add(*dir);
    }

    //dump_map(&map_data.map);
  });

  Some(calc_gps(map_data.map))
  //None
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut map_data = parse_input2(input);
  let mut robot = map_data.start;

  map_data.instructions.iter().for_each(|dir|{

    let mut move_list:HashSet<Coord> = HashSet::new();

    if can_move(&mut map_data.map, &mut move_list, robot, *dir){
      do_move(&mut map_data.map, &mut move_list, *dir);
      robot = robot.add(*dir);
    }

    //dump_map(&map_data.map);
  });
  //dump_map(&map_data.map);
  Some(calc_gps(map_data.map))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
