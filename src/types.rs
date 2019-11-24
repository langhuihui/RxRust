use crate::*;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Rx;

impl Rx {
    #[inline]
    pub fn fromVec<T>(data: Vec<T>) -> FromVec<T> {
        FromVec { data }
    }
    #[inline]
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

pub trait Observable: std::marker::Sized + Clone {
    type Output;
    fn subscribe(
        &self,
        next: impl FnMut(Self::Output) -> bool,
        complete: impl Fn(Result<(), &str>),
    ) -> Abort;
    #[inline]
    fn take(&self, count: usize) -> Take<Self> {
        Take {
            count,
            source: self.clone(),
        }
    }
    #[inline]
    fn takeUntil<CO: Observable>(&self, control: &CO) -> TakeUntil<Self, CO> {
        TakeUntil {
            control: control.clone(),
            source: self.clone(),
        }
    }
    #[inline]
    fn map<T1, T2>(&self, project: impl Fn(T1) -> T2 + 'static) -> Map<Self, T1, T2> {
        Map {
            project: Rc::new(project),
            source: self.clone(),
        }
    }
    #[inline]
    fn filter(&self, project: impl Fn(&Self::Output) -> bool + 'static) -> Filter<Self> {
        Filter {
            project: Rc::new(project),
            source: self.clone(),
        }
    }
}

pub type RcDisposable = Rc<RefCell<Disposable>>;
pub struct Disposable {
    pub deferList: Vec<RcDisposable>,
    pub disposed: bool,
}
#[derive(Clone)]
pub struct Abort(RcDisposable);
impl Abort {
    pub fn new() -> Abort {
        Abort(Disposable::new())
    }
    pub fn aborted(&self) -> bool {
        self.0.borrow().disposed
    }
    pub fn abort(&self) {
        self.0.borrow_mut().dispose()
    }
    pub fn push(&self, a: Abort) -> &Self {
        self.0.borrow_mut().push(a.0);
        self
    }
}

impl Disposable {
    pub fn new() -> RcDisposable {
        Rc::new(RefCell::new(Disposable {
            deferList: vec![],
            disposed: false,
        }))
    }
    pub fn isDisposed(&self) -> bool {
        self.disposed
    }
    pub fn dispose(&mut self) {
        self.disposed = true;
        for i in &self.deferList {
            i.borrow_mut().dispose()
        }
        self.deferList.clear()
    }
    pub fn push(&mut self, d: RcDisposable) -> &mut Self {
        self.deferList.push(d);
        self
    }
}
pub fn noopN<T>(_: &T) -> bool {
    true
}
pub fn noopC(_: Result<(), &str>) {}
