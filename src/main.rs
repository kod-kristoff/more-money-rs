use persi_ds::{
    unsync::list::List,
    unsynced_list
};
use std::rc::Rc;
mod bartosz;


fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .format_timestamp(None)
        .init();
    println!("Identity!");

    let id1 = Identity::of(5);
    let id2: Identity<i32> = id1.fmap(|x| x * 3);
    let id3 = id2.chain(|x| Identity::of(x - 3));

    println!("{:?}", id1);
    println!("{:?}", id2);
    println!("{:?}", id3);

    println!("Pair!");

    let p1 = Pair::of((1, 2));
    let p2: Pair<u32, u32> = p1.fmap(|(a, b)| (a*2, b+3));
    let p3 = p2.chain(|(a,b)| Pair::of((b, a)));

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);

    option_example();
    plan_example();
    // bartosz::monads();
}

fn option_example() {
    println!(">>> option example <<<");
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len: Option<usize> = maybe_some_string.fmap(|s| s.len());

    assert_eq!(maybe_some_len, Some(13));

    fn sq(x: u32) -> Option<u32> { Some(x * x) }
    fn nope(_: u32) -> Option<u32> { None }

    assert_eq!(Some(2).chain(sq).chain(sq), Some(16));
    assert_eq!(Some(2).and_then(sq).and_then(nope), None);
    assert_eq!(Some(2).and_then(nope).and_then(sq), None);
    assert_eq!(None.and_then(sq).and_then(sq), None);
}

fn plan_example() {
    println!(">>> plan example <<<");
    let st = unsynced_list!(1,2,3);
    let sel: Plan<List<i32>, i32> = Plan::new(&select);

//    println!(
//        "sel.chain: {:?}",
//        run_plan(
//            sel.chain(|i| i),
//            st.clone()
//        )
//    );
    println!(
        "plan_ex1: {:?}",
        run_plan(
            Plan::simple((2, 3)),
            st.clone()
        )
    );
    /*let pl: Plan<List<i32>, Pair<i32, i32>> = &sel
       .chain(|i| &sel
           .chain(|j| Plan::mreturn(Pair::of((i, j)))));*/
    //let result = run_plan(sel, st);
    //println!("result = {:?}", result);
//    assert_eq!(
//        result,
//        Pair::of((Pair::of((3,2)), unsynced_list![1]))
//    );
}

// Monad
//
pub trait Pointed where Self: Sized {
    type Unit;

    fn of(unit: Self::Unit) -> Self;
    fn mreturn(unit: Self::Unit) -> Self {
        log::trace!("called Pointed::mreturn");
        Self::of(unit)
    }

    fn unwrap(self) -> Self::Unit;
}

pub trait Functor: Pointed {
    fn fmap<B, F>(self, f: F) -> B
    where 
        B: Functor,
        F: Fn(Self::Unit) -> B::Unit
    {
        log::trace!("called Functor::fmap");
        B::of(f(self.unwrap()))
    }
}

pub trait Monad: Functor {
    fn chain<M, F>(self, f: F) -> M
    where
        M: Monad,
        F: Fn(Self::Unit) -> M
    {
        log::trace!("called Monad::chain");
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

impl<A: PartialEq, B: PartialEq> PartialEq for Pair<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
// Plan
pub struct Plan<'a, State, A>(Rc<dyn Fn(State) -> (A, State) + 'a>);

pub fn run_plan<'a, State, A>(
    pl: Plan<'a, State, A>,
    s: State
) -> (A, State) {
    log::trace!("called run_plan");
    pl.0(s)
}

impl<'a, State: 'a, A: Copy + 'a> Plan<'a,State, A> {
    fn new(f: &'a dyn Fn(State)->(A, State)) -> Self {
        log::trace!("called Plan::new");
        Self(Rc::new(f))
    }
    fn simple(a: A) -> Self {
        log::trace!("called Plan::of");
        Self(Rc::new(move |s: State| (a, s)))
    }

}

impl<'a, State: 'a, A: Copy + 'a> Pointed for Plan<'a,State, A> {
    type Unit = Rc<dyn Fn(State) -> (A, State) + 'a>;
    // type Unit = A;

    fn of(unit: Self::Unit) -> Self {
        log::trace!("called Plan::of");
        Self(unit)
    }

    fn unwrap(self) -> Self::Unit {
        log::trace!("called Plan::unwrap");
        self.0.clone()
    }
}

impl<'a, State: 'a, A: Copy +'a> Functor for Plan<'a,State, A> {}
impl<'a, State: 'a, A: Copy +'a> Monad for Plan<'a,State, A> {
//    fn chain<M, F>(self, k: F) -> M
//    where
//        M: Monad,
//        F: Fn(Self::Unit) -> M
//    {
//        log::trace!("called Plan::chain");
//        M::of(Rc::new(move |s: State| {
//            let (a, s1) = run_plan(self, s);
//            let pl_b = k(a);
//            run_plan(pl_b, s1)
//        }))
//    }
}

// select
pub fn select<A: Clone>(lst: List<A>) -> (A, List<A>) {
    match lst.front() {
        None => panic!("empty list"),
        Some(x) => {
            (x.clone(), lst.popped_front())
        }
    }
}

// Option
impl<T> Pointed for Option<T> {
    type Unit = T;

    fn of(unit: Self::Unit) -> Self {
        Some(unit)
    }

    fn unwrap(self) -> Self::Unit {
        Option::unwrap(self)
    }
}
impl<T> Functor for Option<T> {}
impl<T> Monad for Option<T> {}
