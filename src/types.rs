use crate::*;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
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
pub struct Event<'a, T>(
    pub &'a T,
    pub Rc<RefCell<&'a mut dyn FnMut(Event<T>) -> bool>>,
);
impl<'a, T> Deref for Event<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0
    }
}
impl<'a, T> Event<'a, T> {
    pub fn changeNext(&self, newNext: &'a mut dyn FnMut(Event<T>) -> bool) {
        self.1.replace(newNext);
    }
}
pub trait Observable: std::marker::Sized + Clone {
    type Item;
    fn newNext(
        next: &mut impl FnMut(Event<Self::Item>) -> bool,
    ) -> Rc<RefCell<&mut dyn FnMut(Event<Self::Item>) -> bool>> {
        Rc::new(RefCell::new(next))
    }
    fn next<'b, Item>(
        data: &'b Item,
        handler: Rc<RefCell<&'b mut dyn FnMut(Event<Item>) -> bool>>,
    ) -> bool {
        let newEvent = Event::<'b, Item>(data, handler.clone());
        let nextHandler = &mut *handler.borrow_mut();
        nextHandler(newEvent)
    }
    fn subscribe(
        &self,
        next: impl FnMut(Event<Self::Item>) -> bool,
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
