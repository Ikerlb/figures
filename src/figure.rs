use rand::rngs::ThreadRng;
use super::rectangle::Rectangle;
use super::ellipse::Ellipse;

pub trait Mutate {
    fn mutate(&mut self,rand:&mut ThreadRng);
    fn scanlines(&mut self) -> Vec<(u32,u32,u32)>;
}

#[derive(Debug,Clone)]
pub enum Figure{
    Rectangle(Rectangle),
    Ellipse(Ellipse),
}

impl Figure{
    pub fn mutate(&mut self,rand:&mut ThreadRng){
        match self{
            Figure::Rectangle(rect) => rect.mutate(rand),
            Figure::Ellipse(ellipse) => ellipse.mutate(rand),
        };
    }

    pub fn scanlines(&mut self) -> Vec<(u32,u32,u32)>{
        match self{
            Figure::Rectangle(rect) => rect.scanlines(),
            Figure::Ellipse(ellipse) => ellipse.scanlines(),
        }
    }


    pub fn new(mode:u32,w:u32,h:u32,rand:&mut ThreadRng)->Figure{
        match mode{
            1=>Figure::Rectangle(Rectangle::new(w,h,rand)),
            2=>Figure::Ellipse(Ellipse::new(w,h,rand)),
            _=>panic!("Unimplemented mode"),
        }
    }
}
