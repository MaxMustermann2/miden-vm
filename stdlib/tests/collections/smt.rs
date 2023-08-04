use crate::build_test;
use test_utils::{
    crypto::{MerkleStore, RpoDigest, TieredSmt},
    Felt, StarkField, Word, ONE, ZERO,
};

// CONSTANTS
// ================================================================================================

const EMPTY_VALUE: Word = TieredSmt::EMPTY_VALUE;

// RETRIEVAL TESTS
// ================================================================================================

#[test]
fn smtget_depth_16() {
    let mut smt = TieredSmt::default();

    // create a key
    let raw_a = 0b_01010101_01101100_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);

    // make sure we get an empty value for this key
    assert_smt_get_opens_correctly(&smt, key_a, EMPTY_VALUE);

    // insert a value under this key and make sure we get it back when queried
    let val_a = [ONE, ONE, ONE, ONE];
    smt.insert(key_a, val_a);
    assert_smt_get_opens_correctly(&smt, key_a, val_a);

    // make sure that another key still returns empty value
    let raw_b = 0b_01111101_01101100_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    assert_smt_get_opens_correctly(&smt, key_b, EMPTY_VALUE);

    // make sure that another key with the same 16-bit prefix returns an empty value
    let raw_c = 0b_01010101_01101100_11111111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    assert_smt_get_opens_correctly(&smt, key_c, EMPTY_VALUE);
}

#[test]
fn smtget_depth_32() {
    let mut smt = TieredSmt::default();

    // populate the tree with two key-value pairs sharing the same 16-bit prefix for the keys
    let raw_a = 0b_01010101_01010101_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a = [ONE, ONE, ONE, ONE];
    smt.insert(key_a, val_a);

    let raw_b = 0b_01010101_01010101_11100000_11111111_10010110_10010011_11100000_00000000_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    let val_b = [ZERO, ONE, ONE, ONE];
    smt.insert(key_b, val_b);

    // make sure the values for these keys are retrieved correctly
    assert_smt_get_opens_correctly(&smt, key_a, val_a);
    assert_smt_get_opens_correctly(&smt, key_b, val_b);

    // make sure another key with the same 16-bit prefix returns an empty value
    let raw_c = 0b_01010101_01010101_11100111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    assert_smt_get_opens_correctly(&smt, key_c, EMPTY_VALUE);

    // make sure keys with the same 32-bit prefixes return empty value
    let raw_d = 0b_01010101_01010101_00011111_11111111_11111110_10010011_11100000_00000000_u64;
    let key_d = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_d)]);
    assert_smt_get_opens_correctly(&smt, key_d, EMPTY_VALUE);

    // make sure keys with the same 32-bit prefixes return empty value
    let raw_e = 0b_01010101_01010101_11100000_11111111_10011111_10010011_11100000_00000000_u64;
    let key_e = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_e)]);
    assert_smt_get_opens_correctly(&smt, key_e, EMPTY_VALUE);
}

#[test]
fn smtget_depth_48() {
    let mut smt = TieredSmt::default();

    // populate the tree with two key-value pairs sharing the same 32-bit prefix for the keys
    let raw_a = 0b_01010101_01010101_00011111_11111111_10010110_10010011_11100000_00000000_u64;
    let key_a = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_a)]);
    let val_a = [ONE, ONE, ONE, ONE];
    smt.insert(key_a, val_a);

    let raw_b = 0b_01010101_01010101_00011111_11111111_11111111_10010011_11100000_00000000_u64;
    let key_b = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_b)]);
    let val_b = [ZERO, ONE, ONE, ONE];
    smt.insert(key_b, val_b);

    // make sure the values for these keys are retrieved correctly
    assert_smt_get_opens_correctly(&smt, key_a, val_a);
    assert_smt_get_opens_correctly(&smt, key_b, val_b);

    // make sure another key with the same 32-bit prefix returns an empty value
    let raw_c = 0b_01010101_01010101_00011111_11111111_00000000_10010011_11100000_00000000_u64;
    let key_c = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_c)]);
    assert_smt_get_opens_correctly(&smt, key_c, EMPTY_VALUE);

    // make sure keys with the same 48-bit prefixes return empty value
    let raw_d = 0b_01010101_01010101_00011111_11111111_10010110_10010011_00000111_00000000_u64;
    let key_d = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_d)]);
    assert_smt_get_opens_correctly(&smt, key_d, EMPTY_VALUE);

    // make sure keys with the same 48-bit prefixes return empty value
    let raw_e = 0b_01010101_01010101_00011111_11111111_11111111_10010011_000001011_00000000_u64;
    let key_e = RpoDigest::from([ONE, ONE, ONE, Felt::new(raw_e)]);
    assert_smt_get_opens_correctly(&smt, key_e, EMPTY_VALUE);
}

// TEST HELPERS
// ================================================================================================

/// Asserts key/value opens to root for the provided Tiered Sparse Merkle tree.
fn assert_smt_get_opens_correctly(smt: &TieredSmt, key: RpoDigest, value: Word) {
    let root = smt.root();
    let source = r#"
        use.std::collections::smt

        begin
            exec.smt::get
        end
    "#;
    let initial_stack = [
        root[0].as_int(),
        root[1].as_int(),
        root[2].as_int(),
        root[3].as_int(),
        key[0].as_int(),
        key[1].as_int(),
        key[2].as_int(),
        key[3].as_int(),
    ];
    let expected_output = [
        value[3].as_int(),
        value[2].as_int(),
        value[1].as_int(),
        value[0].as_int(),
        root[3].as_int(),
        root[2].as_int(),
        root[1].as_int(),
        root[0].as_int(),
    ];

    let store = MerkleStore::from(smt);
    let advice_map = smt
        .upper_leaves()
        .map(|(node, key, value)| {
            let mut elements = key.as_elements().to_vec();
            elements.extend(&value);
            (node.as_bytes(), elements)
        })
        .collect::<Vec<_>>();

    let advice_stack = [];
    build_test!(source, &initial_stack, &advice_stack, store, advice_map.into_iter())
        .expect_stack(&expected_output);
}