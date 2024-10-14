use serde_json::Value;

// Function that iter  over array of object and apply a preicate
pub fn json_filter(arr: &Vec<Value>, predicate: fn(&Value) -> bool) -> Vec<Value> {
    arr.iter().filter(|&v| predicate(v)).cloned().collect()
}

pub fn json_map(arr: &Vec<Value>, f: fn(&Value) -> Value) -> Vec<Value> {
    arr.iter().map(|v| f(v)).clone().collect()
}

pub fn json_fold<T>(arr: &Vec<Value>, zero: T, folder: fn(T, &Value) -> T) -> T {
    arr.iter().fold(zero, |acc, v| folder(acc, v))
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_sample() -> Value {
        json!(
            [
                {"name": "John", "age": 30},
                {"name": "Jane", "age": 25}
            ]
        )
    }

    #[test]
    fn test_json_filter() {
        let data = create_sample();
        let filtered = json_filter(&data.as_array().unwrap(), |v| {
            v["age"].as_i64().unwrap() > 25
        });
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_json_map() {
        let data = create_sample();
        let mapped = json_map(&data.as_array().unwrap(), |v| v.clone());
        assert_eq!(mapped.len(), 2);
    }

    #[test]
    fn test_json_fold() {
        let data = create_sample();
        let folded = json_fold(data.as_array().unwrap(), (0, 0), |acc, v| {
            let nb_elements = acc.1 + 1;
            let avg: i64 = (acc.0 + v["age"].as_i64().unwrap()) / nb_elements;
            (avg, nb_elements)
        });
        assert_eq!(folded, (27, 2));
    }
}
