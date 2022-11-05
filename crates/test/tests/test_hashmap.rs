use std::collections::HashMap;

#[test]
fn test_default_hash_map() {
    let mut map = HashMap::<u32, u32>::new();
    let generate_value = |x: u32| -> u32 { x * 8 / 5 };
    let key_limit = 2_000u32;

    for i in 0..key_limit {
        map.insert(i, generate_value(i));
    }

    assert_eq!(key_limit as usize, map.len());

    for i in 0..key_limit {
        assert_eq!(Some(&generate_value(i)), map.get(&i));
        map.remove(&i);
    }

    assert!(map.is_empty());
}
