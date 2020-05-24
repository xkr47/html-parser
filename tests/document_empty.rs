use html_parser::{HtmlParser, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_empty_document() -> Result<()> {
    let markup = "";
    let dom = HtmlParser::parse(markup)?;
    assert_json_snapshot!(dom);
    Ok(())
}
