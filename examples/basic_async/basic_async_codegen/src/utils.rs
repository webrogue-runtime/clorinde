// This file was generated with `cornucopia`. Do not modify.

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
