use macros_core::{ChunkList, IntoCnParser, IntoCnTypes};
use quote::{quote, ToTokens};
use syn::parse2;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let stream = quote! {(false, "kk", "test z"), "qwerty", "test", &&some.test.my(1, &h, *u, &&i), "pp", (true, "qqq"), (&&yy, "pp", "oo")};

    let a = parse2::<ChunkList>(stream)?
        .into_cn_parser()?
        .into_cn_types()
        .to_token_stream()
        .to_string();
    // .into_token_stream();

    println!("lit str: {a:#?}");

    Ok(())
}
