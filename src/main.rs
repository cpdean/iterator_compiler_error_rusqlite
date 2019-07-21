use rusqlite::Result;

pub fn fetch_one_row<F, T>(mut m: rusqlite::MappedRows<F>) -> Result<T>
where
    F: FnMut(&rusqlite::Row<'_>) -> Result<T>,
{
    m.next().unwrap()
}
