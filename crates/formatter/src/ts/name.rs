use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Name;

impl ToFormatElement for Name {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		formatter.format_token(&self.ident_token().unwrap())
	}
}
