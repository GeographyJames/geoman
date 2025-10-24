#[test]
fn json_test() {
    let json = serde_json::json!({"name": "James", "phone": 123456789});
    println!("{json}");
    println!("{:#}", json);
    println!("{}", json.to_string());
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
    panic!("fail test:  {json}")
}
