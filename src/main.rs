#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod models;

fn main() {
    let user = models::User {
        name: "Andre".to_string(),
        group: Some(models::Group {
            group_name: "Admin".to_string(),
            allowed_verbs: vec!["GET".to_string(), "POST".to_string(), "DELETE".to_string()],
        }),
        id: 10001,
    };

    let jjson = serde_json::to_string(&user).expect("Couldn't serialize config");
    println!("{}", jjson);
}
