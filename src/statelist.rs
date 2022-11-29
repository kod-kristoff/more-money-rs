use crate::List;
use persi_ds::sync::list::{concat_all, fmap};
use std::rc::Rc;

pub type State = List<i32>;

pub type PairList<A> = List<(A, State)>;

pub type StateList<A> = Rc<dyn Fn(State) -> PairList<A>>;

pub fn run_state_list<A>(st: StateList<A>, s: State) -> PairList<A> {
    st(s)
}

pub fn make_state_list<A, F>(f: F) -> StateList<A>
where
    F: Fn(State) -> PairList<A> + 'static,
{
    Rc::new(f)
}

pub fn mbind<A, B, F>(g: StateList<A>, k: F) -> StateList<B>
where
    F: Fn(A) -> StateList<B> + 'static,
    B: Clone,
    A: Clone + 'static,
{
    Rc::new(move |s: State| {
        let plst = g(s);
        let lst2 = fmap(
            |p: &(A, State)| {
                let a = p.0.clone();
                let s1 = p.1.clone();
                let ka = k(a);
                let result = run_state_list(ka, s1);
                result
            },
            &plst,
        );
        concat_all(&lst2)
    })
}

pub fn mreturn<A: Clone + 'static>(a: A) -> StateList<A> {
    Rc::new(move |s: State| PairList::from_value((a.clone(), s)))
}

pub fn mzero<A>() -> StateList<A> {
    Rc::new(|_s: State| PairList::<A>::new())
}

pub fn guard(b: bool) -> StateList<()> {
    if b {
        Rc::new(move |s: State| PairList::from_value(((), s)))
    } else {
        mzero()
    }
}
