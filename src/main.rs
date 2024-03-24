#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema; // Ensure this points to the correct schema module

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use schema::users;


// Define a struct to represent your table
#[derive(Queryable)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    name: &'a str,
    email: &'a str,
}

// Define a struct to represent your table
#[derive(Queryable)]
struct Friends {
    id: i32,
    name: String,
    activity: String,
}

// Load environment variables from .env file
fn load_env() {
    dotenv().ok();
}

// Establish database connection
fn establish_connection() -> MysqlConnection {
    load_env();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Function to get all users from the database
fn get_users(conn: &MysqlConnection) -> Vec<User> {
    use crate::schema::users::dsl::*; // Update to use the correct table

    users.load::<User>(conn).expect("Error loading users")
}

// Function to get all friends from the database
fn get_friends(conn: &MysqlConnection) -> Vec<Friends> {
    use crate::schema::friends::dsl::*;
    friends.load::<Friends>(conn).expect("Error loading friends")
}

fn insert_user(conn: &MysqlConnection, name: &str, email: &str){
    use schema::users;
    let user = NewUser{
        name,
        email
    };
    diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
        .expect("Error in inserting user to table");
}

fn update_user(conn: &MysqlConnection, user_id: i32, name: &str, email: &str) {
    diesel::update(users::table.find(user_id))
        .set((users::name.eq(name), users::email.eq(email)))
        .execute(conn)
        .expect("Error updating user");
}

fn delete_user(conn: &MysqlConnection, user_id: i32) {
    diesel::delete(users::table.find(user_id))
        .execute(conn)
        .expect("Error deleting user");
}


// Main function
fn main() {
    // Establish database connection
    let connection = establish_connection();

    // Get all users from the database
    let users = get_users(&connection);
    // Print users
    for user in users {
        println!("ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
    }


    // // Get all friends
    // let friends = get_friends(&connection);
    // for friend in friends {
    //     println!("ID: {}, Name: {}, Activity: {}", friend.id, friend.name, friend.activity);
    // }

    // Insert a user
    insert_user(&connection, "John Doe", "john@example.com");
    
    // Get all users from the database
    let users = get_users(&connection);
    // Print users
    for user in users {
        println!("ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
    }



}
