use std::env;

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn mul(a: i32, b: i32) -> i32 {
    a * b
}
fn diff(a: i32, b: i32) -> i32 {
    a - b
}
fn div(a: i32, b: i32) -> i32 {
    a / b
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_works() {
        let res = super::rpn(vec!["1".to_string(), "2".to_string(), "+".to_string()]);
        assert_eq!(res, 3);
    }
}

fn calculate(operation: String, a: i32, b: i32) -> i32 {
    match operation.as_str() {
        "+" => return sum(a, b),
        "-" => return diff(a, b),
        "/" => return div(a, b),
        "*" => return mul(a, b),
        _ => {
            panic!("Error - unkown operation.");
        }
    }
}

fn rpn(args: Vec<String>) -> i32 {
    let sign: [String; 4] = [
        String::from("+"),
        String::from("-"),
        String::from("/"),
        String::from("*"),
    ];
    let mut values: Vec<i32> = Vec::new();

    for (pos, e) in args.iter().enumerate() {
        println!("Element at position {}: {:?}", pos, e);
        // only the params
        if sign.contains(&e) {
            // that's an operation do it and stack
            //TODO check the values length
            let v1 = values[values.len() - 1];
            let v2 = values[values.len() - 2];
            let r = calculate(String::from(e), v1, v2);
            values.pop();
            values.pop();
            values.push(r);
        } else {
            let v = e.parse().unwrap();
            values.push(v)
        }
    }
    return values[0];
}

fn main() {
    // Read var
    let args: Vec<String> = env::args().collect();
    let values = &args[1..].to_vec();
    let res = rpn(values.to_vec());
    println!("result: {}", res);
}
