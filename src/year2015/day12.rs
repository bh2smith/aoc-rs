use serde_json::{Map, Value};

fn contains_red(json: &Map<String, Value>) -> bool {
    for (key, value) in json {
        if key == "red" || value == "red" {
            return true;
        }
    }
    false
}

fn extract_integers(json: &Value, integers: &mut Vec<i64>, skip_red: bool) {
    match json {
        Value::Number(num) => {
            if let Some(integer) = num.as_i64() {
                integers.push(integer);
            }
        }
        Value::Array(array) => {
            for item in array {
                extract_integers(item, integers, skip_red);
            }
        }
        Value::Object(obj) => {
            if skip_red && contains_red(obj) {
                return;
            }
            for (_, value) in obj {
                extract_integers(value, integers, skip_red);
            }
        }
        _ => {}
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let parsed_json: Value = serde_json::from_str(input).unwrap();

    let mut integers = Vec::new();
    extract_integers(&parsed_json, &mut integers, false);

    integers.iter().sum()
}

pub fn puzzle2(input: &str) -> i64 {
    let parsed_json: Value = serde_json::from_str(input).unwrap();

    let mut integers = Vec::new();
    extract_integers(&parsed_json, &mut integers, true);

    integers.iter().sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(r#"[[[3]]]"#), 3);
        assert_eq!(super::puzzle1(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(super::puzzle1(r#"{"a":[-1,1]}"#), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(super::puzzle2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(super::puzzle2(r#"[1,"red",5]"#), 6);
    }
}
