use rand::Rng;

pub trait Randomizable {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self;
}

impl Randomizable for f64 {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        rng.gen()
    }
}

macro_rules! randomize_tuple {
    ($($t:tt)+) => {
        impl<$($t: Randomizable,)+> Randomizable for ($($t,)+) {
            fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
                ( $( <$t>::random(rng), )+ )
            }
        }
    };
}

randomize_tuple! {A B}
randomize_tuple! {A B C}
randomize_tuple! {A B C E}
randomize_tuple! {A B C E F}
randomize_tuple! {A B C E F G}
randomize_tuple! {A B C E F G H}
randomize_tuple! {A B C E F G H I}
