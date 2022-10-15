pub trait FromKeys<K> {
    fn from_keys<I: IntoIterator<Item = K>>(iter: I) -> Self;
}

pub trait FromKeysAndValues<K, V> {
    fn from_keys_and_values<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self;
}
