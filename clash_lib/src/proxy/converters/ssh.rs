use russh::keys::{EcdsaCurve, HashAlg};

use crate::{
    config::internal::proxy::OutBoundSsh,
    proxy::{
        ssh::{Handler, HandlerOptions},
        HandlerCommonOptions,
    },
};

impl TryFrom<OutBoundSsh> for Handler {
    type Error = crate::Error;

    fn try_from(value: OutBoundSsh) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

/// supported host key algorithms:
///   * `ssh-ed25519`
///   * `rsa-sha2-256`
///   * `rsa-sha2-512`
///   * `ssh-rsa` ✨
///   * `ecdsa-sha2-nistp256` ✨
///   * `ecdsa-sha2-nistp384` ✨
///   * `ecdsa-sha2-nistp521` ✨
fn str_to_algo(value: &str) -> Option<russh::keys::Algorithm> {
    match value {
        "ssh-ed25519" | "ed25519" => Some(russh::keys::Algorithm::Ed25519),
        "rsa-sha2-256" => Some(russh::keys::Algorithm::Rsa {
            hash: Some(HashAlg::Sha256),
        }),
        "rsa-sha2-512" => Some(russh::keys::Algorithm::Rsa {
            hash: Some(HashAlg::Sha512),
        }),
        "ssh-rsa" | "rsa" => Some(russh::keys::Algorithm::Rsa { hash: None }),
        "ecdsa-sha2-nistp256" => Some(russh::keys::Algorithm::Ecdsa {
            curve: EcdsaCurve::NistP256,
        }),
        "ecdsa-sha2-nistp384" => Some(russh::keys::Algorithm::Ecdsa {
            curve: EcdsaCurve::NistP384,
        }),
        "ecdsa-sha2-nistp521" => Some(russh::keys::Algorithm::Ecdsa {
            curve: EcdsaCurve::NistP521,
        }),
        _ => None,
    }
}

impl TryFrom<&OutBoundSsh> for Handler {
    type Error = crate::Error;

    fn try_from(s: &OutBoundSsh) -> Result<Self, Self::Error> {
        let host_key_algorithms = s.host_key_algorithms.clone().map(|algos| {
            algos
                .iter()
                .filter_map(|s| str_to_algo(s))
                .collect::<Vec<_>>()
        });
        let h = Handler::new(HandlerOptions {
            name: s.common_opts.name.to_owned(),
            common_opts: HandlerCommonOptions {
                connector: s.common_opts.connect_via.clone(),
                ..Default::default()
            },
            server: s.common_opts.server.to_owned(),
            username: s.username.clone(),
            port: s.common_opts.port,
            password: s.password.clone(),
            private_key: s.private_key.clone(),
            private_key_passphrase: s.private_key_passphrase.clone(),
            host_key: s.host_key.clone(),
            host_key_algorithms,
        });

        Ok(h)
    }
}
