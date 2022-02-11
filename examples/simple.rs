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
            "{}",
            CustomError::new(ErrorType::ParseError)
                .message("I did really expect to parse it as an isize.")
                .context(Context::line(e.to_string()))
        )
    }

    println!(
        "{}",
        CustomError::new(ErrorType::DivideByZero)
            .help("Divide by 0 is mathematically undefined so it cannot be completed.")
            .url("https://www.mathsisfun.com/numbers/dividing-by-zero.html")
    );

    println!(
        "{}",
        CustomError!(ErrorType::IntegerOverflow).multiple_context(vec![
            Context::lines(vec!["use crate::*;", "#[deny(overflow)]", "",])
                .linenumber(5)
                .highlights(vec![Highlight::new(1, 8, 7)
                    .note("Overflow deny is set here")
                    .info(),]),
            Context::lines(vec![
                "fn calc(test: usize) {",
                "    let n = 123;",
                "    let x = n * test;",
                "    println!(\"{}\", x);",
                "}"
            ])
            .linenumber(121)
            .highlights(vec![
                Highlight::new(2, 12, 8).note("Overflow happened here"),
                Highlight::new(1, 8, 7).note("'n' is small").info(),
                Highlight::new(0, 8, 4)
                    .note("'test' is unconstrained")
                    .info(),
            ]),
        ])
    );
}
