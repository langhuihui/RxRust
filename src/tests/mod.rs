#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        // let x: Vec<_> = (1..2).map(|n| n * 2).collect();
        Rx::fromIterator(1..10)
            .take(4)
            .map(|n| n * 2)
            .filter(|n| *n > 2)
            .subscribe(
                |data| {
                    println!("{}", data);
                    true
                },
                |result| match result {
                    Ok(_) => println!("complete"),
                    Err(err) => println!("{}", err),
                },
            );
    }
}
