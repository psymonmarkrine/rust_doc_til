fn main() {
    let mut v = vec![1, 2, 3];

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let third: &i32 = &v[2];
    // let third: Option<&i32> = v.get(2);

    // println!("{:#?}", third);

    // let _does_not_exist = &v[100];
    // let _does_not_exist = v.get(100);

    // v.push(9);

    // for i in &mut v{
    //     *i += 10;
    //     println!("{:#?}", i);
    // }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:#?}", row);
}
