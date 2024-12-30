use csv_example::Csv;

#[derive(Csv, Debug)]
struct Person {
    id: u64,
    name: String,
    age: u8,
}

fn main() {
    let instance = Person {
        id: 1,
        name: "AZ".to_string(),
        age: 50,
    };

    let csv_string = instance.to_csv_string();
    println!("Csv: {csv_string}");

    let new_instance = Person::new_from_string(&csv_string);
    println!("New object: {:?}", new_instance);
}
