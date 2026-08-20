use crate::domain::DomainError;
use argon2::password_hash::SaltString;
use argon2::{
    Algorithm, Argon2, Params, PasswordHash as Argon2PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PasswordHash(pub String); // or Vec<u8>

impl PasswordHash {
    pub fn parse(s: &str) -> Result<Self, DomainError> {
        Argon2PasswordHash::new(s).map_err(|_| DomainError::InvalidPasswordHash)?;

        Ok(PasswordHash(s.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

const ARGON2_ALG: Algorithm = Algorithm::Argon2id;
const ARGON2_VERSION: Version = Version::V0x13;
const ARGON2_MEMORY_COST: u32 = 19_456;
const ARGON2_TIME_COST: u32 = 2;
const ARGON2_PARALLELISM: u32 = 1;
const ARGON2_HASH_LENGTH: usize = 32;
const SALT_LENGTH: usize = 16;

fn generate_salt() -> SaltString {
    let mut bytes = [0u8; SALT_LENGTH];
    getrandom::fill(&mut bytes).expect("OS randomness must be available");
    SaltString::encode_b64(&bytes).expect("valid salt bytes must encode")
}

fn argon2_ctx() -> Argon2<'static> {
    let params = Params::new(
        ARGON2_MEMORY_COST,
        ARGON2_TIME_COST,
        ARGON2_PARALLELISM,
        Some(ARGON2_HASH_LENGTH),
    )
    .expect("valid argon2 params");

    Argon2::new(ARGON2_ALG, ARGON2_VERSION, params)
}

pub fn needs_rehash(hash: &PasswordHash) -> bool {
    let parsed = Argon2PasswordHash::new(hash.as_str())
        .map_err(|_| DomainError::InvalidPasswordHash)
        .unwrap();

    if parsed.algorithm != ARGON2_ALG.ident() {
        return true;
    }

    if parsed.version != Some(ARGON2_VERSION.into()) {
        return true;
    }

    let params = Params::try_from(&parsed)
        .map_err(|_| DomainError::InvalidPasswordHash)
        .unwrap();

    if params.m_cost() < ARGON2_MEMORY_COST {
        return true;
    }

    // time cost
    if params.t_cost() < ARGON2_TIME_COST {
        return true;
    }

    // parallelism
    if params.p_cost() < ARGON2_PARALLELISM {
        return true;
    }

    false
}

pub fn hash_password(password: &str) -> PasswordHash {
    let salt = generate_salt();
    let password_hash = argon2_ctx()
        .hash_password(password.as_bytes(), &salt)
        .expect("argon2 hashing must succeed")
        .to_string();

    PasswordHash(password_hash)
}

pub enum PasswordCheck {
    Verified,
    VerifiedAndNeedsRehash,
}

pub fn verify_password(password: &str, hash: &PasswordHash) -> Result<PasswordCheck, DomainError> {
    let parsed_hash =
        Argon2PasswordHash::new(&hash.as_str()).map_err(|_| DomainError::InvalidPasswordHash)?;

    argon2_ctx()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| DomainError::InvalidPassword)?;

    if needs_rehash(hash) {
        Ok(PasswordCheck::VerifiedAndNeedsRehash)
    } else {
        Ok(PasswordCheck::Verified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "password123";
        let hash = hash_password(password);
        let result = verify_password(password, &hash).unwrap();
        matches!(result, PasswordCheck::Verified);
    }

    #[test]
    fn test_needs_rehash_older_params() {
        let password = "password123";
        let salt = generate_salt();

        // intentionally use lower cost params
        let params = Params::new(1024, 1, 1, Some(ARGON2_HASH_LENGTH)).unwrap();
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hash = PasswordHash(password_hash);
        assert!(needs_rehash(&hash));

        let result = verify_password(password, &hash).unwrap();
        assert!(matches!(result, PasswordCheck::VerifiedAndNeedsRehash));
    }

    #[test]
    fn test_needs_rehash_different_algorithm() {
        let password = "password123";
        let salt = generate_salt();

        let params = Params::new(
            ARGON2_MEMORY_COST,
            ARGON2_TIME_COST,
            ARGON2_PARALLELISM,
            Some(ARGON2_HASH_LENGTH),
        )
        .unwrap();
        let argon2 = Argon2::new(Algorithm::Argon2i, Version::V0x13, params); // Argon2i instead of Argon2id
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hash = PasswordHash(password_hash);
        assert!(needs_rehash(&hash));
    }
}
