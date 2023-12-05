#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    clippy::derive_partial_eq_without_eq
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod traits;
mod type_names;

pub use prost;
pub use prost_types::Any;
pub use tendermint_proto as tendermint;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("prost/cosmos-sdk/COSMOS_SDK_COMMIT");

/// Cosmos protobuf definitions.
pub mod cosmos {

    /// Base functionality.
    pub mod base {
        /// Application BlockChain Interface (ABCI).
        ///
        /// Interface that defines the boundary between the replication engine
        /// (the blockchain), and the state machine (the application).
        pub mod abci {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.abci.v1beta1.rs");
            }
        }

        /// Key-value pairs.
        pub mod kv {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.kv.v1beta1.rs");
            }
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.query.v1beta1.rs");
            }
        }

        /// Reflection support.
        pub mod reflection {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.reflection.v1beta1.rs");
            }

            pub mod v2alpha1 {
                include!("prost/cosmos-sdk/cosmos.base.reflection.v2alpha1.rs");
            }
        }

        /// Snapshots containing Tendermint state sync info.
        pub mod snapshots {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.snapshots.v1beta1.rs");
            }
        }

        /// Data structure that holds the state of the application.
        pub mod store {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.store.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.base.v1beta1.rs");
        }

        pub mod tendermint {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.base.tendermint.v1beta1.rs");
            }
        }
    }

    /// Transactions.
    pub mod tx {
        /// Transaction signing support.
        pub mod signing {
            pub mod v1beta1 {
                include!("prost/cosmos-sdk/cosmos.tx.signing.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos-sdk/cosmos.tx.v1beta1.rs");
        }
    }
}

/// CosmWasm protobuf definitions.
#[cfg(feature = "cosmwasm")]
pub mod cosmwasm {
    /// Messages and services handling CosmWasm.
    pub mod wasm {
        pub mod v1 {
            include!("prost/wasmd/cosmwasm.wasm.v1.rs");
        }
    }
}