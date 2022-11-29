fn main() {
    puzzle::run();
    example_plan::run();
    example_identity();
    example_pair();
}

pub mod puzzle {
    use more_money::{
        statelist::{guard, make_state_list, mbind, mreturn, run_state_list, PairList, StateList},
        List,
    };

    use persi_ds::sync::list::for_each;

    fn select(lst: List<i32>) -> PairList<i32> {
        if lst.is_empty() {
            return PairList::new();
        }
        let x = lst.front().unwrap();
        let xs = lst.popped_front();

        let mut result = PairList::new();
        for p in select(xs.clone()).iter() {
            let y = p.0;
            let ys = p.1.clone();
            result = result.pushed_front((y, ys.pushed_front(*x)));
        }

        result.pushed_front((*x, xs.clone()))
    }

    fn as_number(v: &[i32]) -> i32 {
        let mut acc = 0;
        for i in v {
            acc = 10 * acc + i;
        }
        acc
    }
    fn solve() -> StateList<(i32, i32, i32)> {
        let sel = make_state_list(&select);

        mbind(sel.clone(), move |s| {
            let sel2 = sel.clone();
            mbind(sel2.clone(), move |e| {
                let sel3 = sel2.clone();
                mbind(sel3.clone(), move |n| {
                    let sel4 = sel3.clone();
                    mbind(sel4.clone(), move |d| {
                        let sel5 = sel4.clone();
                        mbind(sel5.clone(), move |m| {
                            let sel6 = sel5.clone();
                            mbind(sel6.clone(), move |o| {
                                let sel7 = sel6.clone();
                                mbind(sel7.clone(), move |r| {
                                    let sel8 = sel7.clone();
                                    mbind(sel8, move |y| {
                                        mbind(guard(s != 0 && m != 0), move |()| {
                                            let send = as_number(&[s, e, n, d]);
                                            let more = as_number(&[m, o, r, e]);
                                            let money = as_number(&[m, o, n, e, y]);
                                            mbind(guard(send + more == money), move |()| {
                                                mreturn((send, more, money))
                                            })
                                        })
                                    })
                                })
                            })
                        })
                    })
                })
            })
        })
    }
    pub fn run() {
        let lst = List::cons(
            0,
            &List::cons(
                1,
                &List::cons(
                    2,
                    &List::cons(
                        3,
                        &List::cons(
                            4,
                            &List::cons(
                                5,
                                &List::cons(
                                    6,
                                    &List::cons(7, &List::cons(8, &List::from_value(9))),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        let (s, _state) = run_state_list(solve(), lst).front().unwrap().clone();
        println!("  send    {}", s.0);
        println!("+ more  + {}", s.1);
        println!("------  ----------");
        println!(" money   {}", s.2);
    }
}
pub mod example_plan {
    use more_money::List;
    fn select(lst: List<i32>) -> (i32, List<i32>) {
        let i = lst.front().unwrap();
        (*i, lst.popped_front())
    }
    pub fn run() {
        use more_money::state::{make_plan, mbind, mreturn, run_plan};
        let sel = make_plan(select);
        let pl = mbind(sel.clone(), move |i| {
            mbind(sel.clone(), move |j| mreturn((i, j)))
        });
        let st = List::cons(1, &List::cons(2, &List::from_value(3)));
        println!("example: {:?}", run_plan(pl, st));
    }
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
