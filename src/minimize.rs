use super::state::State;
use super::figure::Figure;


pub fn hill_descent<T:Figure+Clone>(state:&mut State,age:usize,figure:&mut T) -> i32{
    let mut bc=state.cost;
    for _ in 0..age{
        let prev=figure.clone();
        figure.mutate(&mut state.rng);
        let c=state.new_cost(figure);
        if c>=bc{
            figure.set_figure(&prev);
        } else{
            bc=c;
        }
    }
    bc
}
