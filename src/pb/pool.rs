// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Liquidations {
    #[prost(message, repeated, tag="1")]
    pub liquidations: ::prost::alloc::vec::Vec<Liquidation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Liquidation {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub block_num: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub collateral_asset: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub debt_asset: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub debt_to_cover: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub liquidated_collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub liquidator: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub receive_a_token: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenLiquidations {
    #[prost(message, repeated, tag="1")]
    pub token_liquidations: ::prost::alloc::vec::Vec<TokenLiquidation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenLiquidation {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub block_num: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub collateral_asset: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub debt_asset: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub debt_to_cover: ::prost::alloc::string::String,
    #[prost(double, tag="7")]
    pub liquidated_collateral_amount: f64,
    #[prost(string, tag="8")]
    pub liquidator: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub receive_a_token: bool,
}
// @@protoc_insertion_point(module)
