extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    sex: String,
}

fn main() {
    let json_str = r#"
        {
            "name": "ekusiadadus",
            "age": 24,
            "sex": "male"
        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let p: Person = res.unwrap();
        println!("The name is {}", p.name);
        println!("The age is {}", p.age);
        println!("The sex is {}", p.sex);
    } else {
        println!("Sorry! Could not parse JSON :(");
    }
}
