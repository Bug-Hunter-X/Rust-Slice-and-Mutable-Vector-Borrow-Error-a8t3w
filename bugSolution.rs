fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Create a clone of the vector to avoid issues with mutability
    let slice = vec.clone()[..];
    println!("The first element is: {}", slice[0]);

    vec.push(3);
    println!("Modified vector: {:?}", vec);
} 