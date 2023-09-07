pub(crate) mod query;
use super::*;

/// BigQuery query builder.
#[derive(Default, Debug)]
pub struct BigQueryQueryBuilder;

const QUOTE: Quote = Quote(b'`', b'`');

impl QuotedBuilder for BigQueryQueryBuilder {
    fn quote(&self) -> Quote {
        QUOTE
    }
}

impl EscapeBuilder for BigQueryQueryBuilder {}

impl TableRefBuilder for BigQueryQueryBuilder {}
