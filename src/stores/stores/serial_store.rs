pub use super::super::structs::*;
pub use super::store::Store;
//pub struct SerialStore{
//    list: Vec<Box<Color>>
//}
pub type SerialStore = Vec<Box<Color>>;
 impl Store for SerialStore{
     fn new() -> SerialStore {
        let mut v: Vec<Box<Color>> = Vec::new();
        v
    }

     fn add_pixel(&mut self, color: Box<Color>){
        self.push(color)
    }

     fn get_nearest(&mut self, color: Box<Color>) -> Option<Box<Color>>{
        self.pop()
    }

     fn size(&self) -> u32 {
        self.len() as u32
    }
}
