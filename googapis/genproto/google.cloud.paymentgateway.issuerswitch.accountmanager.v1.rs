/// A transaction processed by the account manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountManagerTransaction {
    /// The name of the transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project}/accountManagers/{account_manager}/transactions/{transaction}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The account ID for which the transaction was processed.
    #[prost(string, tag = "2")]
    pub account_id: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag = "3")]
    pub info: ::core::option::Option<AccountManagerTransactionInfo>,
    /// The payer in the transaction.
    #[prost(message, optional, tag = "4")]
    pub payer: ::core::option::Option<AccountManagerSettlementParticipant>,
    /// The payee in the transaction.
    #[prost(message, optional, tag = "5")]
    pub payee: ::core::option::Option<AccountManagerSettlementParticipant>,
    /// Reconciliation information for the transaction.
    #[prost(message, optional, tag = "6")]
    pub reconciliation_info: ::core::option::Option<AccountManagerTransactionReconciliationInfo>,
    /// The amount for payment settlement in the transaction.
    #[prost(message, optional, tag = "7")]
    pub amount: ::core::option::Option<super::super::super::super::super::r#type::Money>,
}
/// Information about a transaction processed by the account manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountManagerTransactionInfo {
    /// An identifier that is mandatorily present in every transaction processed
    /// via account manager.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The transaction type.
    #[prost(enumeration = "AccountManagerTransactionType", tag = "3")]
    pub transaction_type: i32,
    /// Output only. The transaction's state.
    #[prost(enumeration = "account_manager_transaction_info::State", tag = "5")]
    pub state: i32,
    /// Metadata about the transaction.
    #[prost(message, optional, tag = "6")]
    pub metadata:
        ::core::option::Option<account_manager_transaction_info::AccountManagerTransactionMetadata>,
    /// Output only. Any error details for the current transaction, if the state is
    /// `FAILED`.
    #[prost(message, optional, tag = "7")]
    pub error_details: ::core::option::Option<
        account_manager_transaction_info::AccountManagerTransactionErrorDetails,
    >,
}
/// Nested message and enum types in `AccountManagerTransactionInfo`.
pub mod account_manager_transaction_info {
    /// Common metadata about a transaction.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccountManagerTransactionMetadata {
        /// The time at which the transaction took place.
        #[prost(message, optional, tag = "1")]
        pub transaction_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. The time at which the transaction resource was created by
        /// the account manager.
        #[prost(message, optional, tag = "2")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. The time at which the transaction resource was last updated
        /// by the account manager.
        #[prost(message, optional, tag = "3")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Retrieval reference number (RRN) for the transaction.
        #[prost(string, tag = "4")]
        pub retrieval_reference_number: ::prost::alloc::string::String,
        /// The initiation mode of this transaction.
        #[prost(string, tag = "5")]
        pub initiation_mode: ::prost::alloc::string::String,
        /// The purpose code of this transaction.
        #[prost(string, tag = "6")]
        pub purpose_code: ::prost::alloc::string::String,
    }
    /// All details about any error in the processing of a transaction.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccountManagerTransactionErrorDetails {
        /// Output only. Error code of the failed transaction.
        #[prost(string, tag = "1")]
        pub error_code: ::prost::alloc::string::String,
        /// Output only. Error description for the failed transaction.
        #[prost(string, tag = "2")]
        pub error_message: ::prost::alloc::string::String,
    }
    /// Specifies the current state of the transaction.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The transaction has successfully completed.
        Succeeded = 1,
        /// The transaction has failed.
        Failed = 2,
    }
}
/// A participant in a payment settlement transaction processed by the
/// account manager. The participant could either be the payer or the payee
/// in the transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountManagerSettlementParticipant {
    /// The participant information.
    #[prost(message, optional, tag = "1")]
    pub participant: ::core::option::Option<AccountManagerParticipant>,
    /// Information about a merchant who is a participant in the payment. This
    /// field will be specified only if the participant is a merchant.
    #[prost(message, optional, tag = "2")]
    pub merchant_info: ::core::option::Option<AccountManagerMerchantInfo>,
}
/// A participant in a transaction processed by the account manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountManagerParticipant {
    /// The payment address of the participant. In the UPI system, this will be the
    /// virtual payment address (VPA) of the participant.
    #[prost(string, tag = "1")]
    pub payment_address: ::prost::alloc::string::String,
    /// The persona of the participant.
    #[prost(enumeration = "account_manager_participant::Persona", tag = "2")]
    pub persona: i32,
    /// Unique identification of an account.
    #[prost(message, optional, tag = "3")]
    pub account: ::core::option::Option<super::super::v1::AccountReference>,
}
/// Nested message and enum types in `AccountManagerParticipant`.
pub mod account_manager_participant {
    /// The type of the participant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Persona {
        /// Unspecified persona.
        Unspecified = 0,
        /// Participant is an entity.
        Entity = 1,
        /// Participant is a person.
        Person = 2,
    }
}
/// A merchant in a transaction processed by the account manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountManagerMerchantInfo {
    /// Merchant Category Code (MCC) as specified by UPI. This is a four-digit
    /// number listed in ISO 18245 for retail financial services.
    #[prost(string, tag = "1")]
    pub category_code: ::prost::alloc::string::String,
    /// ID of the merchant.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// Reconciliation information for a transaction processed by account manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountManagerTransactionReconciliationInfo {
    /// Output only. State of reconciliation.
    #[prost(
        enumeration = "account_manager_transaction_reconciliation_info::ReconciliationState",
        tag = "1"
    )]
    pub state: i32,
    /// Time at which reconciliation was performed for the transaction.
    #[prost(message, optional, tag = "2")]
    pub reconciliation_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `AccountManagerTransactionReconciliationInfo`.
pub mod account_manager_transaction_reconciliation_info {
    /// State of transaction reconciliation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReconciliationState {
        /// Unspecified state.
        Unspecified = 0,
        /// Reconciliation was successful.
        Succeeded = 1,
        /// Reconciliation failed.
        Failed = 2,
    }
}
/// Request for the `ExportAccountManagerTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAccountManagerTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}/accountManagers/{account_manager}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Transaction type of an account manager transaction. The possible
    /// values for transaction type are
    ///
    /// * CREDIT
    /// * CREDIT_REVERSAL
    /// * DEBIT
    /// * DEBIT_REVERSAL
    ///
    /// If no transaction type is specified, records of all the above transaction
    /// types will be exported.
    #[prost(enumeration = "AccountManagerTransactionType", tag = "3")]
    pub transaction_type: i32,
    /// Optional. The start time for the query.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The end time for the query.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `ListAccountManagerTransactions` method. Callers can request
/// for transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountManagerTransactionsRequest {
    /// Required. The parent resource. The format is
    /// `projects/{project}/accountManagers/{account_manager}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of transactions to return. The service may
    /// return fewer than this value. If unspecified or if the specified value is
    /// less than 1, at most 50 transactions will be returned. The maximum value is
    /// 1000; values above 1000 will be coerced to 1000. While paginating, you can
    /// specify a new page size parameter for each page of transactions to be
    /// listed.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `ListAccountManagerTransactions` call. Specify this parameter to retrieve
    /// the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListAccountManagerTransactions` method that returned the page token will
    /// be reused for all further calls where the page token parameter is
    /// specified.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. An expression that filters the list of transactions.
    ///
    /// A filter expression consists of a field name, a comparison operator, and
    /// a value for filtering. The value must be a string, a number, or a
    /// boolean. The comparison operator must be one of: `<`, `>`, or `=`.
    /// Filters are not case sensitive.
    ///
    /// The following fields in the `Transaction` are eligible for filtering:
    ///
    ///   * `accountID` - The account ID. Allowed comparison operators: `=`. When
    ///   account manager is used for managing UPI Lite transactions, accountID
    ///   should be the Lite Reference Number (LRN).
    ///   * `accountNumber` - Bank account number associated with the
    ///   account. Allowed comparison operators: `=`.
    ///   * `IFSC` - Bank IFSC code associated with the account.
    ///   Allowed comparison operators: `=`.
    ///   * `RRN` - The retrieval reference number of the transaction. Allowed
    ///   comparison operators: `=`.
    ///   * `transactionTime` - The timestamp (in UTC) at which the transaction
    ///   took place. The value should be in the format `YYYY-MM-DDTHH:MM:SSZ`.
    ///   Allowed comparison operators: `>`, `<`.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///
    ///   * `rrn = 123456789123` - The RRN is _123456789123_.
    ///   * `(accountID = 12345678) AND (transactionTime < "2021-08-15T14:50:00Z")`
    ///   - The accountID is 12345678 and the transaction was received
    ///   before _2021-08-15 14:50:00 UTC_.
    ///   * `transactionTime > "2021-08-15T14:50:00Z" AND transactionTime <
    ///   "2021-08-16T14:50:00Z"` - The transaction was received between
    ///   _2021-08-15 14:50:00 UTC_ and _2021-08-16 14:50:00 UTC_.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// Response for the `ListAccountManagerTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccountManagerTransactionsResponse {
    /// List of account manager transactions satisfying the filtered request.
    #[prost(message, repeated, tag = "1")]
    pub account_manager_transactions: ::prost::alloc::vec::Vec<AccountManagerTransaction>,
    /// Pass this token in the ListAccountManagerTransactionsRequest to continue to
    /// list results. If all results have been returned, this field is an empty
    /// string or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Reconciliation request for an account manager transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconcileAccountManagerTransactionsRequest {
    /// Required. The transaction that will be reconciled.
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<AccountManagerTransaction>,
}
/// Request for the `BatchReconcileAccountManagerTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchReconcileAccountManagerTransactionsRequest {
    /// Required. The parent resource. The format is
    /// `projects/{project}/accountManagers/{account_manager}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message specifying the transaction to reconcile.
    /// A maximum of 200 transactions can be reconciled in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<ReconcileAccountManagerTransactionsRequest>,
}
/// Response for the `BatchReconcileAccountManagerTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchReconcileAccountManagerTransactionsResponse {
    /// List of transactions reconciled.
    #[prost(message, repeated, tag = "1")]
    pub account_manager_transactions: ::prost::alloc::vec::Vec<AccountManagerTransaction>,
}
/// The type of a account manager transaction. Every transaction processed by the
/// account manager will be one of these transaction types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountManagerTransactionType {
    /// Unspecified transaction type.
    Unspecified = 0,
    /// Credit transaction type.
    Credit = 1,
    /// Credit reversal transaction type.
    CreditReversal = 2,
    /// Debit transaction type.
    Debit = 3,
    /// Debit reversal transaction type.
    DebitReversal = 4,
}
#[doc = r" Generated client implementations."]
pub mod account_manager_transactions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Lists and exports transactions processed by the account manager."]
    #[derive(Debug, Clone)]
    pub struct AccountManagerTransactionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountManagerTransactionsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AccountManagerTransactionsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            AccountManagerTransactionsClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Export transactions received within the specified time range as a"]
        #[doc = " file into a configured target location. The returned `Operation` type has"]
        #[doc = " the following method-specific fields:"]
        #[doc = ""]
        #[doc = " - `metadata`:"]
        #[doc = " [ExportAccountManagerTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.accountmanager.v1.ExportAccountManagerTransactionsMetadata]"]
        #[doc = " - `response`:"]
        #[doc = " [ExportAccountManagerTransactionsResponse][google.cloud.paymentgateway.issuerswitch.accountmanager.v1.ExportAccountManagerTransactionsResponse]"]
        #[doc = ""]
        #[doc = " The exported file will be in the standard CSV format where each row in the"]
        #[doc = " file represents a transaction. The file has the following fields in order:"]
        #[doc = ""]
        #[doc = " 1. `TransactionID`"]
        #[doc = "     * **Min Length** - 35 characters"]
        #[doc = "     * **Max Length** - 35 characters"]
        #[doc = "     * **Description** - Account manager transaction ID."]
        #[doc = " 1. `TransactionType`"]
        #[doc = "     * **Min Length** - 22 characters"]
        #[doc = "     * **Max Length** - 25 characters"]
        #[doc = "     * **Description** - Type of the transaction. This will be one of"]
        #[doc = "     `TRANSACTION_TYPE_CREDIT`, `TRANSACTION_TYPE_CREDIT_REVERSAL`,"]
        #[doc = "     `TRANSACTION_TYPE_DEBIT` or `TRANSACTION_TYPE_DEBIT_REVERSAL`. When"]
        #[doc = "     account manager is used for managing UPI Lite transactions, the CREDIT"]
        #[doc = "     transactions would be for Lite account top-ups and DEBIT transactions"]
        #[doc = "     could be either for a Lite account disablement where balance is"]
        #[doc = "     transferred back the underlying bank account or for a Lite account"]
        #[doc = "     financial transaction which happened offline."]
        #[doc = " 1. `AccountID`"]
        #[doc = "     * **Min Length** - 35 characters"]
        #[doc = "     * **Max Length** - 35 characters"]
        #[doc = "     * **Description** - Account ID. When account manager is used for"]
        #[doc = "     managing UPI Lite transactions, this column will contain the Lite"]
        #[doc = "     Reference Number (LRN) of the UPI Lite account."]
        #[doc = " 1. `State`"]
        #[doc = "     * **Min Length** - 6 characters"]
        #[doc = "     * **Max Length** - 12 characters"]
        #[doc = "     * **Description** - State of the transaction. This will be one of"]
        #[doc = "     `SUCCEEDED` or `FAILED`."]
        #[doc = " 1. `RRN`"]
        #[doc = "     * **Min Length** - 12 characters"]
        #[doc = "     * **Max Length** - 12 characters"]
        #[doc = "     * **Description** - Retrieval reference number associated with the"]
        #[doc = "     transaction."]
        #[doc = " 1. `PayerVPA`"]
        #[doc = "     * **Min Length** - 3 characters"]
        #[doc = "     * **Max Length** - 255 characters"]
        #[doc = "     * **Description** - Virtual Payment Address (VPA) of the payer."]
        #[doc = " 1. `PayerIFSC`"]
        #[doc = "     * **Min Length** - 11 characters"]
        #[doc = "     * **Max Length** - 11 characters"]
        #[doc = "     * **Description** - IFSC of the payer's bank account."]
        #[doc = " 1. `PayerAccountNumber`"]
        #[doc = "     * **Min Length** - 1 characters"]
        #[doc = "     * **Max Length** - 30 characters"]
        #[doc = "     * **Description** - Payer's bank account number."]
        #[doc = " 1. `PayeeVPA`"]
        #[doc = "     * **Min Length** - 3 characters"]
        #[doc = "     * **Max Length** - 255 characters"]
        #[doc = "     * **Description** - Virtual Payment Address (VPA) of the payee."]
        #[doc = " 1. `PayeeIFSC`"]
        #[doc = "     * **Min Length** - 11 characters"]
        #[doc = "     * **Max Length** - 11 characters"]
        #[doc = "     * **Description** - IFSC of the payee's bank account."]
        #[doc = " 1. `PayeeAccountNumber`"]
        #[doc = "     * **Min Length** - 1 characters"]
        #[doc = "     * **Max Length** - 30 characters"]
        #[doc = "     * **Description** - Payee's bank account number."]
        #[doc = " 1. `PayeeMCC`"]
        #[doc = "     * **Min Length** - 4 characters"]
        #[doc = "     * **Max Length** - 4 characters"]
        #[doc = "     * **Description** - Payee's Merchant Category Code (MCC), only if the"]
        #[doc = "     payee is a merchant."]
        #[doc = " 1. `PayeeMerchantID`"]
        #[doc = "     * **Min Length** - 4 characters"]
        #[doc = "     * **Max Length** - 4 characters"]
        #[doc = "     * **Description** - Payee's merchant ID, only if the payee is a"]
        #[doc = "     merchant."]
        #[doc = " 1. `Currency`"]
        #[doc = "     * **Min Length** - 3 characters"]
        #[doc = "     * **Max Length** - 3 characters"]
        #[doc = "     * **Description** - Currency of the amount involved in the transaction."]
        #[doc = "     The currency codes are defined in ISO 4217."]
        #[doc = " 1. `Amount`"]
        #[doc = "     * **Description** - Amount involved in the transaction."]
        #[doc = " 1. `Purpose`"]
        #[doc = "     * **Min Length** - 1 characters"]
        #[doc = "     * **Max Length** - 4 characters"]
        #[doc = "     * **Description** - Purpose code associated with the transaction. When"]
        #[doc = "     account manager is used for managing UPI Lite transactions, this column"]
        #[doc = "     will contain one of the values from `41` (Lite account creation with"]
        #[doc = "     initial topup), `42` (Lite account topup), `43` (Lite account"]
        #[doc = "     disablement with balance transfer) or `44` (Lite account online"]
        #[doc = "     transaction)."]
        #[doc = " 1. `TransactionTime`"]
        #[doc = "     * **Min Length** - 20 characters"]
        #[doc = "     * **Max Length** - 20 characters"]
        #[doc = "     * **Description** - Timestamp (in UTC) indicating the timestamp at"]
        #[doc = "     which the transaction took place. The format will be as per RFC-3339."]
        #[doc = "     Example : 2022-11-22T23:00:05Z"]
        pub async fn export_account_manager_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAccountManagerTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.paymentgateway.issuerswitch.accountmanager.v1.AccountManagerTransactions/ExportAccountManagerTransactions") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List account manager transactions that satisfy specified filter criteria."]
        pub async fn list_account_manager_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountManagerTransactionsRequest>,
        ) -> Result<tonic::Response<super::ListAccountManagerTransactionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.paymentgateway.issuerswitch.accountmanager.v1.AccountManagerTransactions/ListAccountManagerTransactions") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Batch reconcile account manager transactions and return status for each"]
        #[doc = " transaction."]
        pub async fn batch_reconcile_account_manager_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchReconcileAccountManagerTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::BatchReconcileAccountManagerTransactionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.paymentgateway.issuerswitch.accountmanager.v1.AccountManagerTransactions/BatchReconcileAccountManagerTransactions") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Entity representing an account managed by the account manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedAccount {
    /// The name of the account which uniquely identifies the account.
    /// Format:
    /// projects/{project}/accountManagers/{account_manager}/accounts/{account}
    /// When account manager is used for managing UPI Lite transactions,
    /// `{account}` is the Lite Reference Number (LRN).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The associated bank account information.
    #[prost(message, optional, tag = "2")]
    pub account_reference: ::core::option::Option<super::super::v1::AccountReference>,
    /// Output only. State of the account.
    #[prost(enumeration = "managed_account::State", tag = "3")]
    pub state: i32,
    /// Required. Current balance of the account.
    #[prost(message, optional, tag = "4")]
    pub balance: ::core::option::Option<super::super::super::super::super::r#type::Money>,
    /// Output only. State of the last reconciliation done on the account.
    #[prost(enumeration = "managed_account::AccountReconciliationState", tag = "5")]
    pub last_reconciliation_state: i32,
    /// Output only. Time at which last reconciliation was done on the account.
    #[prost(message, optional, tag = "6")]
    pub last_reconciliation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the account was created by the account
    /// manager.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which the account was last updated by the account
    /// manager.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `ManagedAccount`.
pub mod managed_account {
    /// State of an account.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// Account is active.
        Active = 1,
        /// Account is inactive.
        Deactivated = 2,
    }
    /// Reconciliation state of an account.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccountReconciliationState {
        /// Unspecified state.
        Unspecified = 0,
        /// Successful reconciliation.
        Succeeded = 1,
        /// Reconciliation failed.
        Failed = 2,
    }
}
/// Reconciliation request for an account balance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconcileManagedAccountBalanceRequest {
    /// Required. Account that needs to be reconciled.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<ManagedAccount>,
    /// Required. Expected balance amount for the account.
    #[prost(message, optional, tag = "2")]
    pub expected_balance: ::core::option::Option<super::super::super::super::super::r#type::Money>,
    /// Required. Timestamp to be taken as reference for reconciling the balance
    /// amount.
    #[prost(message, optional, tag = "3")]
    pub reference_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `BatchReconcileManagedAccountBalance` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchReconcileManagedAccountBalanceRequest {
    /// Required. The parent resource. The format is
    /// `projects/{project}/accountManagers/{account_manager}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message specifying the accounts to reconcile.
    /// A maximum of 200 account balances can be reconciled in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<ReconcileManagedAccountBalanceRequest>,
}
/// Response for the `BatchReconcileManagedAccountBalance` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchReconcileManagedAccountBalanceResponse {
    /// Accounts whose balances were reconciled.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<ManagedAccount>,
}
/// Request for the `GetManagedAccount` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagedAccountRequest {
    /// Required. The name of the managed account to retrieve.
    /// Format:
    /// `projects/{project}/accountManagers/{account_manager}/accounts/{account}`
    /// When account manager is used for managing UPI Lite transactions, {account}
    /// should be the Lite Reference Number (LRN).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod managed_accounts_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Reconciles and provide balance information for an account within the account"]
    #[doc = " manager."]
    #[derive(Debug, Clone)]
    pub struct ManagedAccountsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ManagedAccountsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ManagedAccountsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ManagedAccountsClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Batch reconcile account balance and return status for each account."]
        pub async fn batch_reconcile_managed_account_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchReconcileManagedAccountBalanceRequest>,
        ) -> Result<
            tonic::Response<super::BatchReconcileManagedAccountBalanceResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.paymentgateway.issuerswitch.accountmanager.v1.ManagedAccounts/BatchReconcileManagedAccountBalance") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get information on the account managed by account manager."]
        pub async fn get_managed_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManagedAccountRequest>,
        ) -> Result<tonic::Response<super::ManagedAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.paymentgateway.issuerswitch.accountmanager.v1.ManagedAccounts/GetManagedAccount") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
