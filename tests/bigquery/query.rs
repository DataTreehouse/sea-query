use super::*;
use pretty_assertions::assert_eq;

#[test]
fn select_1() {
    assert_eq!(
        Query::select()
            .columns([Char::Character, Char::SizeW, Char::SizeH])
            .from(Char::Table)
            .limit(10)
            .offset(100)
            .to_string(BigQueryQueryBuilder),
        r#"SELECT `character`, `size_w`, `size_h` FROM `character` LIMIT 10 OFFSET 100"#
    );
}
