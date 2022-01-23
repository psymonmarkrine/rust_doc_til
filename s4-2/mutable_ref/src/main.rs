fn main() {
    let mut s = String::from("THE_QUICK_BROWN_FOX");

    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}", r2);
    // println!("{}", r1); // 可変参照を複数作成しても実際に参照しなければエラーにならない

    change(&mut s); // r2とsome_stringの2つが参照されるが問題ない？スコープのせい？ライフタイム？
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str("_JUMPS_OVER_THE_LAZY_DOG");
}
