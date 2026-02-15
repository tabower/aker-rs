use crate::prelude::*;

use super::table::Testable;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    let total = tests.len();
    let mut success = 0;

    pr_debug!("Running {} tests...\n", total);

    for (i, test) in tests.iter().enumerate() {
        pr_debug!("[{}] ", i);
        if test.run() {
            success += 1;
        }
    }

    pr_debug!("test result: {}/{} passed\n", success, total);
}
