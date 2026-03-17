#[macro_export]
macro_rules! container_of {
    // NonNull<Node> → NonNull<Container>
    ($node:expr, $container:ty, $field:ident) => {{
        let offset = core::mem::offset_of!($container, $field);
        let p = ($node.as_ptr() as *mut u8).sub(offset);
        core::ptr::NonNull::new_unchecked(p as *mut $container)
    }};
}

#[macro_export]
macro_rules! node_of {
    // NonNull<Container> → NonNull<Node>
    ($ptr:expr, $container:ty, $field:ident) => {{
        let offset = core::mem::offset_of!($container, $field);
        let p = ($ptr.as_ptr() as *mut u8).add(offset);
        core::ptr::NonNull::new_unchecked(p as *mut _)
    }};
}
