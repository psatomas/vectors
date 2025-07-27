fn main() {
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");
    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");

    println!(
        "Length: {}, Capacity: {}",
        seasons.len(),
        seasons.capacity()
    );
}