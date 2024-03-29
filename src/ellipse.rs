use rand::{Rng,rngs::ThreadRng};
use super::util::*;
use super::figure::Mutate;

#[derive(Debug,Clone)]
pub struct Ellipse{
    x_limit:u32,
    y_limit:u32,
    x:u32,
    y:u32,
    xr:u32,
    yr:u32,
}

impl Mutate for Ellipse{
    fn mutate(&mut self,rand:&mut ThreadRng){
        let dx=rand.gen_range(-16i32,17i32);
        let dy=rand.gen_range(-16i32,17i32);
        match rand.gen_range(0,3) {
            0 => {
                self.x=clip_add(self.x,dx,0,self.x_limit-1);
                self.y=clip_add(self.y,dy,0,self.y_limit-1);
            },
            1 => self.xr=clip_add(self.xr,dx,2,self.x_limit-1),
            2 => self.yr=clip_add(self.yr,dy,2,self.y_limit-1),
            _ => unreachable!()
        }
    }

    // fn set_figure(&mut self,other:&Self){
    //     self.x=other.x;
    //     self.y=other.y;
    //     self.xr=other.xr;
    //     self.yr=other.yr;
    // }

    //maybe do better?
    fn scanlines(&mut self) -> Vec<(u32,u32,u32)>{
        let c1=self.xr as f32/self.yr as f32;
        let c2=(self.yr*self.yr) as f32;
        let mut vec = Vec::new();
        for dj in 0..self.yr{
            let y1=self.y as i32 - dj as i32;
            let y2=self.y+dj;
            let x=((c2-(dj.pow(2) as f32)).sqrt()*c1) as i32;
            let mut x1=self.x as i32 - x;
            let mut x2=self.x + x as u32;
            if x1<0{
                x1=0;
            }
            if x2>=self.x_limit{
                x2=self.x_limit-1;
            }
            if y1>=0 && y1<self.y_limit as i32{
                vec.push((y1 as u32,x1 as u32,x2));
            }
            if y2<self.y_limit && dj>0{
                vec.push((y2,x1 as u32,x2));
            }
        }
        vec
    }

    // fn scanlines_increment(&mut self){
    //
    // }

}



impl Ellipse{
    pub fn new(x_limit:u32,y_limit:u32,rand:&mut ThreadRng) -> Ellipse{
        let x=rand.gen_range(0,x_limit);
        let xr=rand.gen_range(4,16);
        let y=rand.gen_range(0,y_limit);
        let yr=rand.gen_range(4,16);
        Ellipse{
            x_limit,
            y_limit,
            x,
            xr,
            y,
            yr,
        }
    }
}
