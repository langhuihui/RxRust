use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

pub type RcRefCell<T> = Rc<RefCell<T>>;
#[inline]
pub fn newRcRefCell<T>(v: T) -> RcRefCell<T> {
    Rc::new(RefCell::new(v))
}

pub trait Observable: std::marker::Sized + Clone {
    type Item;
    fn subscribe<T: Observer<T = Self::Item>>(&self, sink: RcRefCell<T>);
    #[inline]
    fn take(&self, count: usize) -> Take<Self> {
        Take {
            count,
            source: self.clone(),
            disposable: Disposable::new(),
        }
    }
    #[inline]
    fn takeUntil<CO: Observable, CT: Observer>(&self, control: &CO) -> TakeUntil<Self, CO, CT> {
        TakeUntil {
            control: control.clone(),
            source: self.clone(),
            marker: std::marker::PhantomData,
        }
    }
}
pub trait Observer {
    type T;
    #[inline]
    fn next(&mut self, data: &Self::T) {
        self.sinkNext(data)
    }
    #[inline]
    fn complete(&self, result: Result<(), &str>) {
        self.sinkComplete(result)
    }
    #[inline]
    fn dispose(&self) {}
    #[inline]
    fn isDisposed(&self) -> bool {
        false
    }
    #[inline]
    fn pushD(&self, _d: RcDisposable) {}
    #[inline]
    fn sinkNext(&self, _data: &Self::T) {}
    #[inline]
    fn sinkComplete(&self, _result: Result<(), &str>) {}
}

pub type RcDisposable = RcRefCell<Disposable>;
pub struct Disposable {
    pub deferList: Vec<RcDisposable>,
    disposed: bool,
}
impl Disposable {
    pub fn new() -> RcDisposable {
        newRcRefCell(Disposable {
            deferList: vec![],
            disposed: false,
        })
    }
    pub fn isDisposed(&self) -> bool {
        self.disposed
    }
    pub fn dispose(&mut self) {
        self.disposed = true;
        for i in &self.deferList {
            i.borrow_mut().dispose()
        }
    }
}
#[macro_export]
macro_rules! implDisposable {
    (  ) => {
        #[inline]
        fn dispose(&self) {
            self.disposable.borrow_mut().dispose()
        }
        #[inline]
        fn isDisposed(&self) -> bool {
            self.disposable.borrow().isDisposed()
        }
        #[inline]
        fn pushD(&self, d: RcDisposable) {
            self.disposable.borrow_mut().deferList.push(d)
        }
    };
}

#[macro_export]
macro_rules! implOperator {
    (  ) => {
        implDisposable!();
        #[inline]
        fn sinkNext(&self, data: &Self::T) {
            self.sink.borrow_mut().next(data);
        }
        #[inline]
        fn sinkComplete(&self, result: Result<(), &str>) {
            self.sink.borrow().complete(result);
        }
    };
}
