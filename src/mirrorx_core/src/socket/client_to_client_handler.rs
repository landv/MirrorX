use super::{
    endpoint::EndPoint,
    message::client_to_client::{
        ConnectReply, ConnectRequest, KeyExchangeAndVerifyPasswordRequest,
    },
};
use crate::{
    provider::config::ConfigProvider,
    socket::{endpoint::CacheKey, message::client_to_client::KeyExchangeAndVerifyPasswordReply},
};
use anyhow::anyhow;
use log::info;
use ring::rand::SecureRandom;
use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use std::sync::Arc;

pub async fn connect(endpoint: Arc<EndPoint>, req: ConnectRequest) -> anyhow::Result<ConnectReply> {
    info!("connect: {:?}", req);

    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 4096)?;
    let pub_key = RsaPublicKey::from(&priv_key);
    let pub_key_n = pub_key.n().to_bytes_le();
    let pub_key_e = pub_key.e().to_bytes_le();

    endpoint
        .cache()
        .set(CacheKey::PasswordVerifyPrivateKey, priv_key);

    Ok(ConnectReply {
        pub_key_n,
        pub_key_e,
    })
}

pub async fn key_exchange_and_verify_password(
    endpoint: Arc<EndPoint>,
    req: KeyExchangeAndVerifyPasswordRequest,
) -> anyhow::Result<KeyExchangeAndVerifyPasswordReply> {
    info!("key_exchange_and_verify_password: {:?}", req);

    // todo: check white list

    let password = ConfigProvider::current()?
        .read_device_password()?
        .ok_or(anyhow!(
            "key_exchange_and_verify_password: local password not set, refuse request"
        ))?;

    let priv_key = endpoint
        .cache()
        .take::<RsaPrivateKey>(CacheKey::PasswordVerifyPrivateKey)
        .ok_or(anyhow::anyhow!(
            "key_exchange_and_verify_password: no private key found"
        ))?;

    let req_password = priv_key.decrypt(PaddingScheme::PKCS1v15Encrypt, &req.password_secret)?;

    if req_password != Vec::from(password) {
        return Ok(KeyExchangeAndVerifyPasswordReply {
            success: false,
            ..KeyExchangeAndVerifyPasswordReply::default()
        });
    }

    // gen key exchange
    let ephemeral_rng = ring::rand::SystemRandom::new();
    let local_private_key =
        ring::agreement::EphemeralPrivateKey::generate(&ring::agreement::X25519, &ephemeral_rng)
            .map_err(|err| anyhow!(err))?;

    let local_public_key = local_private_key
        .compute_public_key()
        .map_err(|err| anyhow::anyhow!(err))?;

    let exchange_pub_key = local_public_key.as_ref().to_vec();

    let mut exchange_salt = Vec::<u8>::with_capacity(32);
    ephemeral_rng
        .fill(&mut exchange_salt)
        .map_err(|err| anyhow::anyhow!(err))?;

    let remote_public_key =
        ring::agreement::UnparsedPublicKey::new(&ring::agreement::X25519, &req.exchange_pub_key);

    let (send_key, recv_key) = ring::agreement::agree_ephemeral(
        local_private_key,
        &remote_public_key,
        ring::error::Unspecified,
        |key_material| {
            let send_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &req.exchange_salt)
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::with_capacity(32);
                    orm.fill(&mut key)?;
                    Ok(key)
                })?;

            let recv_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &exchange_salt)
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::with_capacity(32);
                    orm.fill(&mut key)?;
                    Ok(key)
                })?;

            Ok((send_key, recv_key))
        },
    )
    .map_err(|err| anyhow::anyhow!(err))?;

    Ok(KeyExchangeAndVerifyPasswordReply {
        success: true,
        exchange_pub_key,
        exchange_salt,
    })
}