use more_money::List;

fn select(lst: List<i32>) -> (i32, List<i32>) {
    let i = lst.front().unwrap();
    (*i, lst.popped_front())
}
fn main() {
    example_plan();
    example_identity();
    example_pair();
}
fn example_plan() {
    use more_money::state::{make_plan, mbind, mreturn, run_plan};
    let sel = make_plan(select);
    let pl = mbind(sel.clone(), move |i| {
        mbind(sel.clone(), move |j| mreturn((i, j)))
    });
    let st = List::cons(1, &List::cons(2, &List::from_value(3)));
    println!("example: {:?}", run_plan(pl, st));
}
fn example_identity() {
    println!("Identity!");

    let id1 = Identity::of(5);
    let id2: Identity<i32> = id1.map(|x| x * 3);
    let id3 = id2.chain(|x| Identity::of(x - 3));

    println!("{:?}", id1);
    println!("{:?}", id2);
    println!("{:?}", id3);
}
fn example_pair() {
    println!("Pair!");

    let p1 = Pair::of((1, 2));
    let p2: Pair<u32, u32> = p1.map(|(a, b)| (a * 2, b + 3));
    let p3 = p2.chain(|(a, b)| Pair::of((b, a)));

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
}

// Monad
//
pub trait Pointed
where
    Self: Sized,
{
    type Unit;

    fn of(unit: Self::Unit) -> Self;

    fn unwrap(self) -> Self::Unit;
}

pub trait Functor: Pointed {
    fn map<B, F>(self, f: F) -> B
    where
        B: Functor,
        F: Fn(Self::Unit) -> B::Unit,
    {
        B::of(f(self.unwrap()))
    }
}

pub trait Monad: Functor {
    fn chain<M, F>(self, f: F) -> M
    where
        M: Monad,
        F: Fn(Self::Unit) -> M,
    {
        f(self.unwrap())
    }
}

// Identity
#[derive(Copy, Clone, Debug)]
pub struct Identity<T>(T);

impl<T> Pointed for Identity<T> {
    type Unit = T;

    fn of(unit: Self::Unit) -> Self {
        Self(unit)
    }

    fn unwrap(self) -> Self::Unit {
        self.0
    }
}

impl<T> Functor for Identity<T> {}
impl<T> Monad for Identity<T> {}

// Pair
#[derive(Copy, Clone, Debug)]
pub struct Pair<A, B>(A, B);

impl<A, B> Pointed for Pair<A, B> {
    type Unit = (A, B);

    fn of(unit: Self::Unit) -> Self {
        Self(unit.0, unit.1)
    }

    fn unwrap(self) -> Self::Unit {
        (self.0, self.1)
    }
}

impl<A, B> Functor for Pair<A, B> {}
impl<A, B> Monad for Pair<A, B> {}
