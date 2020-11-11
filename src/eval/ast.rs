pub enum ArithmeticExpression {
    Value(Int),
    Add(ArithmeticExpression, ArithmeticExpression),
    Sub(ArithmeticExpression, ArithmeticExpression),
    Mul(ArithmeticExpression, ArithmeticExpression),
    Div(ArithmeticExpression, ArithmeticExpression),
}