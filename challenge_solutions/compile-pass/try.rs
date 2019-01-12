// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.
struct ErrorA;
struct ErrorB;

enum Error {
    A(ErrorA),
    B(ErrorB)
}

fn do_a() -> Result<u16, ErrorA> {
    Err(ErrorA)
}

fn do_b() -> Result<u32, ErrorB> {
    Err(ErrorB)
}

fn do_both() -> Result<(u16, u32), Error> {
    let first: u16 = match do_a() {
        Ok(number) => number,
        Err(ErrorA) => panic!("Error occured in do_a"),
    };

    let second: u32 = match do_b() {
        Ok(number) => number,
        Err(ErrorB) => panic!("Error occured in do_b"),
    };

    Ok((first, second))
}

fn main() { }
