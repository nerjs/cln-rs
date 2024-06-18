// use quote::quote;
// use syn::parse2;
// use variant_parsers::VarDerive;
// mod classnames_parser;
// mod to_token_classnames;
// mod variant_parsers;

// fn main() {
//     let stream = quote! {
//         enum TT {
//             #[default]
//             #[key = value]
//             #[serde("k")]
//             #[class("a", "u")]
//             #[tratata(qwerty)]
//             #[class("b", zzz, 2)]
//             #[class()]
//             Simple,
//         }
//     };
//     let input = parse2::<VarDerive>(stream).unwrap();

//     println!("stream: {:?}", input);
// }

fn main() {}
