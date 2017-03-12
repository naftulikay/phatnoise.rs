use super::*;

use std::sync::Arc;

#[test]
fn test_string_pool() {
    let pool = StringPool::new();
    let new = pool.get("brand new");
    let old = pool.get("brand new");

    assert!(Arc::ptr_eq(&new, &old));
}
