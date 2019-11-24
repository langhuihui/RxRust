#![allow(non_snake_case)]
use crate::*;
use std::future::*;
use std::pin::*;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::prelude::*;
#[derive(Clone)]
pub struct FromVec<T> {
    pub(super) data: Vec<T>,
}

// async fn fromVec<T: Clone>(source: FromVec<T>, mut next: impl FnMut(T) -> bool) -> Done {
//     let dis = Abort::new();
//     for i in source.data.clone() {
//         if next(i) == false {
//             return Ok(());
//         }
//     }
//     Ok(())
// }
impl<T: Clone + std::marker::Sync> FromVec<T> {
    async fn sub<N: (FnMut(T) -> bool) + std::marker::Send>(&self, mut next: N) -> () {
        let dis = Abort::new();
        for i in self.data.clone() {
            if next(i) == false {
                return;
            }
        }
        // complete(Ok(()));
    }
}
impl<T: Clone + std::marker::Sync> Observable for FromVec<T> {
    type Output = T;
    fn subscribe<N: (FnMut(T) -> bool) + std::marker::Send>(
        &self,
        mut next: N,
        complete: impl Fn(Result<(), &str>),
    ) -> tokio::executor::Spawn {
        tokio::spawn(self.sub(next))
        // tokio::spawn(async move {
        //     let dis = Abort::new();
        //     for i in self.data.clone() {
        //         if next(i) == false {
        //             return;
        //         }
        //     }
        //     complete(Ok(()));
        // })
    }
}
#[derive(Clone)]
pub struct Interval {
    period: u64,
}
impl Interval {
    async fn subscribeAsyn(&self) {}
}
impl Observable for Interval {
    type Output = usize;
    fn subscribe<R: std::future::Future<Output = Done>>(
        &self,
        mut next: impl FnMut(Self::Output) -> bool,
        complete: impl Fn(Result<(), &str>),
    ) -> R {
        async move {
            let dis = Abort::new();
            let mut interval =
                tokio::timer::Interval::new_interval(Duration::from_millis(self.period));
            loop {
                interval.next().await;
            }
            dis
        }
    }
}
