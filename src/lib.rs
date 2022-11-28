pub use persi_ds::sync::list::List;
use std::rc::Rc;

pub type State = List<i32>;
pub type Plan<A> = Rc<dyn Fn(State) -> (A, State)>;

pub fn run_plan<A>(pl: Plan<A>, s: State) -> (A, State) {
    pl(s)
}

pub fn make_plan<A, F>(f: F) -> Plan<A>
where
    F: Fn(State) -> (A, State) + 'static,
{
    Rc::new(f)
}

pub fn mreturn<A: Copy + 'static>(a: A) -> Plan<A> {
    Rc::new(move |s: State| (a, s))
}

pub fn mbind<A, F, B>(pl: Plan<A>, k: F) -> Plan<B>
where
    A: 'static,
    F: Fn(A) -> Plan<B> + 'static,
{
    Rc::new(move |s: State| {
        let ps = run_plan(pl.clone(), s);
        let pl_b = k(ps.0);
        run_plan(pl_b, ps.1)
    })
}
