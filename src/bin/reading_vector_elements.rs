fn main() {
    let pizza_diameters = vec![8, 10, 12, 14];

    let pepperoni = String::from("Pepperoni");
    let mushrooms = String::from("Mushrooms");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushrooms, sausage];

    let pizza_slice = &pizza_toppings[1..];
    println!("{pizza_slice:?}");
}