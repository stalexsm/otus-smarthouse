use patterns::newtype::Kilometers;

fn main() {
    // Create an instance of `Kilometers`
    let distance = Kilometers::new(5);

    // Call the `to_meters` method of `Kilometers` to convert to `Meters`
    let meters = distance.to_meters();

    // Print the converted value
    println!(
        "{} kilometers is equal to {} meters",
        distance.get_value(),
        meters.get_value()
    );
}
