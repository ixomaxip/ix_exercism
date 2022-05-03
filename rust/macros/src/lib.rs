#[macro_export]
macro_rules! hashmap {
    ( $( $($k:expr => $v:expr)+ $(,)? )* ) => {{
        let mut h_map = ::std::collections::HashMap::new();
        $( $( h_map.insert($k, $v); )* )*
        h_map
    }};
}