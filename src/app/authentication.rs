use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn authenticate(user_password: String) -> bool {

    let password = "password".to_string();

    user_password == password
}

// pub fn generate_new(password: String) -> String {
    
//         let password = "password";
    
//         let salt = SaltString::generate(&mut OsRng);
    
//         let hash = Argon2::default()
//             .hash_password(password.as_bytes(), salt.as_ref())
//             .unwrap()
//             .to_string();
    
//         println!("
    
//     {}", hash);
    
// }