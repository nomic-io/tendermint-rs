//! LightNode Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// LightNode Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LightNodeConfig {
    /// RPC address to request headers and validators from.
    pub rpc_address: String,
    /// The duration until we consider a trusted state as expired.
    pub trusting_period: Duration,
    /// Subjective initialization.
    pub subjective_init: SubjectiveInit,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on LightNodeConfig instead.
impl Default for LightNodeConfig {
    fn default() -> Self {
        Self {
            rpc_address: "localhost:26657".to_owned(),
            trusting_period: Duration::new(6000, 0),
            subjective_init: SubjectiveInit::default(),
        }
    }
}

/// Configuration for subjective initialization.
///
/// Contains the subjective height and validators hash (as a string formatted as hex).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubjectiveInit {
    /// Subjective height.
    pub height: u64,
    /// Subjective validators hash.
    pub validators_hash: String,
}

impl Default for SubjectiveInit {
    fn default() -> Self {
        Self {
            height: 1,
            // TODO(liamsi): a default hash here does not make sense unless it is a valid hash
            // from a public network
            // This hash is for the interchainio/tendermint custom docker image in CI.
            validators_hash: "3FE2453BB45CADB9E80BBD655870EA33677756EC43D0A656448C829185AB0FBF"
                .to_owned(),
        }
    }
}
