extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 操作可能な構文木としてのRustコードの表現を構築する
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // トレイトの実装内容を構築
    // Build the trait implementation
    impl_hello_macro(&ast)
}
