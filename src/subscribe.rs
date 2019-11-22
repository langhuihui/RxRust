use crate::*;
pub struct Subscriber<T, NT, CT> {
    pub onNext: NT,
    pub onComplete: CT,
    pub disposable: RcDisposable,
    marker: std::marker::PhantomData<T>,
}
impl<T, NT: FnMut(&T), CT: Fn(Result<(), &str>)> Subscriber<T, NT, CT> {
    pub fn new(onNext: NT, onComplete: CT) -> RcRefCell<Subscriber<T, NT, CT>> {
        newRcRefCell(Subscriber::<T, NT, CT> {
            onNext,
            onComplete,
            disposable: Disposable::new(),
            marker: std::marker::PhantomData,
        })
    }
}
impl<T, NT: FnMut(&T), CT: Fn(Result<(), &str>)> Observer for Subscriber<T, NT, CT> {
    type T = T;
    fn next(&mut self, data: &T) {
        (self.onNext)(data)
    }
    fn complete(&self, result: Result<(), &str>) {
        (self.onComplete)(result)
    }
    implDisposable!();
}
