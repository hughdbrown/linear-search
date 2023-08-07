pub enum SearchEnum {
    Found(usize),
    NotFound(usize),
}

pub fn search<T>(values: &[T], val: T) -> SearchEnum
    where T: Eq
{
    for (i, v) in values.iter().enumerate() {
        if *v == val {
            return SearchEnum::Found(i + 1);
        }
    }
    SearchEnum::NotFound(values.len())
}
