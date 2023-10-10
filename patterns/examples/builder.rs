use patterns::builder::PersonBuilder;

fn main() {
    let person = PersonBuilder::new()
        .name("John")
        .age(30)
        .address("123 Main St.")
        .build();

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Address: {}", person.address);
}
