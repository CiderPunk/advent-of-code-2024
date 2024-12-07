
advent_of_code::solution!(4);

fn get_char(grid: &Vec<Vec<char>>, x:i32, y:i32) -> char{
  if x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len(){
    'Z'
  }
  else{
    grid[x as usize][y as usize]
  }
}


pub fn part_one(input: &str) -> Option<u32> {
  let xmas: Vec<char> = "XMAS".chars().collect();
  let directions:Vec<(i32,i32)> = [(0,-1),(1,-1),(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1)].to_vec();
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut count:u32 = 0;
  grid.iter().enumerate().for_each(|(x, row)| {
    row.iter().enumerate().for_each(|(y, character)|{
      if *character == 'X'{
        directions.iter().for_each(|(right, up)| {
          if xmas.iter().enumerate().all(|(i,v)| {
            get_char(&grid, x as i32 + (right * i as i32), y as i32 +(up * i as i32)) == *v
          }){
            count += 1;
          }
        });
      }
    });
  });
  Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
  let diagonals:Vec<Vec<(i32,i32)>> =  [[(-1,-1),(1,1)].to_vec(), [(-1,1),(1,-1)].to_vec()].to_vec();
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut count:u32 = 0;
  grid.iter().enumerate().for_each(|(x, row)| {
    row.iter().enumerate().for_each(|(y, character)|{
      if *character == 'A'{
        if diagonals.iter().all(|d| { 
            let chars:Vec<char> = d.iter().map(|(up,left)| { get_char(&grid, x as i32 + left, y as i32 + up) } ).collect();
            chars.contains(&'M') && chars.contains(&'S')
          }
        ){
          count+=1;
        }
      }
    });
  });
  Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
