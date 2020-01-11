#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    user: String,
    number: u32,
}

fn main() {

    let user = User {
        user : "aohno".to_owned(),
        number : 125
    };
    let json_str = serde_json::to_string(&user).unwrap();

    println!("{}", json_str);
}
