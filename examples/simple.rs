use custom_error::*;

#[derive(Debug)]
enum ErrorType {
    ParseError,
    IntegerOverflow,
    DivideByZero,
}

fn main() {
    if let Err(e) = "oops".parse::<isize>() {
        println!(
            "{}\n",
            CustomError::new(ErrorType::ParseError, "Could not parse to isize")
                .message("I did really expect to parse it as an isize.")
                .context(Context::new(e.to_string()))
        )
    }

    println!(
        "{}\n",
        CustomError::new(ErrorType::DivideByZero, "Attempt to divide by 0")
            .help("Divide by 0 is mathematically undefined so it cannot be completed.")
            .url("https://www.mathsisfun.com/numbers/dividing-by-zero.html")
    );

    println!(
        "{}\n",
        CustomError!(ErrorType::IntegerOverflow, "Integer overflow").context(
            Context::new("    let x = n / test;")
                .linenumber(123)
                .context_before(vec!["fn calc(test: usize) {", "    let n = 123;"])
                .context_after(vec!["    println!(\"{}\", x);", "}"])
                .highlight(12, 8)
        )
    );
}
