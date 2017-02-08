// fn double_first(vec: Vec<&str>) -> i32 {
//     let first = vec.first().unwrap(); // Generate error 1
//     2 * first.parse::<i32>().unwrap() // Generate error 2
// }

type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or("please input a none empty vector".to_owned())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.to_string()).map(|i| 2 * i))
}

fn print(result:Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
    // println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty

    // println!("The first doubled is {}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}
