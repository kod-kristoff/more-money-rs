use persi_ds::{
    unsync::list::List,
    unsynced_list,
};
use std::rc::Rc;

pub fn monads() {
    println!(">>> Bartosz monads <<<");

    assert_eq!(List::mreturn(5), unsynced_list![5]);

    let l1 = unsynced_list![3, 2, 1];
    //let l2 = l1.mbind(|x| List::from_value(x));
    println!("l2 = {:?}", l1);
}


// Monad

pub trait Monad where Self: Sized {
    type Unit;

    fn mreturn(unit: Self::Unit) -> Self;

    // fn mbind<F, M>(self, k: F) -> M;
}

// List
impl<A> Monad for List<A> {
    type Unit = A;

    fn mreturn(a: A) -> Self {
        Self::from_value(a)
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

impl<'a, State: 'a, A: 'a> Plan<'a,State, A> {
    fn new(f: &'a dyn Fn(State)->(A, State)) -> Self {
        log::trace!("called Plan::new");
        Self(Rc::new(f))
    }

    fn from_rc(f: Rc<dyn Fn(State) -> (A, State)>) -> Self {
        Self(f)
    }
}

impl<'a, State: 'a, A: Copy + 'a> Plan<'a,State, A> {
    // type Unit = Box<dyn Fn(State) -> Pair<A, State> + 'a>;
    // type Unit = A;

    fn mreturn(a: A) -> Self {
        log::trace!("called Plan::of");
        Self(Rc::new(move |s: State| (a, s)))
    }

    fn unwrap(self) -> A {
        log::trace!("called Plan::unwrap");
        unimplemented!()
    }
}

// impl<'a, State: 'a, A: Copy +'a> Functor for Plan<'a,State, A> {}
impl<'a, State: 'a, A: Copy +'a> Plan<'a,State, A> {
    fn mbind<B, F>(self, k: F) -> Plan<'a, State, B>
    where
        F: Fn(A) -> Plan<'a, State, B>
    {
        log::trace!("called Plan::chain");
        Plan::from_rc(Rc::new(move |s: State| {
            let (a, s1) = run_plan(self, s);
            let pl_b = k(a);
            run_plan(pl_b, s1)
        }))
    }
}
