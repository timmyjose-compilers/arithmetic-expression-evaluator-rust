use crate::eval::traits;

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticExpression {
    Value(i32),
    Add(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Sub(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Mul(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
    Div(Box<ArithmeticExpression>, Box<ArithmeticExpression>),
}

impl traits::Evaluatable for ArithmeticExpression {
    type Output = i32;

    fn evaluate(&self) -> Self::Output {
        match *self {
            ArithmeticExpression::Value(ival) => ival,
            ArithmeticExpression::Add(ref lhs, ref rhs) => lhs.evaluate() + rhs.evaluate(),
            ArithmeticExpression::Sub(ref lhs, ref rhs) => lhs.evaluate() - rhs.evaluate(),
            ArithmeticExpression::Mul(ref lhs, ref rhs) => lhs.evaluate() * rhs.evaluate(),
            ArithmeticExpression::Div(ref lhs, ref rhs) => {
                let rval = rhs.evaluate();
                if rval == 0 {
                    0
                } else {
                    lhs.evaluate() / rval
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_expression() {
        let add_expr = ArithmeticExpression::Add(
            Box::new(ArithmeticExpression::Value(1)),
            Box::new(ArithmeticExpression::Add(
                Box::new(ArithmeticExpression::Value(2)),
                Box::new(ArithmeticExpression::Value(3)),
            )),
        );

        println!("{:?}", add_expr);
    }

    #[test]
    fn test_add_and_subexpression() {
        let add_sub_expr = ArithmeticExpression::Add(
            Box::new(ArithmeticExpression::Value(1)),
            Box::new(ArithmeticExpression::Sub(
                Box::new(ArithmeticExpression::Value(2)),
                Box::new(ArithmeticExpression::Value(3)),
            )),
        );

        println!("add_sub_expr = {:?}", add_sub_expr);

        let sub_add_expr = ArithmeticExpression::Sub(
            Box::new(ArithmeticExpression::Add(
                Box::new(ArithmeticExpression::Value(1)),
                Box::new(ArithmeticExpression::Value(2)),
            )),
            Box::new(ArithmeticExpression::Value(3)),
        );

        println!("sub_add_expr = {:?}", sub_add_expr);
    }
}