use num::{Num, Signed};


pub mod template;

// Use this file to add helper functions and additional modules.


#[derive(Copy, Clone, Eq, Hash, Debug)]
pub struct CoordGeneric<T:Num>{
  pub x:T,
  pub y:T,
}

impl<T:Num> PartialEq for CoordGeneric<T> {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl<T:Num+Copy> CoordGeneric<T>{
  pub fn add(&self, coord:CoordGeneric<T>) -> CoordGeneric<T>{
    CoordGeneric{
      x: self.x + coord.x,
      y: self.y + coord.y,
    }
  }  
  pub fn add_scale(&self, coord:CoordGeneric<T>, scale:T) -> CoordGeneric<T>{
    CoordGeneric{
      x: self.x + coord.x * scale,
      y: self.y + coord.y * scale,
    }
  }  
  pub fn sub(&self, coord:CoordGeneric<T>) -> CoordGeneric<T>{
    CoordGeneric{
      x: self.x - coord.x,
      y: self.y - coord.y,
    }
  }


}  


impl<T:Num+Copy+Signed> CoordGeneric<T>{

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
  pub fn mod_components(&self, mod_target:CoordGeneric<T>) -> CoordGeneric<T>{
    CoordGeneric{
      x: self.x % mod_target.x,
      y: self.y % mod_target.y,
    }
  }


}

pub type Coord = CoordGeneric<i32>;
