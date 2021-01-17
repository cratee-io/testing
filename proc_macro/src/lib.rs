// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

extern crate proc_macro;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

use proc_macro::TokenStream;
//use regex::Regex;

#[cfg(test)]
mod tests;

/// test parses the test function in pattern of
///
/// #[test]
/// #[should_panic(expected = "...")]
/// #[ignore]
/// fn any_fn_name_your_like() {
///     ...
/// }
///
/// @dev the order of #[should_panic] and #[ignore] is interchangeable.
/// @dev #[should_panic] and #[ignore] must comes after #[test] but not before
#[proc_macro_attribute]
pub fn test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    //use proc_macro::TokenTree;
    //for v in input.clone() {
    //    match v {
    //        TokenTree::Group(v) => println!("group: {}", v),
    //        TokenTree::Ident(v) => println!("id: {}", v),
    //        TokenTree::Punct(v) => println!("punct: {}", v),
    //        TokenTree::Literal(v) => println!("literal: {}", v),
    //    }
    //}

    let tokens = input.to_string();
    let (attrs, func) = split_attrs_and_func(tokens.as_str());

    let attrs = canonicalize_attributes(attrs);
    let attrs = attrs.as_str();

    let token_fn: TokenStream = func.parse().expect("invalid 'fn' token stream");

    let (should_panic, ignored) = figure_out_should_panic_and_ignored(attrs);
    let should_panic = if let Some(expected) = should_panic {
        quote! { Some(#expected) }
    } else {
        quote! { None }
    };

    let with_testing_gate = quote! { #[cfg(feature = "with-testing")] };

    let f = parse_macro_input!(token_fn as ItemFn);
    let f_ident = &f.sig.ident;
    // I know no ways to make the line/column for panic number right =_=
    // even if adding back original attributes to occupy lines
    let q = quote!(
        #with_testing_gate

        #f

        #with_testing_gate

        inventory::submit!(
            testing::TestCase::new(
                concat!(module_path!(), "::", stringify!(#f_ident)),
                #f_ident,
                #should_panic,
                #ignored,
            )
        );
    );

    //println!("{}", q);

    q.into()
}

// canonicalize_attributes canonicalized special characters in attrs.
// Currently, following replacements are taken
//   \" => "
fn canonicalize_attributes(attrs: &str) -> String {
    attrs.replace(r#"\""#, r#"""#)
}

// @dev assume attrs has been formatted
fn figure_out_should_panic_and_ignored(attrs: &str) -> (Option<&str>, bool) {
    const IGNORE: &str = "#[ignore]";

    let (attrs, ignored) = {
        let attrs = attrs.trim();
        if let Some(v) = attrs.strip_prefix(IGNORE) {
            (v.trim_start(), true)
        } else if let Some(v) = attrs.strip_suffix(IGNORE) {
            (v.trim_end(), true)
        } else {
            (attrs, false)
        }
    };

    let should_panic_expected = figure_out_should_panic(attrs);

    return (should_panic_expected, ignored);
}

fn figure_out_should_panic(attrs: &str) -> Option<&str> {
    const SHOULD_PANIC: &str = "#[should_panic]";
    const EXPECTED_SHOULD_PANIC_PREFIX: &str = r#"#[should_panic(expected ="#;
    const EXPECTED_SHOULD_PANIC_SUFFIX: &str = r#"")]"#;

    if attrs == SHOULD_PANIC {
        return Some("");
    }

    attrs
        .strip_prefix(EXPECTED_SHOULD_PANIC_PREFIX)
        .map(|v| v.trim_start())
        //.map(|v| { println!("1. '{}'", v); v })
        .map(|v| v.strip_prefix('"'))
        .map(|v| v.expect("missing leading quote"))
        //.map(|v| { println!("2. '{}'", v); v })
        .map(|v| v.strip_suffix(EXPECTED_SHOULD_PANIC_SUFFIX))
        .map(|v| v.expect("missing wrapping expected should_panic"))
}

fn split_attrs_and_func(tokens: &str) -> (&str, &str) {
    if tokens.starts_with("fn ") {
        return ("", tokens);
    }

    // @dev: actually this is buggy if attrs contains 'fn'. Possible solutions is to parse token
    // tree manually.
    for v in &["\nfn ", " fn\n", " fn "] {
        if let Some(idx) = tokens.find(v) {
            return tokens.split_at(idx);
        }
    }

    unreachable!();
}
