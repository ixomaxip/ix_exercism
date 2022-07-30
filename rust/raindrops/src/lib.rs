pub fn raindrops(n: u32) -> String {
    let drops = vec!((3, "Pling"), (5, "Plang"), (7, "Plong"));

    let mut s = drops.iter()
        .map(|(k, v)| match n % k {
            0 => v,
            _ => ""
        })
        .fold(String::new(), |a, b| a + b );
    if s == "" {
        s = n.to_string();
    }
    s
}
