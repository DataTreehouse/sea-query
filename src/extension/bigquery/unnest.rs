use crate::SimpleExpr;

#[derive(Debug, Clone, PartialEq)]
pub struct Unnest {
    pub(crate) array_expression: Vec<SimpleExpr>
}

impl Unnest {
    pub fn new(arrays:Vec<SimpleExpr>) -> Unnest {
        Unnest {array_expression:arrays}
    }
}