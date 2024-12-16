fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let slice = &vec[..];
    // The following line will cause a panic if vec is modified after creating the slice
    println!("The first element is: {}", slice[0]);
    vec.push(3);
}