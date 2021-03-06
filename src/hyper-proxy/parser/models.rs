use std::error::Error;
use std::fs::File;
use std::path::Path;

use std::collections::{HashMap, HashSet};
use json::JsonValue;

#[derive(Serialize, Deserialize,Default, Debug)]
pub struct User {
    pub name: String,
    pub group: Option<Group>,
    pub id: u32,
}

pub struct UserRole {

    pub id: String,
    pub token: Option<String>,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub group_name: String,
    pub allowed_verbs: Vec<String>,
}


#[derive(Debug)]
pub struct Role {
    pub name: String,
    pub rule: Option<String>,
    pub data: Vec<String>,
    pub item_type: i64,
}

#[derive(Serialize, Deserialize,Default, Debug)]
pub struct Actions {
    pub role: String,
//    pub user: Vec<String>,
//    pub admin: Vec<String>,
//    pub superuser: Vec<String>,
}


//Data
#[derive(Serialize, Deserialize,Default, Debug)]
pub struct RBAC {
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    //pub grants: Actions,
    pub grants: HashMap<String, Vec<String>>,
    pub allowed_admins: HashSet<String>,

}

impl RBAC {
//    pub fn new() -> Self {
//        RBAC {
//            roles: HashMap::new(),
//            permissions: HashMap::new(),
//            grants: HashMap::new(),
//            //parents: HashMap::new()
//        }
//    }

    pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<Error>> {
     // Open the file in read-only mode.
     let file = File::open(path)?;
     let rbac = serde_json::from_reader(file)?;
     Ok(rbac)
    }

    pub fn check_access(&self, user_id: &'static str, user_status: &'static str, action: &'static str ) -> bool {

        let actions: Option<&Vec<String>> = self.grants.get(user_status);//.ok_or(false);
        //println!("Actions are: {:?}", actions);
        match actions {
            Some(x) => x.contains(&action.to_string()),
            None => false,
        }

        //println!("{:?}", actions);

        //println!("{:?}", res);
        //res

    }


    pub fn check_token(token:&'static str) -> bool {

        false
    }

    pub fn add_user_roles(&mut self, user_id: &'static str, token: &'static str, role:&'static str) -> (){
        //check id
        //check token

        if (role.to_ascii_lowercase() == "admin") {
            self.allowed_admins.insert(user_id.to_string());
            println!("User {} added as admin successfully", user_id);
        }

        //and update
//        UserRole{
//            id: user_id.to_string(),
//            token: Some(token.to_string()),
//            role: Some(role.to_string())
//        }
    }

    pub fn is_allowed(&self, user_id: &'static str, action: &'static str) -> bool {
        //if .get(name) if user.token == token

        false
    }
}




//#[test]
//#[should panic]
//#[ignore]

//#[test]
//fn test_data() {
//    //models::Data::new();
//    let rbac = models::RBAC::new();
//}