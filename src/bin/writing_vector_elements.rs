fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushrooms = String::from("Mushrooms");
    let sausage = String::from("Sausage");
    let mut pizza_toppings = vec![pepperoni, mushrooms, sausage];
    
    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:#?}");

    let target_topping = &mut pizza_toppings[2];
    target_topping.push_str(" and Meatballs");
    let another_topping = &pizza_toppings[2];
    let another_one = &pizza_toppings[2];
    println!("{another_topping} {another_one}");
    println!("{pizza_toppings:#?}");
}