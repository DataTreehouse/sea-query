use crate::extension::bigquery::{DateTimePart, WeekDay};
use super::*;

impl OperLeftAssocDecider for BigQueryQueryBuilder {
    fn well_known_left_associative(&self, op: &BinOper) -> bool {
        common_well_known_left_associative(op)
    }
}

impl PrecedenceDecider for BigQueryQueryBuilder {
    fn inner_expr_well_known_greater_precedence(
        &self,
        inner: &SimpleExpr,
        outer_oper: &Oper,
    ) -> bool {
        common_inner_expr_well_known_greater_precedence(inner, outer_oper)
    }
}

impl QueryBuilder for BigQueryQueryBuilder {
    fn placeholder(&self) -> (&str, bool) {
        ("$", true)
    }

    fn prepare_simple_expr(&self, simple_expr: &SimpleExpr, sql: &mut dyn SqlWriter) {
        match simple_expr {
            SimpleExpr::AsEnum(type_name, expr) => {
                let simple_expr = expr.clone().cast_as(SeaRc::clone(type_name));
                self.prepare_simple_expr_common(&simple_expr, sql);
            }
            SimpleExpr::Struct(nfs) => {
                write!(sql, "STRUCT (").unwrap();
                for (i, nf) in nfs.iter().enumerate() {
                    self.prepare_simple_expr(&nf.field_value, sql);
                    if let Some(name) = &nf.name {
                        write!(sql, " AS {name}").unwrap();
                    }
                    if i != (nfs.len() -1) {
                        write!(sql, ",").unwrap();
                    }
                }
                write!(sql, ")").unwrap();
            }
            _ => self.prepare_simple_expr_common( simple_expr, sql),
        }
    }

    fn prepare_table_ref(&self, table_ref: &TableRef, sql: &mut dyn SqlWriter) {
        if let TableRef::Unnest(unnest, alias) = table_ref {
            write!(sql, "UNNEST ").unwrap();
            write!(sql, "[").unwrap();
            for se in &unnest.array_expression {
                self.prepare_simple_expr(se, sql);
            }
            let quote = self.quote();
            write!(sql,"] AS {}{}{}", quote.0, alias.to_string(), quote.1 ).unwrap();
        } else {
            self.prepare_table_ref_common(table_ref, sql);
        }
    }

    fn prepare_bin_oper(&self, bin_oper: &BinOper, sql: &mut dyn SqlWriter) {
        self.prepare_bin_oper_common(bin_oper, sql);
    }

    fn prepare_query_statement(&self, query: &SubQueryStatement, sql: &mut dyn SqlWriter) {
        query.prepare_statement(self, sql);
    }

    fn prepare_function_call(&self, func:&FunctionCall, sql: &mut dyn SqlWriter) {
        if let Function::BqFunction(bqf) = &func.func {
            match bqf {
                BqFunction::Extract(datetime_part) => {
                    write!(sql, "EXTRACT(").unwrap();
                    let datetime_part_string = match datetime_part {
                        DateTimePart::MICROSECOND => {
                            "MICROSECOND".to_string()
                        }
                        DateTimePart::MILLISECOND => {
                            "MILLISECOND".to_string()
                        }
                        DateTimePart::SECOND => {
                            "SECOND".to_string()
                        }
                        DateTimePart::MINUTE => {
                            "MINUTE".to_string()
                        }
                        DateTimePart::HOUR => {
                            "HOUR".to_string()
                        }
                        DateTimePart::DAYOFWEEK => {
                            "DAYOFWEEK".to_string()
                        }
                        DateTimePart::DAY => {
                            "DAY".to_string()
                        }
                        DateTimePart::DAYOFYEAR => {
                            "DAYOFYEAR".to_string()
                        }
                        DateTimePart::WEEK => {
                            "WEEK".to_string()
                        }
                        DateTimePart::WEEKNUMBER(weekday) => {
                            format!("WEEK({})", match weekday {
                                WeekDay::SUNDAY => {"SUNDAY"}
                                WeekDay::MONDAY => {"MONDAY"}
                                WeekDay::TUESDAY => {"TUESDAY"}
                                WeekDay::WEDNESDAY => {"WEDNESDAY"}
                                WeekDay::THURSDAY => {"THURSDAY"}
                                WeekDay::FRIDAY => {"FRIDAY"}
                                WeekDay::SATURDAY => {"SATURDAY"}
                            })
                        }
                        DateTimePart::ISOWEEK => {"ISOWEEK".to_string()}
                        DateTimePart::MONTH => {"MONTH".to_string()}
                        DateTimePart::QUARTER => {"QUARTER".to_string()}
                        DateTimePart::YEAR => {"YEAR".to_string()}
                        DateTimePart::ISOYEAR => {"ISOYEAR".to_string()}
                        DateTimePart::DATE => {"DATE".to_string()}
                        DateTimePart::TIME => {"TIME".to_string()}
                    };
                    write!(sql, "{} FROM ", datetime_part_string).unwrap();
                    assert_eq!(func.args.len(), 1);
                    self.prepare_simple_expr(func.args.get(0).unwrap(), sql);
                    write!(sql, ")").unwrap();
                }
            }
        } else {
            self.prepare_function_call_common(func, sql);
        }
    }

    fn prepare_order_expr(&self, order_expr: &OrderExpr, sql: &mut dyn SqlWriter) {
        if !matches!(order_expr.order, Order::Field(_)) {
            self.prepare_simple_expr(&order_expr.expr, sql);
        }
        self.prepare_order(order_expr, sql);
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => write!(sql, " NULLS LAST").unwrap(),
            Some(NullOrdering::First) => write!(sql, " NULLS FIRST").unwrap(),
        }
    }

    fn prepare_value(&self, value: &Value, sql: &mut dyn SqlWriter) {
        sql.push_param(value.clone(), self as _);
    }

    fn write_string_quoted(&self, string: &str, buffer: &mut String) {
        let escaped = self.escape_string(string);
        let string = if escaped.find('\\').is_some() {
            "E'".to_owned() + &escaped + "'"
        } else {
            "'".to_owned() + &escaped + "'"
        };
        write!(buffer, "{string}").unwrap()
    }

    fn write_bytes(&self, bytes: &[u8], buffer: &mut String) {
        write!(
            buffer,
            "'\\x{}'",
            bytes.iter().map(|b| format!("{b:02X}")).collect::<String>()
        )
        .unwrap()
    }

    fn if_null_function(&self) -> &str {
        "COALESCE"
    }
}
