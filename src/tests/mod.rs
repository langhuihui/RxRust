#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        Rx::fromIterator(1..10).take(2).subscribe(Subscriber::new(
            |data| {
                println!("{}", data);
                if *data == 1 {}
            },
            |result| match result {
                Ok(_) => println!("complete"),
                Err(err) => println!("{}", err),
            },
        ));
    }
}
