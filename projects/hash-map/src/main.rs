use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(75);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let mut _numberlist = [9, 3, 1, 4, 2, 6, 5, 7, 8, 0];
    _numberlist.sort();
    let _mid = _numberlist.len() / 2;

    if _numberlist.len() % 2 == 0 {
        let _median = (_numberlist[_mid] + _numberlist[_mid - 1]) / 2;
        println!("Median: {}", _median);
    } else {
        let _median = _numberlist[_mid];
        println!("Median: {}", _median);
    }
    println!("{:?}", _numberlist);
}
