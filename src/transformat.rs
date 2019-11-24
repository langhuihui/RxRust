#![allow(non_snake_case)]
use crate::*;
use std::rc::Rc;

pub struct Map<O, T1, T2> {
    pub project: Rc<dyn Fn(T1) -> T2>,
    pub source: O,
}
impl<O: Observable<Output = T1>, T1, T2> Observable for Map<O, T1, T2> {
    type Output = T2;
    fn subscribe<R: std::future::Future<Output = Done>>(
        &self,
        mut next: impl FnMut(T2) -> bool,
        complete: impl Fn(Result<(), &str>),
    ) -> R {
        self.source
            .subscribe(|data| next((self.project)(data)), complete)
    }
}
impl<O: Observable<Output = T1>, T1, T2> Clone for Map<O, T1, T2> {
    fn clone(&self) -> Self {
        Map {
            source: self.source.clone(),
            project: self.project.clone(),
        }
    }
}
