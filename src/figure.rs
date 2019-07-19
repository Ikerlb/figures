use rand::rngs::ThreadRng;

pub trait Figure {
    fn mutate(&mut self,rand:&mut ThreadRng);
    fn scanlines(&mut self) -> Vec<(u32,u32,u32)>;
    fn set_figure(&mut self,other:&Self);
}
