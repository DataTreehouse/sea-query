use crate::SimpleExpr;

#[derive(Debug, Clone, PartialEq)]
pub struct NamedField {
    pub name:Option<String>,
    pub field_value: Box<SimpleExpr>
}

impl NamedField {
    pub fn new(name:Option<String>, field_value:SimpleExpr) -> NamedField {
        NamedField { name, field_value: Box::new(field_value) }
    }
}