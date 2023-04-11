
pub fn authenticate(user_password: String) -> bool {
    let password = "password".to_string(); // load file todo!!!!!
    if user_password == password {
        return true;
    } else {
        return false;
    }
}
