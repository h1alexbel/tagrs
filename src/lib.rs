// The MIT License (MIT)
//
// Copyright (c) 2024 Aliaksei Bialiauski
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
/*!
Rust test tagging.
 */
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

/// Tag.
#[proc_macro_attribute]
pub fn tag(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    let tag = if let Some(arg) = args.first() {
        if let syn::NestedMeta::Lit(syn::Lit::Str(lit)) = arg {
            lit.value()
        } else {
            panic!("Expected a string literal for the tag");
        }
    } else {
        panic!("Expected a tag argument");
    };
    let name = &input.sig.ident;
    let block = &input.block;
    let has_test_attr = input.attrs.iter().any(|attr| attr.path.is_ident("test"));
    let run = std::env::var("TTAG").unwrap_or_else(|_| "<none>".to_string());
    let ignore_attr = if run != tag {
        quote! { #[ignore] }
    } else {
        quote! {}
    };
    let gen = if has_test_attr {
        quote! {
            #ignore_attr
            #[test]
            fn #name() -> anyhow::Result<()> {
                if #run != #tag {
                    println!(
                        "Test '{}' ignored due to tag mismatch: expected '{}', run '{}'",
                        stringify!(#name), #tag, #run
                    );
                }
                #block
            }
        }
    } else { 
        panic!("Tag {} should be used with #[test]", tag);
    };
    gen.into()
}
