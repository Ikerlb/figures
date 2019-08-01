use image::{RgbaImage,Rgba};
use super::{imgutil,figure::Figure,minimize};

use rand::rngs::ThreadRng;

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

    pub fn step(&mut self,random_figure:u32,mode:u32){
        let figure=self.best_random_figure(random_figure,mode);
        let (mut figure,c)=minimize::hill_descent(self,100,figure);
        self.cost=c;
        let color=imgutil::compute_color(&self.target,&figure.scanlines(),self.alpha);
        imgutil::composite(&mut self.current,&figure.scanlines(),&color);
    }

    pub fn new_cost(&mut self,figure:&mut Figure) -> i32{
        let color=imgutil::compute_color(&self.target,&figure.scanlines(),self.alpha);
        imgutil::partial_cost(&self.target,&mut self.current,self.cost,&figure.scanlines(),&color)
    }

    fn best_random_figure(&mut self,m:u32,mode:u32)->Figure{
        let (w,h)=self.dimensions;
        let mut bc=self.cost;
        let mut bf=Figure::new(mode,w,h,&mut self.rng);
        for _ in 0..m{
            let mut f=Figure::new(mode,w,h,&mut self.rng);
            let color=imgutil::compute_color(&self.target,&f.scanlines(),self.alpha);
            let c=imgutil::partial_cost(&self.target,&mut self.current,self.cost,&f.scanlines(),&color);
            if c<bc{
                bc=c;
                bf=f.clone();
            }
        }
        bf
    }

}
