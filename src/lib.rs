pub mod eval;
pub mod repl;

pub mod error {
    use std::error;
    use std::fmt;

    #[derive(Debug)]
    pub struct ArithError {
        message: String,
    }

    impl ArithError {
        pub fn new(message: String) -> Self {
            ArithError { message }
        }
    }

    impl error::Error for ArithError {}

    impl fmt::Display for ArithError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }
}