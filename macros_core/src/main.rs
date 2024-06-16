// use macros_core::{ChunkList, IntoCnParser, IntoCnTypes};
// use quote::{quote, ToTokens};
// use syn::parse2;

// fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//     let stream = quote! {(false, "kk", "test z"), "qwerty", "test", &&some.test.my(1, &h, *u, &&i), "pp", (true, "qqq"), (&&yy, "pp", "oo")};

//     let a = parse2::<ChunkList>(stream)?
//         .into_cn_parser()?
//         .into_cn_types()
//         .to_token_stream()
//         .to_string();
//     // .into_token_stream();

//     println!("lit str: {a:#?}");

//     Ok(())
// }

use macros_core::CnBuilder;

fn main() {
    let qw: Option<&str> = Some("item");
    let qw2: Option<&str> = None;
    let classlist = CnBuilder::new()
        .add("item")
        .add(String::from("test"))
        .add(qw)
        .add(qw2)
        .add(Some("aaa"))
        .add((true, "true-once"))
        .add((false, "false-once"))
        .add((true, "true-two", "false-two"))
        .add((false, "true-two", "false-two"))
        .to_classlist();

    println!("cnl: {}", classlist);
}
