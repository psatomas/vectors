fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushrooms = String::from("Mushrooms");
    let sausage = String::from("Sausage");
    let pizza_toppings = vec![pepperoni, mushrooms, sausage];

    let option = pizza_toppings.get(2);

    match option {
        Some(topping) => println!("The topping is: {topping}"),
        None => println!("No value at that index position")
    }
}