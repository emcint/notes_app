use std::{hash::Hash, path::PathBuf, fmt::Debug};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn authenticate(user_password: String, path: PathBuf) -> bool {

    let user_password_bytes = user_password.as_bytes();

    let hash_string = std::fs::read_to_string(path).expect("Unable to read file");
    let hash = argon2::PasswordHash::new(&hash_string).expect("Unable to parse hash");
    let result = match Argon2::default().verify_password(user_password_bytes, &hash) {
        Ok(_) => true,
        Err(_) => false,
    };
    return result;
    
    

    
}

pub fn generate_new(password: String) -> String {

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password");

    return hashed_password.to_string();
}
