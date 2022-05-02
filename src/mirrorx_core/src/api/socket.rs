use crate::provider::config::ConfigProvider;
use crate::provider::endpoint::EndPointProvider;
use crate::socket::endpoint::CacheKey;
use crate::socket::endpoint::EndPoint;
use crate::socket::message::client_to_client::ConnectRequest;
use crate::socket::message::client_to_client::KeyExchangeAndVerifyPasswordRequest;
use anyhow::anyhow;
use anyhow::bail;
use log::info;
use once_cell::sync::Lazy;
use rand::thread_rng;
use ring::rand::SecureRandom;
use rsa::BigUint;
use rsa::PublicKey;
use rsa::RsaPublicKey;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

static CONNECT_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub async fn desktop_connect(remote_device_id: String) -> anyhow::Result<()> {
    CONNECT_MUTEX.lock().await;

    if EndPointProvider::current()?.contains(&remote_device_id) {
        bail!(
            "desktop_connect: remote_device_id {} already connected",
            &remote_device_id
        );
    }

    let local_device_id = ConfigProvider::current()?
        .read_device_id()?
        .ok_or(anyhow::anyhow!("desktop_connect: local device id not set"))?;

    let endpoint = EndPoint::new(local_device_id, remote_device_id.clone());

    let resp = endpoint
        .desktop_connect(
            ConnectRequest {
                    // offer_device_id,
                    // ask_device_id,
                },
            Duration::from_secs(20),
        )
        .await?;

    let n = BigUint::from_bytes_le(&resp.pub_key_n);
    let e = BigUint::from_bytes_le(&resp.pub_key_e);

    let public_key = RsaPublicKey::new(n, e)?;

    endpoint
        .cache()
        .set(CacheKey::PasswordVerifyPublicKey, public_key);

    EndPointProvider::current()?.insert(remote_device_id, Arc::new(endpoint));

    Ok(())
}

pub async fn desktop_key_exchange_and_password_verify(
    remote_device_id: String,
    password: String,
) -> anyhow::Result<bool> {
    let endpoint = EndPointProvider::current()?
        .get(&remote_device_id)
        .ok_or(anyhow!(
            "desktop_key_exchange_and_password_verify: remote device '{}' already connected",
            &remote_device_id
        ))?;

    let ask_device_pub_key = endpoint
        .cache()
        .take::<RsaPublicKey>(CacheKey::PasswordVerifyPublicKey)
        .ok_or(anyhow!(
            "desktop_key_exchange_and_password_verify: verify password public key not exists"
        ))?;

    let mut rng = thread_rng();
    let password_secret = ask_device_pub_key
        .encrypt(
            &mut rng,
            rsa::PaddingScheme::PKCS1v15Encrypt,
            &password.as_bytes(),
        )
        .map_err(|err| {
            anyhow!(
                "desktop_key_exchange_and_password_verify: encrypt password failed: {}",
                err
            )
        })?;

    let ephemeral_rng = ring::rand::SystemRandom::new();
    let local_private_key =
        ring::agreement::EphemeralPrivateKey::generate(&ring::agreement::X25519, &ephemeral_rng)
            .map_err(|err| {
                anyhow!(
            "desktop_key_exchange_and_password_verify: generate ephemeral private key failed: {}",
            err
        )
            })?;

    let local_public_key = local_private_key.compute_public_key().map_err(|err| {
        anyhow!(
            "desktop_key_exchange_and_password_verify: compute public key failed: {}",
            err
        )
    })?;

    let exchange_pub_key = local_public_key.as_ref().to_vec();

    let mut exchange_salt = Vec::<u8>::with_capacity(32);
    ephemeral_rng.fill(&mut exchange_salt).map_err(|err| {
        anyhow!(
            "desktop_key_exchange_and_password_verify: generate exchange salt failed: {}",
            err
        )
    })?;

    let resp = endpoint
        .desktop_key_exchange_and_verify_password(
            KeyExchangeAndVerifyPasswordRequest {
                password_secret,
                exchange_pub_key,
                exchange_salt: exchange_salt.clone(),
            },
            Duration::from_secs(20),
        )
        .await?;

    if !resp.success {
        return Ok(false);
    }

    let remote_public_key =
        ring::agreement::UnparsedPublicKey::new(&ring::agreement::X25519, &resp.exchange_pub_key);

    let (send_key, recv_key) = ring::agreement::agree_ephemeral(
        local_private_key,
        &remote_public_key,
        ring::error::Unspecified,
        |key_material| {
            let send_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &exchange_salt)
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::with_capacity(32);
                    orm.fill(&mut key)
                })?;

            let recv_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &resp.exchange_salt)
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::with_capacity(32);
                    orm.fill(&mut key)
                })?;

            Ok((send_key, recv_key))
        },
    )
    .map_err(|err| {
        anyhow!(
            "desktop_key_exchange_and_password_verify: agree ephemeral key failed: {:?}",
            err
        )
    })?;

    info!("key exchange and password verify success");
    info!("send key: {:?}", send_key);
    info!("recv key: {:?}", recv_key);

    Ok(true)
}

// pub fn desktop_connect_offer(ask_device_id: String) -> anyhow::Result<bool, APIError> {
//     RUNTIME.block_on(async move {
//         let offer_device_id = match super::config::read_device_id()? {
//             Some(id) => id,
//             None => return Err(APIError::ConfigError),
//         };

//         // ask remote device
//         let resp = CLIENT
//             .call(
//                 Message::DesktopConnectOfferReq(DesktopConnectOfferReq {
//                     offer_device_id,
//                     ask_device_id: ask_device_id.to_owned(),
//                 }),
//                 Duration::from_secs(15),
//             )
//             .await
//             .map_err(|err| map_message_error(err))?;

//         let resp_message = match resp {
//             Message::DesktopConnectOfferResp(message) => message,
//             _ => return Err(APIError::InternalError),
//         };

//         // store remote password auth public key
//         if resp_message.agree {
//             let n = BigUint::from_bytes_le(resp_message.password_auth_public_key_n.as_ref());
//             let e = BigUint::from_bytes_le(resp_message.password_auth_public_key_e.as_ref());
//             let remote_password_auth_public_key = RsaPublicKey::new(n, e).map_err(|err| {
//                 error!("failed to create public key: {:?}", err);
//                 APIError::InternalError
//             })?;

//             let mut remote_password_auth_public_key_map =
//                 REMOTE_PASSWORD_AUTH_PUBLIC_KEY_MAP.lock().unwrap();
//             remote_password_auth_public_key_map
//                 .insert(ask_device_id.to_owned(), remote_password_auth_public_key);
//             drop(remote_password_auth_public_key_map);
//         }

//         Ok(resp_message.agree)
//     })
// }

// pub fn desktop_connect_offer_auth_password(
//     ask_device_id: String,
//     device_password: String,
// ) -> anyhow::Result<bool, APIError> {
//     RUNTIME.block_on(async move {
//         let offer_device_id = match super::config::read_device_id()? {
//             Some(id) => id,
//             None => return Err(APIError::ConfigError),
//         };

//         let mut remote_password_auth_public_key_map =
//             crate::constant::REMOTE_PASSWORD_AUTH_PUBLIC_KEY_MAP
//                 .lock()
//                 .unwrap();
//         let remote_password_auth_public_key =
//             match remote_password_auth_public_key_map.remove(&ask_device_id) {
//                 Some(key) => key,
//                 None => {
//                     error!("remote_password_auth_public_key is None");
//                     return Err(APIError::InternalError);
//                 }
//             };
//         drop(remote_password_auth_public_key_map);

//         let secret_message = remote_password_auth_public_key
//             .encrypt(
//                 &mut rand::rngs::OsRng,
//                 PaddingScheme::PKCS1v15Encrypt,
//                 &device_password.as_bytes(),
//             )
//             .map_err(|err| {
//                 error!("failed to encrypt device password: {:?}", err);
//                 APIError::InternalError
//             })?;

//         let resp = CLIENT
//             .call(
//                 Message::DesktopConnectOfferAuthReq(DesktopConnectOfferAuthReq {
//                     offer_device_id,
//                     ask_device_id,
//                     secret_message,
//                 }),
//                 Duration::from_secs(10),
//             )
//             .await
//             .map_err(|err| map_message_error(err))?;

//         let resp_message = match resp {
//             Message::DesktopConnectOfferAuthResp(message) => message,
//             _ => return Err(APIError::InternalError),
//         };

//         Ok(resp_message.password_correct)
//     })
// }

// pub fn desktop_connect_open_stream(ask_device_id: String) -> anyhow::Result<(), APIError> {
//     RUNTIME.block_on(async move {
//         let offer_device_id = match super::config::read_device_id()? {
//             Some(id) => id,
//             None => return Err(APIError::ConfigError),
//         };

//         let resp = CLIENT
//             .call(
//                 Message::DesktopConnectOpenStreamReq(DesktopConnectOpenStreamReq {
//                     offer_device_id,
//                     ask_device_id,
//                 }),
//                 Duration::from_secs(10),
//             )
//             .await
//             .map_err(|err| map_message_error(err))?;

//         Ok(())
//     })
// }

// fn map_message_error(message_error: MessageError) -> APIError {
//     match message_error {
//         MessageError::InternalError | MessageError::MismatchedResponseMessage => {
//             APIError::InternalError
//         }
//         MessageError::Timeout => APIError::Timeout,
//         MessageError::InvalidArguments => APIError::InvalidArguments,
//         MessageError::RemoteClientOfflineOrNotExist => APIError::RemoteClientOfflineOrNotExist,
//     }
// }
