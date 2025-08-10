use sqlx::query::QueryAs;
use sqlx::{Database, Encode, Type};

pub trait QueryAsExt<'q, DB: Database, O> {
    fn bind_all<T: 'q + Encode<'q, DB> + Type<DB>>(
        self,
        values: impl IntoIterator<Item = T>,
    ) -> QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>>;
}

impl<'q, DB: Database, O> QueryAsExt<'q, DB, O>
    for QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>>
{
    fn bind_all<T: 'q + Encode<'q, DB> + Type<DB>>(
        mut self,
        values: impl IntoIterator<Item = T>,
    ) -> QueryAs<'q, DB, O, <DB as Database>::Arguments<'q>> {
        for value in values {
            self = self.bind(value);
        }
        self
    }
}
