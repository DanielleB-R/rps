use rps::db;

fn main() {
    let connection = db::establish_connection();

    let user = db::create_user(&connection, "danielle", "Danielle", "she/her", 38)
        .expect("Insert should succeed");

    println!("{:?}", user);

    let user_again = db::get_user_by_id(&connection, user.id).expect("Row should exist");

    println!("{:?}", user_again);
}
