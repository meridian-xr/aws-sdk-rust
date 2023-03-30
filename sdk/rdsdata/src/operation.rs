// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchExecuteStatement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_execute_statement`](crate::client::Client::batch_execute_statement).
            ///
            /// `ParseStrictResponse` impl for `BatchExecuteStatement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchExecuteStatement {
    _private: ()
}
impl BatchExecuteStatement {
    /// Creates a new builder-style object to manufacture [`BatchExecuteStatementInput`](crate::input::BatchExecuteStatementInput).
    pub fn builder() -> crate::input::batch_execute_statement_input::Builder {
        crate::input::batch_execute_statement_input::Builder::default()
    }
    /// Creates a new `BatchExecuteStatement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchExecuteStatement {
                type Output = std::result::Result<crate::output::BatchExecuteStatementOutput, crate::error::BatchExecuteStatementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_batch_execute_statement_error(response)
                     } else {
                        crate::operation_deser::parse_batch_execute_statement_response(response)
                     }
                }
            }

/// Operation shape for `BeginTransaction`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`begin_transaction`](crate::client::Client::begin_transaction).
            ///
            /// `ParseStrictResponse` impl for `BeginTransaction`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BeginTransaction {
    _private: ()
}
impl BeginTransaction {
    /// Creates a new builder-style object to manufacture [`BeginTransactionInput`](crate::input::BeginTransactionInput).
    pub fn builder() -> crate::input::begin_transaction_input::Builder {
        crate::input::begin_transaction_input::Builder::default()
    }
    /// Creates a new `BeginTransaction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BeginTransaction {
                type Output = std::result::Result<crate::output::BeginTransactionOutput, crate::error::BeginTransactionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_begin_transaction_error(response)
                     } else {
                        crate::operation_deser::parse_begin_transaction_response(response)
                     }
                }
            }

/// Operation shape for `CommitTransaction`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`commit_transaction`](crate::client::Client::commit_transaction).
            ///
            /// `ParseStrictResponse` impl for `CommitTransaction`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CommitTransaction {
    _private: ()
}
impl CommitTransaction {
    /// Creates a new builder-style object to manufacture [`CommitTransactionInput`](crate::input::CommitTransactionInput).
    pub fn builder() -> crate::input::commit_transaction_input::Builder {
        crate::input::commit_transaction_input::Builder::default()
    }
    /// Creates a new `CommitTransaction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CommitTransaction {
                type Output = std::result::Result<crate::output::CommitTransactionOutput, crate::error::CommitTransactionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_commit_transaction_error(response)
                     } else {
                        crate::operation_deser::parse_commit_transaction_response(response)
                     }
                }
            }

/// Operation shape for `ExecuteSql`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`execute_sql`](crate::client::Client::execute_sql).
            ///
            /// `ParseStrictResponse` impl for `ExecuteSql`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExecuteSql {
    _private: ()
}
impl ExecuteSql {
    /// Creates a new builder-style object to manufacture [`ExecuteSqlInput`](crate::input::ExecuteSqlInput).
    pub fn builder() -> crate::input::execute_sql_input::Builder {
        crate::input::execute_sql_input::Builder::default()
    }
    /// Creates a new `ExecuteSql` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExecuteSql {
                type Output = std::result::Result<crate::output::ExecuteSqlOutput, crate::error::ExecuteSqlError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_execute_sql_error(response)
                     } else {
                        crate::operation_deser::parse_execute_sql_response(response)
                     }
                }
            }

/// Operation shape for `ExecuteStatement`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`execute_statement`](crate::client::Client::execute_statement).
            ///
            /// `ParseStrictResponse` impl for `ExecuteStatement`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExecuteStatement {
    _private: ()
}
impl ExecuteStatement {
    /// Creates a new builder-style object to manufacture [`ExecuteStatementInput`](crate::input::ExecuteStatementInput).
    pub fn builder() -> crate::input::execute_statement_input::Builder {
        crate::input::execute_statement_input::Builder::default()
    }
    /// Creates a new `ExecuteStatement` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExecuteStatement {
                type Output = std::result::Result<crate::output::ExecuteStatementOutput, crate::error::ExecuteStatementError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_execute_statement_error(response)
                     } else {
                        crate::operation_deser::parse_execute_statement_response(response)
                     }
                }
            }

/// Operation shape for `RollbackTransaction`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`rollback_transaction`](crate::client::Client::rollback_transaction).
            ///
            /// `ParseStrictResponse` impl for `RollbackTransaction`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RollbackTransaction {
    _private: ()
}
impl RollbackTransaction {
    /// Creates a new builder-style object to manufacture [`RollbackTransactionInput`](crate::input::RollbackTransactionInput).
    pub fn builder() -> crate::input::rollback_transaction_input::Builder {
        crate::input::rollback_transaction_input::Builder::default()
    }
    /// Creates a new `RollbackTransaction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RollbackTransaction {
                type Output = std::result::Result<crate::output::RollbackTransactionOutput, crate::error::RollbackTransactionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_rollback_transaction_error(response)
                     } else {
                        crate::operation_deser::parse_rollback_transaction_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

