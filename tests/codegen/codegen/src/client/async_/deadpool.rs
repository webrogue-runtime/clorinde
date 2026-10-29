// This file was generated with `cornucopia`. Do not modify.

use super::generic_client::GenericClient;
use deadpool_postgres::{
    Client as DeadpoolClient, ClientWrapper, Transaction as DeadpoolTransaction,
};
use tokio_postgres::{
    Client as PgClient, Error, Row, RowStream, Statement, ToStatement,
    Transaction as PgTransaction,
    types::{BorrowToSql, ToSql},
};
impl GenericClient for DeadpoolClient {
    fn stmt_cache() -> bool {
        true
    }
    fn prepare(
        &self,
        query: &str,
    ) -> impl std::future::Future<Output = Result<Statement, Error>> + Send {
        async move { ClientWrapper::prepare_cached(self, query).await }
    }
    fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { PgClient::execute(self, query, params).await }
    }
    fn execute_typed(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send {
        async move { PgClient::execute_typed(self, query, typed_params).await }
    }
    fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { PgClient::query_one(self, statement, params).await }
    }
    fn query_typed_one(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send {
        async move { PgClient::query_typed_one(self, query, typed_params).await }
    }
    fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { PgClient::query_opt(self, statement, params).await }
    }
    fn query_typed_opt(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send {
        async move { PgClient::query_typed_opt(self, query, typed_params).await }
    }
    fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Vec<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { PgClient::query(self, query, params).await }
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
        async move { PgClient::query_raw(self, statement, params).await }
    }
    fn query_typed_raw(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<RowStream, Error>> + Send {
        async move {
            PgClient::query_typed_raw(self, query, crate::slice_iter_typed(typed_params)).await
        }
    }
}
impl GenericClient for DeadpoolTransaction<'_> {
    fn stmt_cache() -> bool {
        false
    }
    fn prepare(
        &self,
        query: &str,
    ) -> impl std::future::Future<Output = Result<Statement, Error>> + Send {
        async move { DeadpoolTransaction::prepare_cached(self, query).await }
    }
    fn execute<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { PgTransaction::execute(self, query, params).await }
    }
    fn execute_typed(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<u64, Error>> + Send {
        async move { PgTransaction::execute_typed(self, query, typed_params).await }
    }
    fn query_one<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { PgTransaction::query_one(self, statement, params).await }
    }
    fn query_typed_one(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Row, Error>> + Send {
        async move { PgTransaction::query_typed_one(self, query, typed_params).await }
    }
    fn query_opt<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { PgTransaction::query_opt(self, statement, params).await }
    }
    fn query_typed_opt(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<Option<Row>, Error>> + Send {
        async move { PgTransaction::query_typed_opt(self, query, typed_params).await }
    }
    fn query<T>(
        &self,
        query: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> impl std::future::Future<Output = Result<Vec<Row>, Error>> + Send
    where
        T: ?Sized + ToStatement + Sync + Send,
    {
        async move { PgTransaction::query(self, query, params).await }
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
        async move { PgTransaction::query_raw(self, statement, params).await }
    }
    fn query_typed_raw(
        &self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> impl std::future::Future<Output = Result<RowStream, Error>> + Send {
        async move {
            PgTransaction::query_typed_raw(self, query, crate::slice_iter_typed(typed_params)).await
        }
    }
}
