use crate::prelude::*;

pub trait Testable {
    fn run(&self) -> bool;
}

impl<T: Fn() -> bool> Testable for T {
    fn run(&self) -> bool {
        pr_debug!("{}", core::any::type_name::<T>());
        let result = self();
        if result {
            pr_debug!(" -- [OK]\n");
        } else {
            pr_error!(" -- [FAILED]\n");
        }
        result
    }
}
