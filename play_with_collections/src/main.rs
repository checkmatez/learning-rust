fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Float(3.0),
        SpreadsheetCell::Text(String::from("red")),
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Text(s) => println!("cell: {}", s),
            _ => continue,
        }
    }
}
