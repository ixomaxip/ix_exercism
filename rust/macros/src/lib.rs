#[macro_export]
macro_rules! hashmap {
    () => {{
        crate::HashMap::new()
    }};
    ( $($k:expr => $v:expr),* $(,)?) => {{
        let mut h_map = crate::HashMap::new();
        $(
            h_map.insert($k, $v);
        )*
        h_map
    }};
}