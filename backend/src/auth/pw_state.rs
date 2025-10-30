use argon2::password_hash::SaltString;
use centaurus::{auth::pw::PasswordState, db::init::Connection};
use rsa::{
  RsaPrivateKey,
  pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey},
  pkcs8::LineEnding,
  rand_core::OsRng,
};
use tracing::info;
use uuid::Uuid;

use crate::{config::EnvConfig, db::DBTrait};

const PW_KEY: &str = "password";
#[cfg(not(any(test, feature = "test")))]
pub const KEY_SIZE: usize = 4096;
#[cfg(any(test, feature = "test"))]
pub const KEY_SIZE: usize = 512;

pub async fn init_pw_state(config: &EnvConfig, db: &Connection) -> PasswordState {
  let key = if let Ok(key) = db.key().get_key_by_name(PW_KEY.into()).await {
    RsaPrivateKey::from_pkcs1_pem(&key.private_key).expect("Failed to parse private password key")
  } else {
    info!(
      "Generating new RSA key for password encryption with key size {KEY_SIZE}, this may take a while..."
    );
    let mut rng = OsRng {};
    let private_key = RsaPrivateKey::new(&mut rng, KEY_SIZE).expect("Failed to create Rsa key");
    let key = private_key
      .to_pkcs1_pem(LineEnding::CRLF)
      .expect("Failed to export private key")
      .to_string();

    db.key()
      .create_key("password".into(), key.clone(), Uuid::new_v4())
      .await
      .expect("Failed to save key");

    private_key
  };

  let pepper = config.auth.auth_pepper.as_bytes().to_vec();
  let state = PasswordState::init(pepper, key).await;

  // initial user setup
  let user_count = db
    .user()
    .list_users()
    .await
    .expect("Failed to list users")
    .len();
  if user_count == 0 || config.auth.overwrite_initial_user {
    let salt = SaltString::generate(OsRng {}).to_string();
    let password = state
      .pw_hash_raw(&salt, &config.auth.initial_user_password)
      .expect("Failed to hash initial password");

    let user = entity::user::Model {
      id: Uuid::new_v4(),
      name: config.auth.initial_user_username.clone(),
      password,
      salt,
    };

    if let Ok(user) = db.user().get_user_by_name(user.name.clone()).await {
      db.user()
        .delete_user(user.id)
        .await
        .expect("Failed to overwrite initial user");

      info!(
        "Initial user '{}' deleted",
        config.auth.initial_user_username
      );
    }

    db.user()
      .create_user(user)
      .await
      .expect("Failed to create initial user");

    info!(
      "Initial user '{}' created",
      config.auth.initial_user_username
    );
  } else {
    info!("Users already exist, skipping initial user creation");
  }

  state
}
