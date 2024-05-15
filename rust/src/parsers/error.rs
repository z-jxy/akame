use nom::error::ContextError;

#[derive(Debug, PartialEq, Clone)]
pub enum CustomError<I> {
    MainFunctionWithParams(I),
    UnexpectedToken(I),
    // ... add other variants as needed
}

impl<I> nom::error::ParseError<I> for CustomError<I> {
    fn from_error_kind(input: I, _kind: nom::error::ErrorKind) -> Self {
        CustomError::UnexpectedToken(input)
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl std::fmt::Display for CustomError<&str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::MainFunctionWithParams(input) => write!(f, "Main function defenition cannot require arguments. Consider using `std::args` to access the arguments passed to the program: \x1b[91m{}\x1b[0m", input),
            CustomError::UnexpectedToken(input) => write!(f, "Unexpected token: {}", input),
            // ... handle other variants similarly
        }
    }
}

impl<I> ContextError<I> for CustomError<I>
where
    I: std::fmt::Display,
{
    fn add_context(_input: I, _ctx: &'static str, err: Self) -> Self {
        // Here, you can add the context to your CustomError if necessary
        // For the sake of this example, we'll just pass it through without modification
        err
    }
}

impl<'a> From<nom::error::Error<&'a str>> for CustomError<&'a str> {
    fn from(err: nom::error::Error<&'a str>) -> Self {
        CustomError::UnexpectedToken(err.input)
    }
}

/*
pub fn convert_error(err: nom::Err<nom::error::Error<&str>>) -> nom::Err<CustomError<&str>> {
    match err {
        nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        nom::Err::Error(e) => nom::Err::Error(CustomError::from(e)),
        nom::Err::Failure(e) => nom::Err::Failure(CustomError::from(e)),
    }
}
 */
