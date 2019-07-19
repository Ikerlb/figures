use image::{RgbaImage,Rgba};
use super::{imgutil,rectangle::Rectangle};
use super::figure::Figure;
use rand::rngs::ThreadRng;
use super::minimize;

#[derive(Debug)]
pub struct State{
    pub current: RgbaImage,
    target: RgbaImage,
    alpha: u8,
    dimensions: (u32,u32),
    pub cost:i32,
    pub rng:ThreadRng,
}

impl State{
    pub fn new(target:RgbaImage,alpha:u8)->State{
        let dimensions=target.dimensions();
        let (w,h)=dimensions;
        let (r,g,b)=imgutil::average_color(&target);
        let current=RgbaImage::from_fn(w,h,|_,_|{Rgba{data:[r,g,b,255]}});
        let cost=imgutil::full_cost(&target,&current);
        let rng=rand::thread_rng();
        let st= State{
            current,
            target,
            alpha,
            dimensions,
            cost,
            rng,
        };

        return st;
    }

    pub fn step(&mut self){
        let (w,h)=self.dimensions;
        let mut figure=Rectangle::new(w,h,&mut self.rng);
        self.cost=minimize::hill_descent(self,100,&mut figure);
        let color=imgutil::compute_color(&self.target,&figure.scanlines(),self.alpha);
        imgutil::composite(&mut self.current,&figure.scanlines(),&color);
    }

    pub fn new_cost<T:Figure>(&mut self,figure:&mut T) -> i32{
        let color=imgutil::compute_color(&self.target,&figure.scanlines(),self.alpha);
        imgutil::partial_cost(&self.target,&mut self.current,self.cost,&figure.scanlines(),&color)
    }

}
