use proc_macro2::TokenStream;
use quote::quote;

use super::{DependencyAnalysis, vfs::Vfs};
use crate::config::Config;

pub(crate) fn gen_lib(
    dependency_analysis: &DependencyAnalysis,
    config: &Config,
) -> proc_macro2::TokenStream {
    let base_tokens = quote! {
        #[allow(clippy::all, clippy::pedantic)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub mod types;

        #[allow(clippy::all, clippy::pedantic)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub mod queries;

        pub mod client;

        mod array_iterator;
        mod domain;
        mod type_traits;
        mod utils;

        pub(crate) use utils::{slice_iter, slice_iter_typed};

        pub use array_iterator::ArrayIterator;
        pub use domain::{Domain, DomainArray};
        pub use type_traits::{ArraySql, BytesSql, IterSql, StringSql};
    };

    let db_imports = if config.r#async {
        quote! {
            #[cfg(feature = "deadpool")]
            pub use deadpool_postgres;

            #[cfg(any(feature = "deadpool", feature = "wasm-async"))]
            pub use tokio_postgres;
            #[cfg(any(feature = "deadpool", feature = "wasm-async"))]
            pub use tokio_postgres::fallible_iterator;

            #[cfg(not(any(feature = "deadpool", feature = "wasm-async")))]
            pub use postgres;
            #[cfg(not(any(feature = "deadpool", feature = "wasm-async")))]
            pub use postgres::fallible_iterator;
        }
    } else {
        quote! {
            pub use postgres;
            pub use postgres::fallible_iterator;
        }
    };

    let json_imports = dependency_analysis
        .json
        .then_some(quote! {
            pub use type_traits::JsonSql;
        })
        .unwrap_or_else(|| quote!());

    quote! {
        #base_tokens
        #db_imports
        #json_imports
    }
}

pub(crate) fn gen_clients(
    vfs: &mut Vfs,
    dependency_analysis: &DependencyAnalysis,
    config: &Config,
) {
    // Generate common files
    vfs.add("src/utils.rs", core_utils());
    vfs.add("src/domain.rs", core_domain());
    vfs.add("src/array_iterator.rs", core_array());
    vfs.add("src/type_traits.rs", core_type_traits(dependency_analysis));
    if config.sync {
        vfs.add("src/client/sync.rs", sync());
        vfs.add("src/client/sync/generic_client.rs", sync_generic_client());
    }
    if config.r#async {
        vfs.add("src/client/async_.rs", async_());
        vfs.add(
            "src/client/async_/generic_client.rs",
            async_generic_client(),
        );
        vfs.add("src/client/async_/deadpool.rs", async_deadpool());
    }
    vfs.add("src/client.rs", client(config))
}

pub fn client(config: &Config) -> proc_macro2::TokenStream {
    match (config.r#async, config.sync) {
        (true, false) => quote! {
            pub(crate) mod async_;
            pub use async_::*;
        },
        (false, true) => quote! {
            pub(crate) mod sync;
            pub use sync::*;
        },
        _ => quote! {
            pub mod sync;
            pub mod async_;
        },
    }
}

pub fn core_utils() -> proc_macro2::TokenStream {
    quote! {
        use postgres_types::{Kind, ToSql, Type};

        pub fn escape_domain(ty: &Type) -> &Type {
            match ty.kind() {
                Kind::Domain(ty) => ty,
                _ => ty,
            }
        }

        pub fn slice_iter<'a>(
            s: &'a [&'a (dyn ToSql + Sync)],
        ) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
            s.iter().map(|s| *s as _)
        }

        pub fn slice_iter_typed<'a>(
            s: &'a [(&'a (dyn ToSql + Sync), Type)],
        ) -> impl ExactSizeIterator<Item = (&'a dyn ToSql, Type)> + 'a {
            s.iter().map(|(s, t)| (*s as _, t.clone()))
        }
    }
}

pub fn core_domain() -> proc_macro2::TokenStream {
    quote! {
        use postgres_protocol::types::{array_to_sql, ArrayDimension};
        use postgres_types::{private::BytesMut, IsNull, Kind, ToSql, Type};
        use std::{
            error::Error,
            fmt::{Debug, Formatter},
        };

        use super::{type_traits::ArraySql, utils::escape_domain};

        pub struct Domain<T: ToSql>(pub T);

        impl<T: ToSql + Debug> Debug for Domain<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("DomainWrapper").field(&self.0).finish()
            }
        }

        impl<T: ToSql> ToSql for Domain<T> {
            fn to_sql(
                &self,
                ty: &Type,
                out: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>>
            where
                Self: Sized,
            {
                postgres_types::ToSql::to_sql(&self.0, escape_domain(ty), out)
            }

            fn accepts(ty: &Type) -> bool
            where
                Self: Sized,
            {
                T::accepts(escape_domain(ty))
            }

            fn to_sql_checked(
                &self,
                ty: &Type,
                out: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }

        pub struct DomainArray<'a, T: ToSql + Sync, A: ArraySql<Item = T>>(pub &'a A);

        impl<T: ToSql + Sync, A: ArraySql<Item = T>> Debug for DomainArray<'_, T, A> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("ArrayDomain").field(&self.0).finish()
            }
        }

        impl<'a, T: ToSql + Sync + 'a, A: ArraySql<Item = T>> ToSql for DomainArray<'a, T, A> {
            fn to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
                self.0.escape_domain_to_sql(ty, w)
            }

            fn accepts(ty: &Type) -> bool {
                match *ty.kind() {
                    Kind::Array(ref member) => T::accepts(escape_domain(member)),
                    _ => false,
                }
            }

            fn to_sql_checked(
                &self,
                ty: &Type,
                out: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }

        pub fn escape_domain_to_sql<T: ToSql>(
            ty: &Type,
            w: &mut BytesMut,
            iter: impl ExactSizeIterator<Item = T>,
        ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
            let member_type = match *ty.kind() {
                Kind::Array(ref member) => escape_domain(member),
                _ => panic!("expected array type got {ty}"),
            };

            let dimension = ArrayDimension {
                len: downcast(iter.len())?,
                lower_bound: 1,
            };

            array_to_sql(
                Some(dimension),
                member_type.oid(),
                iter,
                |e, w| match Domain(e).to_sql(member_type, w)? {
                    IsNull::No => Ok(postgres_protocol::IsNull::No),
                    IsNull::Yes => Ok(postgres_protocol::IsNull::Yes),
                },
                w,
            )?;
            Ok(IsNull::No)
        }

        fn downcast(len: usize) -> Result<i32, Box<dyn Error + Sync + Send>> {
            if len > i32::MAX as usize {
                Err("value too large to transmit".into())
            } else {
                Ok(len as i32)
            }
        }
    }
}

pub fn core_array() -> proc_macro2::TokenStream {
    quote! {
        use postgres_protocol::types::{array_from_sql, ArrayValues};
        use postgres_types::{FromSql, Kind, Type};
        use std::fmt::Debug;
        use std::marker::PhantomData;

        use super::fallible_iterator::FallibleIterator;
        use super::utils::escape_domain;

        /// Iterator over the items in a PostgreSQL array. You only need this if you are
        /// working with custom zero-cost type mapping of rows containing PostgreSQL arrays.
        pub struct ArrayIterator<'a, T: FromSql<'a>> {
            values: ArrayValues<'a>,
            ty: Type,
            _type: PhantomData<T>,
        }

        impl<'a, T: FromSql<'a>> Debug for ArrayIterator<'a, T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("ArrayIterator")
                    .field("values", &"[T]")
                    .field("ty", &self.ty)
                    .field("_type", &self._type)
                    .finish()
            }
        }

        impl<'a, T: FromSql<'a>> Iterator for ArrayIterator<'a, T> {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                self.values
                    .next()
                    .unwrap()
                    .map(|raw| T::from_sql_nullable(&self.ty, raw).unwrap())
            }
        }

        impl<'a, T: FromSql<'a>> FromSql<'a> for ArrayIterator<'a, T> {
            fn from_sql(
                ty: &Type,
                raw: &'a [u8],
            ) -> Result<ArrayIterator<'a, T>, Box<dyn std::error::Error + Sync + Send>>
            {
                let member_type = match *escape_domain(ty).kind() {
                    Kind::Array(ref member) => escape_domain(member),
                    _ => panic!("expected array type got {ty}"),
                };

                let array = array_from_sql(raw)?;
                if array.dimensions().count()? > 1 {
                    return Err("array contains too many dimensions".into());
                }

                Ok(ArrayIterator {
                    ty: member_type.clone(),
                    values: array.values(),
                    _type: PhantomData,
                })
            }

            fn accepts(ty: &Type) -> bool {
                match *ty.kind() {
                    Kind::Array(ref inner) => T::accepts(escape_domain(inner)),
                    _ => false,
                }
            }
        }
    }
}

pub fn core_type_traits(dependency_analysis: &DependencyAnalysis) -> proc_macro2::TokenStream {
    let base_traits = quote! {
        use std::borrow::Cow;

        use super::domain::escape_domain_to_sql;
        use postgres_protocol::types::{self, ArrayDimension};
        use postgres_types::{private::BytesMut, to_sql_checked, IsNull, Kind, ToSql, Type};

        pub trait StringSql: std::fmt::Debug + ToSql + Sync {}
        impl<T: StringSql> StringSql for &T {}
        impl StringSql for String {}
        impl StringSql for &str {}
        impl StringSql for Cow<'_, str> {}
        impl StringSql for Box<str> {}

        pub trait BytesSql: std::fmt::Debug + ToSql + Send + Sync {}
        impl<T: BytesSql> BytesSql for &T {}
        impl BytesSql for Vec<u8> {}
        impl BytesSql for &[u8] {}
    };

    let json_traits = if dependency_analysis.json {
        quote! {
            pub trait JsonSql: std::fmt::Debug + ToSql + Sync + Send {}
            impl<T: JsonSql> JsonSql for &T {}
            impl JsonSql for serde_json::value::Value {}
            impl<T: serde::ser::Serialize + std::fmt::Debug + Sync + Send> JsonSql for postgres_types::Json<T> {}
        }
    } else {
        quote!()
    };

    let array_traits = quote! {
        pub trait ArraySql: std::fmt::Debug + ToSql + Send + Sync {
            type Item;
            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>;
        }

        impl<T: std::fmt::Debug + ToSql + Sync, A: ArraySql<Item = T>> ArraySql for &A {
            type Item = T;

            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                A::escape_domain_to_sql(self, ty, w)
            }
        }

        impl<T: std::fmt::Debug + ToSql + Send + Sync> ArraySql for Vec<T> {
            type Item = T;

            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                escape_domain_to_sql(ty, w, self.iter())
            }
        }

        impl<T: std::fmt::Debug + ToSql + Sync> ArraySql for &[T] {
            type Item = T;

            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                escape_domain_to_sql(ty, w, self.iter())
            }
        }

        impl<
            T: std::fmt::Debug + ToSql + Send + Sync,
            I: Iterator<Item = T> + ExactSizeIterator,
            F: Fn() -> I + Send + Sync,
        > ArraySql for IterSql<T, I, F> {
            type Item = T;

            fn escape_domain_to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                escape_domain_to_sql(ty, w, (self.0)())
            }
        }

        pub struct IterSql<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync>(pub F);

        impl<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync> std::fmt::Debug
            for IterSql<T, I, F>
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("ArrayFn").finish()
            }
        }

        // Taken from `postgres`
        impl<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync> ToSql
            for IterSql<T, I, F>
        {
            fn to_sql(
                &self,
                ty: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                let member_type = match *ty.kind() {
                    Kind::Array(ref member) => member,
                    _ => panic!("expected array type"),
                };

                let iter = (self.0)();

                let dimension = ArrayDimension {
                    len: downcast(iter.len())?,
                    lower_bound: 1,
                };

                types::array_to_sql(
                    Some(dimension),
                    member_type.oid(),
                    iter,
                    |e, w| match e.to_sql(member_type, w)? {
                        IsNull::No => Ok(postgres_protocol::IsNull::No),
                        IsNull::Yes => Ok(postgres_protocol::IsNull::Yes),
                    },
                    w,
                )?;
                Ok(IsNull::No)
            }

            fn accepts(ty: &Type) -> bool {
                match *ty.kind() {
                    Kind::Array(ref member) => T::accepts(member),
                    _ => false,
                }
            }

            to_sql_checked!();
        }

        // https://github.com/sfackler/rust-postgres/blob/765395f288861209a644c621bf72172acd482515/postgres-types/src/lib.rs
        fn downcast(len: usize) -> Result<i32, Box<dyn std::error::Error + Sync + Send>> {
            if len > i32::MAX as usize {
                Err("value too large to transmit".into())
            } else {
                Ok(len as i32)
            }
        }
    };

    quote! {
        #base_traits
        #json_traits
        #array_traits
    }
}

pub fn sync() -> proc_macro2::TokenStream {
    quote! {
        pub use generic_client::GenericClient;
        mod generic_client;

        use postgres::{
            types::ToSql,
            Error, Row, RowIter, Statement,
        };

        /// This trait allows you to bind parameters to a query using a single
        /// struct, rather than passing each bind parameter as a function parameter.
        pub trait Params<'c, 'a, 's, P, O, C> {
            fn params(&'s self, client: &'c mut C, params: &'a P) -> O;
        }

        pub fn one<C: GenericClient>(
            client: &mut C,
            query: &str,
            params: &[&(dyn ToSql + Sync)],
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
            cached: Option<&Statement>,
        ) -> Result<Row, Error> {
            if let Some(cached) = cached {
                client.query_one(cached, params)
            } else if C::stmt_cache() {
                let cached = client.prepare(query)?;
                client.query_one(&cached, params)
            } else {
                client.query_typed_one(query, typed_params)
            }
        }

        pub fn opt<C: GenericClient>(
            client: &mut C,
            query: &str,
            params: &[&(dyn ToSql + Sync)],
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
            cached: Option<&Statement>,
        ) -> Result<Option<Row>, Error> {
            if let Some(cached) = cached {
                client.query_opt(cached, params)
            } else if C::stmt_cache() {
                let cached = client.prepare(query)?;
                client.query_opt(&cached, params)
            } else {
                client.query_typed_opt(query, typed_params)
            }
        }

        pub fn raw<'a, C: GenericClient>(
            client: &'a mut C,
            query: &str,
            params: &[&(dyn ToSql + Sync)],
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
            cached: Option<&Statement>,
        ) -> Result<RowIter<'a>, Error> {
            if let Some(cached) = cached {
                client.query_raw(cached, crate::slice_iter(params))
            } else if C::stmt_cache() {
                let cached = client.prepare(query)?;
                client.query_raw(&cached, crate::slice_iter(params))
            } else {
                client.query_typed_raw(query, typed_params)
            }
        }
    }
}

pub fn async_() -> proc_macro2::TokenStream {
    quote! {
        pub use generic_client::GenericClient;
        mod generic_client;

        #[cfg(feature = "deadpool")]
        mod deadpool;

        use tokio_postgres::{
            types::ToSql,
            Error, Row, RowStream, Statement,
        };

        /// This trait allows you to bind parameters to a query using a single
        /// struct, rather than passing each bind parameter as a function parameter.
        pub trait Params<'c, 'a, 's, P, O, C> {
            fn params(&'s self, client: &'c C, params: &'a P) -> O;
        }

        pub async fn one<C: GenericClient>(
            client: &C,
            query: &str,
            params: &[&(dyn ToSql + Sync)],
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
            cached: Option<&Statement>,
        ) -> Result<Row, Error> {
            if let Some(cached) = cached {
                client.query_one(cached, params).await
            } else if C::stmt_cache() {
                let cached = client.prepare(query).await?;
                client.query_one(&cached, params).await
            } else {
                client.query_typed_one(query, typed_params).await
            }
        }

        pub async fn opt<C: GenericClient>(
            client: &C,
            query: &str,
            params: &[&(dyn ToSql + Sync)],
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
            cached: Option<&Statement>,
        ) -> Result<Option<Row>, Error> {
            if let Some(cached) = cached {
                client.query_opt(cached, params).await
            } else if C::stmt_cache() {
                let cached = client.prepare(query).await?;
                client.query_opt(&cached, params).await
            } else {
                client.query_typed_opt(query, typed_params).await
            }
        }

        pub async fn raw<C: GenericClient>(
            client: &C,
            query: &str,
            params: &[&(dyn ToSql + Sync)],
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
            cached: Option<&Statement>,
        ) -> Result<RowStream, Error> {
            if let Some(cached) = cached {
                client.query_raw(cached, crate::slice_iter(params)).await
            } else if C::stmt_cache() {
                let cached = client.prepare(query).await?;
                client.query_raw(&cached, crate::slice_iter(params)).await
            } else {
                client.query_typed_raw(query, typed_params).await
            }
        }
    }
}

/// Signature differences between the synchronous and asynchronous client backends.
struct Backend {
    /// Receiver of the client methods.
    receiver: TokenStream,
    /// Wraps a result type into the return position of a method.
    ret: fn(TokenStream) -> TokenStream,
    /// Extra bounds on the `?Sized + ToStatement` query parameter.
    extra_bounds: TokenStream,
    /// Type returned by the `*_raw` methods.
    raw: TokenStream,
    /// Generics of `query_raw`, whose shape differs between the backends.
    query_raw_generics: TokenStream,
    /// Bounds of `query_raw`, whose shape differs between the backends.
    query_raw_bounds: TokenStream,
    /// Wraps a forwarding call into an implementation body.
    body: fn(TokenStream) -> TokenStream,
}

fn in_result(ty: TokenStream) -> TokenStream {
    quote!(Result<#ty, Error>)
}

fn in_future(ty: TokenStream) -> TokenStream {
    quote!(impl std::future::Future<Output = Result<#ty, Error>> + Send)
}

fn sync_body(call: TokenStream) -> TokenStream {
    quote!({ #call })
}

fn async_body(call: TokenStream) -> TokenStream {
    quote!({ async move { #call.await } })
}

fn sync_backend() -> Backend {
    Backend {
        receiver: quote!(&mut self),
        ret: in_result,
        extra_bounds: quote!(),
        raw: quote!(RowIter<'_>),
        query_raw_generics: quote!(<T, P, I>),
        query_raw_bounds: quote! {
            T: ?Sized + ToStatement,
            P: BorrowToSql,
            I: IntoIterator<Item = P>,
            I::IntoIter: ExactSizeIterator
        },
        body: sync_body,
    }
}

fn async_backend() -> Backend {
    Backend {
        receiver: quote!(&self),
        ret: in_future,
        extra_bounds: quote!(+ Sync + Send),
        raw: quote!(RowStream),
        query_raw_generics: quote!(<T, I>),
        query_raw_bounds: quote! {
            T: ?Sized + ToStatement + Sync + Send,
            I: IntoIterator + Sync + Send,
            I::IntoIter: ExactSizeIterator,
            I::Item: BorrowToSql
        },
        body: async_body,
    }
}

/// Emits the client methods of a backend, either as trait declarations
/// (`target: None`) or as forwarding implementations for `target`.
fn methods(
    backend: &Backend,
    target: Option<TokenStream>,
    prepare_call: TokenStream,
) -> TokenStream {
    let receiver = &backend.receiver;
    let extra_bounds = &backend.extra_bounds;
    let query_raw_generics = &backend.query_raw_generics;
    let query_raw_bounds = &backend.query_raw_bounds;

    let ret_statement = (backend.ret)(quote!(Statement));
    let ret_u64 = (backend.ret)(quote!(u64));
    let ret_row = (backend.ret)(quote!(Row));
    let ret_opt_row = (backend.ret)(quote!(Option<Row>));
    let ret_rows = (backend.ret)(quote!(Vec<Row>));
    let ret_raw = (backend.ret)(backend.raw.clone());

    let forwarding = |method: TokenStream, args: TokenStream| match &target {
        Some(target) => quote!(#target::#method(self, #args)),
        None => quote!(),
    };
    let body = |call: TokenStream| match &target {
        Some(_) => (backend.body)(call),
        None => quote!(;),
    };

    let prepare_body = body(prepare_call);
    let execute_body = body(forwarding(quote!(execute), quote!(query, params)));
    let execute_typed_body = body(forwarding(
        quote!(execute_typed),
        quote!(query, typed_params),
    ));
    let query_one_body = body(forwarding(quote!(query_one), quote!(statement, params)));
    let query_typed_one_body = body(forwarding(
        quote!(query_typed_one),
        quote!(query, typed_params),
    ));
    let query_opt_body = body(forwarding(quote!(query_opt), quote!(statement, params)));
    let query_typed_opt_body = body(forwarding(
        quote!(query_typed_opt),
        quote!(query, typed_params),
    ));
    let query_body = body(forwarding(quote!(query), quote!(query, params)));
    let query_raw_body = body(forwarding(quote!(query_raw), quote!(statement, params)));
    let query_typed_raw_body = body(forwarding(
        quote!(query_typed_raw),
        quote!(query, crate::slice_iter_typed(typed_params)),
    ));

    quote! {
        fn prepare(#receiver, query: &str) -> #ret_statement #prepare_body

        fn execute<T>(
            #receiver,
            query: &T,
            params: &[&(dyn ToSql + Sync)],
        ) -> #ret_u64
        where
            T: ?Sized + ToStatement #extra_bounds
        #execute_body

        fn execute_typed(
            #receiver,
            query: &str,
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
        ) -> #ret_u64 #execute_typed_body

        fn query_one<T>(
            #receiver,
            statement: &T,
            params: &[&(dyn ToSql + Sync)],
        ) -> #ret_row
        where
            T: ?Sized + ToStatement #extra_bounds
        #query_one_body

        fn query_typed_one(
            #receiver,
            query: &str,
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
        ) -> #ret_row #query_typed_one_body

        fn query_opt<T>(
            #receiver,
            statement: &T,
            params: &[&(dyn ToSql + Sync)],
        ) -> #ret_opt_row
        where
            T: ?Sized + ToStatement #extra_bounds
        #query_opt_body

        fn query_typed_opt(
            #receiver,
            query: &str,
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
        ) -> #ret_opt_row #query_typed_opt_body

        fn query<T>(
            #receiver,
            query: &T,
            params: &[&(dyn ToSql + Sync)],
        ) -> #ret_rows
        where
            T: ?Sized + ToStatement #extra_bounds
        #query_body

        fn query_raw #query_raw_generics(
            #receiver,
            statement: &T,
            params: I,
        ) -> #ret_raw
        where
            #query_raw_bounds
        #query_raw_body

        fn query_typed_raw(
            #receiver,
            query: &str,
            typed_params: &[(&(dyn ToSql + Sync), postgres_types::Type)],
        ) -> #ret_raw #query_typed_raw_body
    }
}

pub fn sync_generic_client() -> proc_macro2::TokenStream {
    let backend = sync_backend();
    let trait_methods = methods(&backend, None, quote!());
    let transaction_methods = methods(
        &backend,
        Some(quote!(Transaction)),
        quote!(Transaction::prepare(self, query)),
    );
    let client_methods = methods(
        &backend,
        Some(quote!(Client)),
        quote!(Client::prepare(self, query)),
    );

    quote! {
        use postgres::{
            types::{BorrowToSql, ToSql},
            Client, Error, Row, RowIter, Statement, ToStatement, Transaction,
        };

        /// Abstraction over multiple types of synchronous clients.
        /// This allows you to use postgres clients and transactions interchangeably.
        pub trait GenericClient {
            fn stmt_cache() -> bool {
                false
            }

            #trait_methods
        }

        impl GenericClient for Transaction<'_> {
            #transaction_methods
        }

        impl GenericClient for Client {
            #client_methods
        }
    }
}

pub fn async_generic_client() -> proc_macro2::TokenStream {
    let backend = async_backend();
    let trait_methods = methods(&backend, None, quote!());
    let transaction_methods = methods(
        &backend,
        Some(quote!(Transaction)),
        quote!(Transaction::prepare(self, query)),
    );
    let client_methods = methods(
        &backend,
        Some(quote!(Client)),
        quote!(Client::prepare(self, query)),
    );

    quote! {
        use tokio_postgres::{
            types::{BorrowToSql, ToSql},
            Client, Error, Row, RowStream, Statement, ToStatement, Transaction,
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

            #trait_methods
        }

        impl GenericClient for Transaction<'_> {
            #transaction_methods
        }

        impl GenericClient for Client {
            #client_methods
        }
    }
}

pub fn async_deadpool() -> proc_macro2::TokenStream {
    let backend = async_backend();
    let client_methods = methods(
        &backend,
        Some(quote!(PgClient)),
        quote!(ClientWrapper::prepare_cached(self, query)),
    );
    let transaction_methods = methods(
        &backend,
        Some(quote!(PgTransaction)),
        quote!(DeadpoolTransaction::prepare_cached(self, query)),
    );

    quote! {
        use deadpool_postgres::{
            Client as DeadpoolClient, ClientWrapper, Transaction as DeadpoolTransaction,
        };
        use tokio_postgres::{
            types::{BorrowToSql, ToSql}, Client as PgClient, Error, Row, RowStream, Statement,
            ToStatement, Transaction as PgTransaction,
        };

        use super::generic_client::GenericClient;

        impl GenericClient for DeadpoolClient {
            fn stmt_cache() -> bool {
                true
            }

            #client_methods
        }

        impl GenericClient for DeadpoolTransaction<'_> {
            fn stmt_cache() -> bool {
                false
            }

            #transaction_methods
        }
    }
}
