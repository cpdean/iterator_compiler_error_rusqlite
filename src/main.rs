pub fn fetch_one_row<F, T, E>(mut m: rusqlite::MappedRows<F>) -> Result<T, E>
where
    F: FnMut(&rusqlite::Row<'_>) -> Result<T, E>,
{
    m.next().unwrap()
}

fn main() {
    println!("boop");
}
