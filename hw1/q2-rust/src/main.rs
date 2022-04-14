/// Takes an array of strings as input and finds the longest common prefix of all of the strings
fn find_lcp(arr: Vec<String>) -> String {
    if arr.len() == 0 {
        return "".to_string();
    }
    let mut lcp = arr[0].clone(); // treat first item as current longest common prefix
    for s in arr.clone() {
        let mut temp: String = "".to_string(); // to keep track of common prefix in current 2-string comparison
        for i in 0..s.len() {
            // if index out of range or characters not the same
            if i >= lcp.len() || s.chars().nth(i).unwrap() != lcp.chars().nth(i).unwrap() {
                break;
            } else {
                temp.push(s.chars().nth(i).unwrap());
            }
        }
        if temp.len() < lcp.len() { // update lcp if new word compared results in a shorter one
            lcp = temp;
        }
    }
    return lcp;
}

fn main() {
    let test_cases: [Vec<&str>; 8] = [
        vec!["apple", "app", "aple", "appl"],
        vec![],
        vec![""],
        vec!["abc"],
        vec!["abc", "xyz"],
        vec!["zzzzz", "zz", "zzzz"],
        vec!["bamboo", "bamboozled"],
        vec!["bamboo", "bamboozled", "bambam"]
    ];
    for tc in test_cases {
        let mut converted = Vec::new();
        print!("[");
        let mut count = 0;
        for s in tc.clone() {
            converted.push(s.to_string()); // convert from &str to String
            print!("{}", s);
            if count == tc.len()-1 {
                break;
            }
            print!(",");
            count += 1;
        }
        println!("]: {}", find_lcp(converted));
    }
}
