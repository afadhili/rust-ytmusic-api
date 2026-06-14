use serde_json::Value;

fn collect_key<'a>(data: &'a Value, key: &str, dead_end: bool, out: &mut Vec<&'a Value>) {
    match data {
        Value::Object(map) => {
            if let Some(value) = map.get(key) {
                out.push(value);
                if dead_end {
                    return;
                }
            }
            for value in map.values() {
                collect_key(value, key, false, out);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_key(item, key, false, out);
            }
        }
        _ => {}
    }
}

pub fn traverse_list<'a>(data: &'a Value, keys: &[&str]) -> Vec<&'a Value> {
    let mut current = vec![data];
    for (index, key) in keys.iter().enumerate() {
        let dead_end = index == keys.len() - 1;
        let mut next = Vec::new();
        for value in current {
            collect_key(value, key, dead_end, &mut next);
        }
        current = next;
        if current.is_empty() {
            break;
        }
    }
    current
}

pub fn traverse_first<'a>(data: &'a Value, keys: &[&str]) -> Option<&'a Value> {
    traverse_list(data, keys).into_iter().next()
}

pub fn traverse_string(data: &Value, keys: &[&str]) -> String {
    traverse_first(data, keys)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

pub fn strings(data: &Value, keys: &[&str]) -> Vec<String> {
    traverse_list(data, keys)
        .into_iter()
        .filter_map(|v| v.as_str().map(ToString::to_string))
        .collect()
}
