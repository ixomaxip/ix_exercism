#[macro_export]
macro_rules! hashmap {
    ( $($k:expr => $v:expr),* ) => {{
        let mut h_map = crate::HashMap::new();
        $(
            h_map.insert($k, $v);
        )*
        h_map
    }};
    ( $($k:expr => $v:expr,)* ) => {{
        crate::hashmap!($($k => $v),*)
    }}
}