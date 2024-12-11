advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
  let mut files:Vec<u32> = vec![];
  let mut gaps:Vec<u32> = vec![];
  input.chars().enumerate().for_each(|(i,c)| {
    let size= c.to_digit(10 as u32).unwrap() as u32;
    if i % 2 == 0{
      files.push(size);
    }  
    else{ 
      gaps.push(size);
    }
  });
  let mut total:u64 = 0;
  let mut pos:usize = 0;
  //let files_iter = files.iter_mut().enumerate();

  let mut spaces = gaps.iter();
  let mut head = files.clone().into_iter().enumerate();
  let mut tail = files.clone().into_iter().enumerate().rev();
  let mut tail_file_num:usize = 100000;
  let mut tail_file_length:u32 = 0;
//let mut tail = files.iter().rev();

  //let mut done = false;
  loop{
    //dbg!("file");
    let (file_num, v) = head.next().unwrap(); 
    if file_num == tail_file_num{
      break;
    }
    //dbg!(v);
    for _ in 0 .. v{
      total += (pos * file_num) as u64; 
      //dbg!(total, pos, file_num);
      pos += 1;  
    }
    let gap = spaces.next();
    //dbg!("gap");
    for _ in 0..*gap.unwrap(){
      if tail_file_length == 0{
        let (file_num, t) = tail.next().unwrap();
        tail_file_num = file_num;
        tail_file_length = t as u32;
      }
      total += (tail_file_num * pos) as u64;
      //dbg!(total, pos, tail_file_num);
      pos+=1;
      tail_file_length -= 1;
    }
  }
  for _ in 0 .. tail_file_length{
    total += (tail_file_num * pos) as u64;
    //dbg!(total, pos, tail_file_num);
    pos+=1;
  }

  Some(total)
}

struct File{
  number:u32,
  position:u32,
  size:u32,
}

struct Gap{
  position:u32,
  size:u32,
}

pub fn part_two(input: &str) -> Option<u64> {
  let mut files:Vec<File> = vec![];
  let mut gaps:Vec<Gap> = vec![];
  let mut pos:u32 = 0;
  //build lists of gaps and files
  input.chars().enumerate().for_each(|(i,c)| {
    let size= c.to_digit(10 as u32).unwrap() as u32;
    if i % 2 == 0{
      files.push(File{ number:i as u32/2, position:pos, size: size,});
    }  
    else{ 
      gaps.push(Gap{ position:pos, size: size,});
    }
    pos += size;
  });

  //re-order everything with the gaps
  files.iter_mut().rev().for_each(|file| {
    let target = gaps.iter_mut().find(|gap| gap.position < file.position && gap.size >= file.size );
    if target.is_some(){
      let t = target.unwrap();
      file.position = t.position;
      t.position+= file.size;
      t.size -= file.size;
    }
  });

  //calculate those checksums

  Some(
    files.into_iter().map(|file| 
      (0..file.size).map(|i| file.number * (file.position + i)).sum::<u32>() as u64
    ).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
