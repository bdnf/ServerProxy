use crate::parser::models::{RBAC, User};
use std::fs::File;
use std::io::Read;
use std::env;
use std::collections::HashMap;

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
                    },
           "allowed_admins" : ["43857938475", "43857938435"]
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
    assert_eq!(rbac.check_access(user.id, user.status, "createuser"), false);
    assert_eq!(rbac.check_access(user.id, "hacker", "create"), false);

}

#[test]
fn user_check_no_access() {
    let user = TestUser {
        id: "10001",
        status: "admin",
    };


    let rbac = RBAC::read_config_from_file("config.json").unwrap();

    //assert!(rbac.check_access(user.id, user.status, "createuser"), None);
    assert_eq!(rbac.check_access(user.id, "hacker", "create"), false);

}


#[test]
fn add_admin() {
    let mut rbac = RBAC::read_config_from_file("config.json").unwrap();
    rbac.add_user_roles("123", "JHJFGFS", "Admin");
    assert_eq!( rbac.allowed_admins.contains("123") , true );

}

#[test]
fn parse_request_fields() {


    let mut test_string = "{\"username\": \"testname8\", \"password\": \"paswrd\", \"email\": \"test@email.com\"}".to_string();
    test_string.retain(|c| (c != '"') & (c != '{') && (c != '}') && (c != ' ') );

    let v: Vec<&str> = test_string.split(',').collect();
    let v2 = v.into_iter()
        .map(|kv| kv.split(':').collect::<Vec<&str>>())
        .map(|vec| { (vec[0], vec[1])  })
        .collect::<HashMap<_, _>>();

    assert_eq!( v2.get(&"username"), Some(&"testname8") );
    assert_eq!( v2.get(&"password"), Some(&"paswrd") );
    assert_eq!( v2.get(&"email"), Some(&"test@email.com") );

}

