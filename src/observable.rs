#![allow(non_snake_case)]
use crate::*;
// use std::time::{Duration, SystemTime};
#[derive(Clone)]
pub struct FromVec<T> {
    data: Vec<T>,
}
impl<T: Clone> Observable for FromVec<T> {
    type Item = T;
    fn subscribe<OT: Observer<T = T>>(&self, sink: RcRefCell<OT>) {
        let mut observer = sink.borrow_mut();
        for i in &self.data {
            observer.next(i);
            if observer.isDisposed() {
                return;
            }
        }
        observer.complete(Ok(()))
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
pub struct Rx;

impl Rx {
    pub fn fromVec<'a, T: 'a>(data: Vec<T>) -> FromVec<T> {
        FromVec { data }
    }
    pub fn fromIterator<T, I: IntoIterator<Item = T>>(data: I) -> FromVec<T> {
        FromVec {
            data: data.into_iter().collect(),
        }
    }
    // pub fn interval<'a, T: 'a>(period: u32) -> Interval<T> {
    //     Interval {
    //         period,
    //         marker: std::marker::PhantomData,
    //     }
    // }
}
