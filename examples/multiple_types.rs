use custom_error::CustomError;

#[derive(Debug)]
enum Type1 {
    Error1,
}

impl From<Type1> for SuperError {
    fn from(t: Type1) -> SuperError {
        SuperError::Type1(t)
    }
}

#[derive(Debug)]
enum Type2 {
    Error1,
}

impl From<Type2> for SuperError {
    fn from(t: Type2) -> SuperError {
        SuperError::Type2(t)
    }
}

#[derive(Debug)]
enum SuperError {
    Type1(Type1),
    Type2(Type2),
}

fn fn1() -> CustomError<Type1> {
    CustomError::new(Type1::Error1, "One error")
}

fn fn2() -> CustomError<Type2> {
    CustomError::new(Type2::Error1, "Another error")
}

fn fn3(one: bool) -> CustomError<SuperError> {
    if one {
        fn1().convert()
    } else {
        fn2().convert()
    }
}

fn main() {
    println!("{}", fn3(true));
    println!("{}", fn3(false));
}
