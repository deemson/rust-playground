extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn revert_hash_map() {
        use std::collections::HashMap;
        let m1 = HashMap::from([
            ("yo", "sup"),
            ("hello", "world"),
        ]);
        let m2: HashMap<_, _> = m1.iter().map(|(k, v)| {
            (v.clone(), k.clone())
        }).collect();
        assert_eq!("sup", m1["yo"]);
        assert_eq!("yo", m2["sup"]);
    }
}
