use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::utils::jwt::{create_jwt, Role};

pub fn login(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    assert!(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());
    parsed_hash.to_string();
    create_jwt("prince!", &Role::from_str("Admin"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_login() {
        let password = "password";
        let hashed_password = login(password);
        assert!(Argon2::default()
            .verify_password(
                password.as_bytes(),
                &PasswordHash::new(&hashed_password).unwrap()
            )
            .is_ok());
    }
}
