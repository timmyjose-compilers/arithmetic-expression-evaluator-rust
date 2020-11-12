pub trait Evaluatable {
    type Output;

    fn evaluate(&self) -> Self::Output;
}