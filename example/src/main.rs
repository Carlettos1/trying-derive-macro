use rand::{rngs::ThreadRng, Rng};
use randomizable::Randomizable;
use randomizable_derive::Randomizable;

fn main() {
    println!("Hello, world!");
    let mut rng = ThreadRng::default();
    println!("{:?}", Cosas::random(&mut rng));
    println!("{:?}", Cosas::random(&mut rng));
    println!("{:?}", Cosas::random(&mut rng));
    println!("{:?}", Cosas::random(&mut rng));
    println!("{:?}", Cosas::random(&mut rng));
    println!("{:?}", Cosas::random(&mut rng));
    println!("{:?}", Cosas::random(&mut rng));
    println!("{:?}", Cosas::random(&mut rng));
    println!("{:?}", Cosas::random(&mut rng));
}

#[derive(Randomizable, Debug)]
pub enum Cosas {
    A,
    B,
    C,
    D,
}
