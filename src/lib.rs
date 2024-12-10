pub mod template;

// Use this file to add helper functions and additional modules.


#[derive(Copy, Clone, Eq, Hash, Debug)]
pub struct Coord{
  pub x:i32,
  pub y:i32,
}

impl PartialEq for Coord {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl Coord{
  pub fn add(&self, coord:Coord) -> Coord{
    Coord{
      x: self.x + coord.x,
      y: self.y + coord.y,
    }
  }  
  pub fn add_scale(&self, coord:Coord, scale:i32) -> Coord{
    Coord{
      x: self.x + coord.x * scale,
      y: self.y + coord.y * scale,
    }
  }  


  pub fn sub(&self, coord:Coord) -> Coord{
    Coord{
      x: self.x - coord.x,
      y: self.y - coord.y,
    }
  }
  pub fn rotate_ccw(&mut self){
    let temp = self.x;
    self.x = -self.y;
    self.y = temp;
  }
  pub fn rotate_cw(&mut self){
    let temp = self.x;
    self.x = self.y;
    self.y = -temp;
  }
}
