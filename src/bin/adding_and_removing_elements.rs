fn main() {
    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    pizza_diameters.push(18);

    pizza_diameters.insert(0, 4);

    let last_pizza_diameter = pizza_diameters.pop();
    println!("{last_pizza_diameter:?}");

    let third_diameter_from_start = pizza_diameters.remove(2);
    println!("{third_diameter_from_start:?}");

    println!("{pizza_diameters:?}");
}    