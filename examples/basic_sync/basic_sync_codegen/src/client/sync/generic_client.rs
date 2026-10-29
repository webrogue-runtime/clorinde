// This file was generated with `cornucopia`. Do not modify.

use postgres::{
    Client, Error, Row, RowIter, Statement, ToStatement, Transaction,
    types::{BorrowToSql, ToSql},
};
/// Abstraction over multiple types of synchronous clients.
/// This allows you to use postgres clients and transactions interchangeably.
pub trait GenericClient {
    fn stmt_cache() -> bool {
        false
    }
    fn prepare(&mut self, query: &str) -> Result<Statement, Error>;
    fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement;
    fn execute_typed(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<u64, Error>;
    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement;
    fn query_typed_one(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<Row, Error>;
    fn query_opt<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement;
    fn query_typed_opt(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<Option<Row>, Error>;
    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement;
    fn query_raw<T, P, I>(&mut self, statement: &T, params: I) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
        P: BorrowToSql,
        I: IntoIterator<Item = P>,
        I::IntoIter: ExactSizeIterator;
    fn query_typed_raw(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<RowIter<'_>, Error>;
}
impl GenericClient for Transaction<'_> {
    fn prepare(&mut self, query: &str) -> Result<Statement, Error> {
        Transaction::prepare(self, query)
    }
    fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::execute(self, query, params)
    }
    fn execute_typed(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<u64, Error> {
        Transaction::execute_typed(self, query, typed_params)
    }
    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query_one(self, statement, params)
    }
    fn query_typed_one(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<Row, Error> {
        Transaction::query_typed_one(self, query, typed_params)
    }
    fn query_opt<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query_opt(self, statement, params)
    }
    fn query_typed_opt(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<Option<Row>, Error> {
        Transaction::query_typed_opt(self, query, typed_params)
    }
    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Transaction::query(self, query, params)
    }
    fn query_raw<T, P, I>(&mut self, statement: &T, params: I) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
        P: BorrowToSql,
        I: IntoIterator<Item = P>,
        I::IntoIter: ExactSizeIterator,
    {
        Transaction::query_raw(self, statement, params)
    }
    fn query_typed_raw(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<RowIter<'_>, Error> {
        Transaction::query_typed_raw(self, query, crate::slice_iter_typed(typed_params))
    }
}
impl GenericClient for Client {
    fn prepare(&mut self, query: &str) -> Result<Statement, Error> {
        Client::prepare(self, query)
    }
    fn execute<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::execute(self, query, params)
    }
    fn execute_typed(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<u64, Error> {
        Client::execute_typed(self, query, typed_params)
    }
    fn query_one<T>(&mut self, statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query_one(self, statement, params)
    }
    fn query_typed_one(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<Row, Error> {
        Client::query_typed_one(self, query, typed_params)
    }
    fn query_opt<T>(
        &mut self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query_opt(self, statement, params)
    }
    fn query_typed_opt(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<Option<Row>, Error> {
        Client::query_typed_opt(self, query, typed_params)
    }
    fn query<T>(&mut self, query: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Client::query(self, query, params)
    }
    fn query_raw<T, P, I>(&mut self, statement: &T, params: I) -> Result<RowIter<'_>, Error>
    where
        T: ?Sized + ToStatement,
        P: BorrowToSql,
        I: IntoIterator<Item = P>,
        I::IntoIter: ExactSizeIterator,
    {
        Client::query_raw(self, statement, params)
    }
    fn query_typed_raw(
        &mut self,
        query: &str,
        typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
    ) -> Result<RowIter<'_>, Error> {
        Client::query_typed_raw(self, query, crate::slice_iter_typed(typed_params))
    }
}
