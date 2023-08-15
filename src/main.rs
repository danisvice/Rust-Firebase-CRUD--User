use std::collections::HashMap;
use firebase_rs::*;
use serde::{Deserialize, Serialize};

struct User{
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    name: String,
}

#[tokio::main]
async fn main(){
    let user = User {
        name: "Danny Ortiz".to_string(),
        age: 25,
        email: "d.ortixe@icloud.com".to_string(),
    };

    let firebase = Firebase::new("https://rust-crud-b3c56-default-rtdb.firebaseio.com/").unwrap();

    let response = set_user(&firebase, &user).await;

    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    let users = get_users(&firebase).await;
    println!("{:?}", users);

    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &response, &user).await;
    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await;
    println!("User Deleted");
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}", users);
}

async fn get_user() -> User{

}

async fn update_user() -> User{

}

async fn delete_user(){

}

//convert string to response
fn string_to_response(s: &str) -> Response{
    serde_json::from_reader(s).unwrap()
}

//convert string to a user
fn string_to_user(s: &str) -> User{
    serde_json::from_reader(s).unwrap()
}