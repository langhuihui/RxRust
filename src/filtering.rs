#![allow(non_snake_case)]
use crate::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
//Begin Take Operator
#[derive(Clone)]
pub struct Take<O> {
    pub count: usize,
    pub source: O,
    pub disposable: RcDisposable,
}

impl<O: Observable> Observable for Take<O> {
    type Item = O::Item;
    fn subscribe<OT: Observer<T = Self::Item>>(&self, sink: RcRefCell<OT>) {
        let newTake = self.clone()
        sink.borrow().pushD(self.disposable.clone());
        self.source.subscribe(newRcRefCell(self));
    }
}

impl<O: Observer> Observer for Take<O> {
    type T = O::T;
    fn next(&mut self, data: &Self::T) {
        self.sinkNext(data);
        self.count -= 1;
        if self.count == 0 {
            self.dispose();
            self.complete(Ok(()));
        }
    }
    implOperator!();
}

//End TakeOperator

//Begin TakeUntil
pub struct TakeUntil<SO, CO, CT> {
    pub control: CO,
    pub source: SO,
    pub marker: std::marker::PhantomData<CT>,
}
impl<SO: Observable, CO: Observable, CT: Observer<T = CO::Item>> Observable
    for TakeUntil<SO, CO, CT>
{
    type Item = SO::Item;
    fn subscribe<OT: Observer<T = Self::Item>>(&self, sink: RcRefCell<OT>) -> RcRefCell<OT> {
        let dis = Disposable::new();
        let cdis = Disposable::new();
        sink.borrow().pushD(dis.clone());
        sink.borrow().pushD(cdis.clone());
        let controlObWeak = newRcRefCell(Weak::new());
        let observer = newRcRefCell(TakeUntilObserver {
            sink: sink.clone(),
            control: controlObWeak.clone(),
            disposable: dis,
        });
        let controlObserver = newRcRefCell(TakeUntilControlObserver::<OT, CT> {
            target: observer.clone(),
            disposable: cdis,
        });
        *controlObWeak.borrow_mut() = Rc::downgrade(&controlObserver);
        self.control
            .subscribe::<TakeUntilControlObserver<OT, CT>>(controlObserver);
        self.source.subscribe(observer);
        sink
    }
}
impl<SO: Observable, CO: Observable, CT: Observer<T = CO::Item>> Clone for TakeUntil<SO, CO, CT> {
    fn clone(&self) -> Self {
        TakeUntil {
            source: self.source.clone(),
            control: self.control.clone(),
            marker: self.marker,
        }
    }
}
struct TakeUntilControlObserver<OT, CT> {
    target: RcRefCell<TakeUntilObserver<OT, CT>>,
    pub disposable: RcDisposable,
}
struct TakeUntilObserver<OT, CT> {
    sink: RcRefCell<OT>,
    control: RcRefCell<Weak<RefCell<TakeUntilControlObserver<OT, CT>>>>,
    pub disposable: RcDisposable,
}

impl<OT: Observer, CT: Observer> Observer for TakeUntilControlObserver<OT, CT> {
    type T = CT::T;
    #[inline]
    fn next(&mut self, _data: &Self::T) {
        self.target.as_ref().borrow().complete(Ok(()))
    }
    fn complete(&self, _result: Result<(), &str>) {}
}
impl<OT: Observer, CT: Observer> Observer for TakeUntilObserver<OT, CT> {
    type T = OT::T;
    fn complete(&self, result: Result<(), &str>) {
        self.control.as_ref().borrow().upgrade().and_then(|c| {
            c.as_ref().borrow().dispose();
            Some(())
        });
        self.sinkComplete(result)
    }
    implOperator!();
}
