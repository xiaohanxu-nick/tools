use anyhow::Result;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::BTreeSet;

use crate::{to_upper_snake_case, Grammar};

pub fn create_syntax_kinds(grammars: &[Grammar]) -> Result<String> {
	let mut kinds: BTreeSet<Ident> = BTreeSet::new();
	let mut match_functions = Vec::new();

	for grammar in grammars {
		let mut match_arms: Vec<TokenStream> = Vec::new();
		for node in &grammar.nodes {
			let kind_id = grammar.language.id_for_node_kind(&node.kind, node.named);
			if kind_id == 0 {
				continue;
			}

			let name = format_ident!("{}", to_upper_snake_case(&node.kind_name()));
			kinds.insert(name.clone());

			let arm = quote! { #kind_id => #name };
			match_arms.push(arm);
		}
		let error_kind = grammar.language.id_for_node_kind("ERROR", true);
		let match_fn_name = format_ident!("{}_syntax_kind", grammar.name);
		let match_fn = quote! {
			pub fn #match_fn_name(kind_id: u16) -> SyntaxKind {
				match kind_id {
					#(#match_arms),*,
					#error_kind => SyntaxKind::ERROR,
					_ => panic!("Syntax Kind not found: {}", kind_id)
				}
			}
		};
		match_functions.push(match_fn);
	}

	let tokenstream = quote! {
		#![doc = "This file has been autogenerated. Do not modify."]
		#![doc = "To update, run: `cargo run --bin sourcegen"]
		#![allow(bad_style)]
		#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
		#[repr(u16)]
		pub enum SyntaxKind {
		#(#kinds),*,
		WHITESPACE,
		ROOT,
		ERROR,
		__LAST,
		}
		use SyntaxKind::*;

		#(#match_functions)*
	};

	let code = crate::reformat(&tokenstream.to_string())?;
	Ok(code)
}
