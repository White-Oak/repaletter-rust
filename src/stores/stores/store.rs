pub use super::super::structs::*;
pub trait Store{
     fn new() -> Self;
     fn add_pixel(&mut self, color: Box<Color>);

     fn get_nearest(&mut self, color: Box<Color>) -> Option<Box<Color>>;

     fn size(&self) -> u32;
}
