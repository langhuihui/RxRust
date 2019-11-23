#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        Rx::fromIterator(1..10).take(4).subscribe(
            |data| {
                println!("{}", *data);
                true
            },
            |result| match result {
                Ok(_) => println!("complete"),
                Err(err) => println!("{}", err),
            },
        );
    }
}
