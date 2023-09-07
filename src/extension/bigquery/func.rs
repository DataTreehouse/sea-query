use crate::{Function, FunctionCall, SimpleExpr};

#[derive(Debug, Clone, PartialEq)]
pub enum WeekDay {
    SUNDAY,
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateTimePart {
    MICROSECOND,
    MILLISECOND,
    SECOND,
    MINUTE,
    HOUR,
    DAYOFWEEK,
    DAY,
    DAYOFYEAR,
    WEEK,
    WEEKNUMBER(WeekDay),
    ISOWEEK,
    MONTH,
    QUARTER,
    YEAR,
    ISOYEAR,
    DATE,
    TIME,
}
#[derive(Debug, Clone, PartialEq)]
pub enum BqFunction {
    Extract(DateTimePart),
}

#[derive(Debug, Clone)]
pub struct BqFunc;
impl BqFunc {
    pub fn extract<T>(datetime_part: DateTimePart, datetime_expr: T) -> FunctionCall
    where
        T: Into<SimpleExpr>,
    {
        FunctionCall::new(Function::BqFunction(BqFunction::Extract(datetime_part))).arg(datetime_expr)
    }
}
