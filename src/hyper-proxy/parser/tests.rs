use crate::parser::models::{RBAC, User};
use std::fs::File;
use std::io::Read;
use std::env;

pub struct TestUser {
    pub id: &'static str,
    pub status: &'static str,
}

#[test]
fn test_rbac_config_file() {

    let rbac = RBAC::read_config_from_file("config.json").unwrap();
    assert!(rbac.roles == vec!["guest", "user", "admin", "superuser"]);

}

#[test]
fn test_open_directly_from_file_config() {

    let mut file = File::open("config.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let rbac: RBAC = serde_json::from_str(&buff).unwrap();
    assert!(rbac.roles[0] == "guest");
}

#[test]
fn test_json_string() {

    let config = r#" {
          "roles": ["guest", "user", "admin", "superuser"],

          "permissions": ["Create", "Read", "Update", "Delete"],

          "grants": { "guest" : ["Read"],
                      "user": ["Read", "CreateOwnData", "UpdateOwnData"],
                      "admin": ["Read","CreateUser", "DeleteUser", "DeleteAny", "UpdateRole"],
                      "superuser": ["Any", "CreateAdmin"]
                    }
        }
    "#;

    let rbac: RBAC = serde_json::from_str::<RBAC>(config).unwrap();
    assert!( rbac.roles == vec!["guest", "user", "admin", "superuser"]);
}

#[test]
fn user_check_access() {
    let user = TestUser {
        id: "10001",
        status: "admin",
    };


    let rbac = RBAC::read_config_from_file("config.json").unwrap();
    assert!(rbac.check_access(user.id, user.status, "CreateUser"), true);
    assert!(rbac.check_access(user.id, user.status, "createuser"), false);
//    assert!(rbac.check_access(user.id, "hacker", "create"), false);

}

