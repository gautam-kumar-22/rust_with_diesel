// @generated automatically by Diesel CLI.

diesel::table! {
    Department (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    Employee (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        departmentID -> Nullable<Integer>,
        salary -> Nullable<Integer>,
    }
}

diesel::table! {
    friends (id) {
        id -> Integer,
        name -> Varchar,
        activity -> Varchar,
    }
}

diesel::table! {
    purchase (purchase_id) {
        purchase_id -> Integer,
        user_id -> Nullable<Integer>,
        purchase_date -> Date,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(Employee -> Department (departmentID));

diesel::allow_tables_to_appear_in_same_query!(
    Department,
    Employee,
    friends,
    purchase,
    users,
);
