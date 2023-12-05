//! Type name registry: used to compute type URLs.

// TODO(tarcieri): generate these automatically using `prost-build`
// See: https://github.com/tokio-rs/prost/issues/926

use crate::{cosmos, traits::Name};

macro_rules! impl_name {
    ($type:ty, $package:expr, $name:expr) => {
        impl Name for $type {
            const NAME: &'static str = $name;
            const PACKAGE: &'static str = $package;

            fn full_name() -> String {
                full_name::<Self>()
            }
        }
    };
}

impl_name!(
    cosmos::base::abci::v1beta1::MsgData,
    "cosmos.base.v1beta1.abci",
    "MsgData"
);
impl_name!(
    cosmos::base::abci::v1beta1::TxMsgData,
    "cosmos.base.v1beta1.abci",
    "TxMsgData"
);

impl_name!(cosmos::tx::v1beta1::Tx, "cosmos.tx.v1beta1", "Tx");
impl_name!(
    cosmos::tx::v1beta1::AuthInfo,
    "cosmos.tx.v1beta1",
    "AuthInfo"
);
impl_name!(cosmos::tx::v1beta1::Fee, "cosmos.tx.v1beta1", "Fee");
impl_name!(cosmos::tx::v1beta1::TxBody, "cosmos.tx.v1beta1", "TxBody");
impl_name!(
    cosmos::tx::v1beta1::SignerInfo,
    "cosmos.tx.v1beta1",
    "SingerInfo"
);
impl_name!(
    cosmos::tx::v1beta1::ModeInfo,
    "cosmos.tx.v1beta1",
    "ModeInfo"
);

#[cfg(feature = "cosmwasm")]
mod wasm {
    use super::full_name;
    use crate::{cosmwasm, traits::Name};

    const COSMWASM_PACKAGE: &str = "cosmwasm.wasm.v1";

    impl_name!(
        cosmwasm::wasm::v1::AccessConfigUpdate,
        COSMWASM_PACKAGE,
        "AccessConfigUpdate"
    );
    impl_name!(
        cosmwasm::wasm::v1::AccessConfig,
        COSMWASM_PACKAGE,
        "AccessConfig"
    );
    impl_name!(
        cosmwasm::wasm::v1::MigrateContractProposal,
        COSMWASM_PACKAGE,
        "MigrateContractProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::UpdateInstantiateConfigProposal,
        COSMWASM_PACKAGE,
        "UpdateInstantiateConfigProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::SudoContractProposal,
        COSMWASM_PACKAGE,
        "SudoContractProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::ExecuteContractProposal,
        COSMWASM_PACKAGE,
        "ExecuteContractProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::UpdateAdminProposal,
        COSMWASM_PACKAGE,
        "UpdateAdminProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::ClearAdminProposal,
        COSMWASM_PACKAGE,
        "ClearAdminProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::PinCodesProposal,
        COSMWASM_PACKAGE,
        "PinCodesProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::UnpinCodesProposal,
        COSMWASM_PACKAGE,
        "UnpinCodesProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::InstantiateContractProposal,
        COSMWASM_PACKAGE,
        "InstantiateContractProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::StoreCodeProposal,
        COSMWASM_PACKAGE,
        "StoreCodeProposal"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgStoreCode,
        COSMWASM_PACKAGE,
        "MsgStoreCode"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgInstantiateContract,
        COSMWASM_PACKAGE,
        "MsgInstantiateContract"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgInstantiateContract2,
        COSMWASM_PACKAGE,
        "MsgInstantiateContract2"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgExecuteContract,
        COSMWASM_PACKAGE,
        "MsgExecuteContract"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgMigrateContract,
        COSMWASM_PACKAGE,
        "MsgMigrateContract"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgUpdateAdmin,
        COSMWASM_PACKAGE,
        "MsgUpdateAdmin"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgClearAdmin,
        COSMWASM_PACKAGE,
        "MsgClearAdmin"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgStoreCodeResponse,
        COSMWASM_PACKAGE,
        "MsgStoreCodeResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgInstantiateContractResponse,
        COSMWASM_PACKAGE,
        "MsgInstantiateContractResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgExecuteContractResponse,
        COSMWASM_PACKAGE,
        "MsgExecuteContractResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgMigrateContractResponse,
        COSMWASM_PACKAGE,
        "MsgMigrateContractResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgUpdateAdminResponse,
        COSMWASM_PACKAGE,
        "MsgUpdateAdminResponse"
    );
    impl_name!(
        cosmwasm::wasm::v1::MsgClearAdminResponse,
        COSMWASM_PACKAGE,
        "MsgClearAdminResponse"
    );
}

// TODO(tarcieri): remove this when tokio-rs/prost#923 is released (v0.12.1?)
fn full_name<T: Name>() -> String {
    format!("{}.{}", T::PACKAGE, T::NAME)
}
