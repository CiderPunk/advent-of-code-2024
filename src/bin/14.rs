use advent_of_code::Coord;
use regex::Regex;

advent_of_code::solution!(14);


struct Robot{
  pos:Coord,
  vel:Coord,
}




fn parse_robots(input: &str)->Vec<Robot>{
  let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
  let mut robots:Vec<Robot> = vec![];
  for (_, [s_x,s_y,v_x,v_y]) in re.captures_iter(input).map(|c| c.extract()) {
    robots.push(Robot{
      pos:Coord{x: s_x.parse::<i32>().unwrap(),y:s_y.parse::<i32>().unwrap()},
      vel: Coord{x: v_x.parse::<i32>().unwrap(),y:v_y.parse::<i32>().unwrap()},
    })
  }
  robots
}

pub fn part_one(input: &str) -> Option<u32> {
  let size = Coord { x:101, y:103 };
  let time = 100;
  let robots = parse_robots(input);
  let mut sectors = [0,0,0,0];
  robots.iter().for_each(|robot| {
    let mut pos = robot.pos.add_scale(robot.vel, time).mod_components(size);
    if pos.x < 0 { pos.x += size.x;}
    if pos.y < 0 { pos.y += size.y;}
    let mut s = 0;
    //dbg!(pos);
    if pos.x == size.x / 2 || pos.y == size.y / 2{
      return;
    }

    if pos.x > (size.x / 2){ s+=1; }
    if pos.y > (size.y / 2){ s+=2; }
    sectors[s] += 1;
  });

  //dbg!(sectors);
  Some(sectors.iter().fold(1 as u32, |acc,v| acc*v))
}

pub fn part_two(input: &str) -> Option<u32> {
  let sector_size:i32 = 10;
  let size = Coord { x:101, y:103 };
  let mut robots = parse_robots(input);
  let sector_map_size = Coord{x:size.x / sector_size, y:size.y / sector_size };
  //dbg!(sector_map_size);
  let expected_average:i32 = robots.len() as i32 / (sector_map_size.x * sector_map_size.y);
  let mut count = 0;
  let mut tree_found = false;



  while  !tree_found{

    count+=1;
    let mut sectors:Vec<i32> = vec![0; (sector_map_size.x * sector_map_size.y) as usize ];
    robots.iter_mut().for_each(|robot| {
      robot.pos = robot.pos.add(robot.vel).mod_components(size);
      if robot.pos.x < 0 { robot.pos.x += size.x;}
      if robot.pos.y < 0 { robot.pos.y += size.y;}

      let sector = Coord{ x:robot.pos.x / sector_size, y: robot.pos.y / sector_size };
      if sector.x < sector_map_size.x && sector.y < sector_map_size.y{  
        //dbg!(robot.pos, sector);
        sectors[((robot.pos.y / sector_size) * (size.x / sector_size) + (robot.pos.x / sector_size)) as usize] +=1;
      }
    });

    let variance = sectors.iter().fold(0, |acc, v| {
      let diff = v - expected_average;
      acc + (diff * diff)  
    }) / (sector_map_size.x * sector_map_size.y);
    //dbg!(count, variance);
    if variance > 40{ tree_found = true;}
  }

  visualize(robots, size);

  Some(count)
}


fn visualize(robots:Vec<Robot>, size:Coord){
  let mut buffer:Vec<char> = vec![' '; (size.x * size.y) as usize];
  robots.iter().for_each(|robot| { buffer[((robot.pos.y * size.x) + robot.pos.x) as usize] = '#'; });

  buffer.chunks(size.x as usize).for_each(|row| {
    let s:String = String::from_iter(row);
    println!("{}",s);
  });
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
