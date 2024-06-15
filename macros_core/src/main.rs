mod parsers;

use parsers::ChunkList;
use quote::quote;
use syn::parse2;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let stream =
        quote! {"qwerty", &&some.test.my(1, &h, *u, &&i), "pp", (true, "qqq"), (&&yy, "pp", "oo")};
    println!("stream: {:#?}", stream.to_string());
    let a = parse2::<ChunkList>(stream)?;

    println!("lit str: {a:#?}");

    Ok(())
}
