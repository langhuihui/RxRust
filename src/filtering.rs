#![allow(non_snake_case)]
use crate::*;
use std::rc::Rc;

pub struct Filter<O: Observable> {
    pub project: Rc<dyn Fn(&O::Output) -> bool>,
    pub source: O,
}
impl<O: Observable> Observable for Filter<O> {
    type Output = O::Output;
    fn subscribe(
        &self,
        mut next: impl FnMut(Self::Output) -> bool,
        complete: impl Fn(Result<(), &str>),
    ) -> Abort {
        self.source.subscribe(
            |data| {
                if (self.project)(&data) {
                    next(data)
                } else {
                    true
                }
            },
            complete,
        )
    }
}
impl<O: Observable> Clone for Filter<O> {
    fn clone(&self) -> Self {
        Filter {
            source: self.source.clone(),
            project: self.project.clone(),
        }
    }
}

//Begin Take Operator
#[derive(Clone)]
pub struct Take<O> {
    pub count: usize,
    pub source: O,
}
impl<O: Observable> Observable for Take<O> {
    type Output = O::Output;
    fn subscribe(
        &self,
        mut next: impl FnMut(Self::Output) -> bool,
        complete: impl Fn(Result<(), &str>),
    ) -> Abort {
        let mut remain = self.count;
        let defer = Abort::new();
        defer.push(self.source.subscribe(
            |data| {
                next(data);
                remain -= 1;
                if defer.aborted() || remain == 0 {
                    complete(Ok(()));
                    return false;
                }
                true
            },
            &complete,
        ));
        defer
    }
}
//End TakeOperator

//Begin TakeUntil
#[derive(Clone)]
pub struct TakeUntil<SO, CO> {
    pub control: CO,
    pub source: SO,
}

impl<SO: Observable, CO: Observable> Observable for TakeUntil<SO, CO> {
    type Output = SO::Output;
    fn subscribe(
        &self,
        next: impl FnMut(Self::Output) -> bool,
        complete: impl Fn(Result<(), &str>),
    ) -> Abort {
        let aball = Abort::new();
        let abc = self.control.subscribe(
            |_data| {
                aball.abort();
                complete(Ok(()));
                true
            },
            noopC,
        );
        aball
            .push(abc.clone())
            .push(self.source.subscribe(next, |result| {
                abc.abort();
                (&complete)(result)
            }));
        aball
    }
}
