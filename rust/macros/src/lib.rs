#[macro_export]
macro_rules! hashmap {
    () => {
        {
            HashMap::new()
        }
    };
    ( $($k:expr => $v:expr),* $(,)?) => {
        {
            let mut h_map = HashMap::new();
            $(
                h_map.insert($k, $v);
            )*
            h_map
        }
    };
}
