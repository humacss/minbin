use crate::rust_std::vec::Vec;

// Collect references and sort them without cloning collection keys or values.
pub(super) fn sorted_by_key<T, K: Ord>(items: impl IntoIterator<Item = T>, key: impl FnMut(&T) -> K) -> Vec<T> {
    let mut items = items.into_iter().collect::<Vec<_>>();
    items.sort_by_key(key);
    items
}
