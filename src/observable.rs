#![allow(non_snake_case)]
use crate::*;
// use std::time::{Duration, SystemTime};
#[derive(Clone)]
pub struct FromVec<T> {
    pub(super) data: Vec<T>,
}

impl<T: Clone> Observable for FromVec<T> {
    type Output = T;
    fn subscribe(
        &self,
        mut next: impl FnMut(Self::Output) -> bool,
        complete: impl Fn(Result<(), &str>),
    ) -> Abort {
        let dis = Abort::new();
        for i in self.data.clone() {
            if next(i) == false {
                return dis;
            }
        }
        complete(Ok(()));
        dis
    }
}

// pub struct Interval<T> {
//     period: u32,
//     marker: std::marker::PhantomData<T>,
// }
// impl<T> Observable for Interval<T> {
//     type T = T;
//     fn subscribe<OT: Observer<T = Self::T>>(&mut self, sink: Rc<RefCell<OT>>) -> Rc<RefCell<OT>> {
//         let mut observer = sink.borrow_mut();
//         sink
//     }
// }
