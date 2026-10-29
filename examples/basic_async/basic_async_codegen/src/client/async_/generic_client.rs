// This file was generated with `cornucopia`. Do not modify.

use tokio_postgres::{
    Client, Error, Row, RowStream, Statement, ToStatement, Transaction,
    types::{BorrowToSql, ToSql},
};
/// Abstraction over multiple types of asynchronous clients.
/// This allows you to use tokio_postgres clients and transactions interchangeably.
///
/// In addition, when the `deadpool` feature is enabled (default), this trait also
/// abstracts over deadpool clients and transactions
pub trait GenericClient: Send + Sync {
    fn stmt_cache() -> bool {
        false
    }
    fn prepare(
        &self,
        query: &str,
    ) -> impl std::future::Future<Output = Result<Statement, Error>> + Send;
    fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send;
    fn execute_typed(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send;
    fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send;
    fn query_typed_one(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send;
    fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send;
    fn query_typed_opt(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send;
    fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Vec<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send;
    fn query_raw<T, I>(
        &self,
        statement: &T,
        params: I,
    ) -> impl std::future::Future<Output = Result<RowStream, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
        I: IntoIterator + Sync + Send,
        I::IntoIter: ExactSizeIterator,
        I::Item: BorrowToSql;
    fn query_typed_raw(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<RowStream, Error>> + Send;
}
impl GenericClient for Transaction<'_> {
    fn prepare(
        &self,
        query: &str,
    ) -> impl std::future::Future<Output = Result<Statement, Error>> + Send {
        async move { Transaction::prepare(self, query).await }
    }
    fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { Transaction::execute(self, query, params).await }
    }
    fn execute_typed(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send {
        async move { Transaction::execute_typed(self, query, typed_params).await }
    }
    fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { Transaction::query_one(self, statement, params).await }
    }
    fn query_typed_one(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send {
        async move { Transaction::query_typed_one(self, query, typed_params).await }
    }
    fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { Transaction::query_opt(self, statement, params).await }
    }
    fn query_typed_opt(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send {
        async move { Transaction::query_typed_opt(self, query, typed_params).await }
    }
    fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Vec<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { Transaction::query(self, query, params).await }
    }
    fn query_raw<T, I>(
        &self,
        statement: &T,
        params: I,
    ) -> impl std::future::Future<Output = Result<RowStream, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
        I: IntoIterator + Sync + Send,
        I::IntoIter: ExactSizeIterator,
        I::Item: BorrowToSql,
    {
        async move { Transaction::query_raw(self, statement, params).await }
    }
    fn query_typed_raw(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<RowStream, Error>> + Send {
        async move {
            Transaction::query_typed_raw(self, query, crate::slice_iter_typed(typed_params)).await
        }
    }
}
impl GenericClient for Client {
    fn prepare(
        &self,
        query: &str,
    ) -> impl std::future::Future<Output = Result<Statement, Error>> + Send {
        async move { Client::prepare(self, query).await }
    }
    fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { Client::execute(self, query, params).await }
    }
    fn execute_typed(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send {
        async move { Client::execute_typed(self, query, typed_params).await }
    }
    fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { Client::query_one(self, statement, params).await }
    }
    fn query_typed_one(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send {
        async move { Client::query_typed_one(self, query, typed_params).await }
    }
    fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { Client::query_opt(self, statement, params).await }
    }
    fn query_typed_opt(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send {
        async move { Client::query_typed_opt(self, query, typed_params).await }
    }
    fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Vec<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { Client::query(self, query, params).await }
    }
    fn query_raw<T, I>(
        &self,
        statement: &T,
        params: I,
    ) -> impl std::future::Future<Output = Result<RowStream, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
        I: IntoIterator + Sync + Send,
        I::IntoIter: ExactSizeIterator,
        I::Item: BorrowToSql,
    {
        async move { Client::query_raw(self, statement, params).await }
    }
    fn query_typed_raw(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<RowStream, Error>> + Send {
        async move { Client::query_typed_raw(self, query, crate::slice_iter_typed(typed_params)).await }
    }
}
