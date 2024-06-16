// use macros_core::*;
use macros::cn;

fn main() {
    let first = cn!();
    let second = cn!("aaa", "bbb ccc", (true, "bbb"));
    let q = Some("uuu");
    let w: Option<&str> = None;
    let e = Some(true);
    let r: Option<bool> = None;
    let s = Some(String::from("from-var"));
    let last = cn!(
        "test",
        Some("second"),
        (false, "", "test"),
        q,
        w,
        (e, "f1", "s1"),
        (r, "f2", "s2"),
        s,
        Some(String::from("from-row"))
    );

    println!("1: {}, 2: {}, 3: {}", first, second, last)

    // let string_aa = true;
    // let a = cn!("hfff", ne.ty, (true, "test string"), (string_aa, "first", "second"));
    // println!("normalize list: {:?}", a);

    // let b = zzz!("aa bb   zz aa");
    // println!("normalize list from zzz: {}", b);

    // let b = normalize_list_from_util!("aa bb   zz aa");
    // println!("normalize list from util: {}", b);
}
