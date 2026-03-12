// props.rs
use fdt::node::FdtNode;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct RegDesc {
    pub base: usize,
    pub size: usize,
}

// Get string property
#[allow(dead_code)]
pub(super) fn prop_str<'a>(
    node: &FdtNode<'a, '_>,
    name: &str,
) -> Option<&'a str> {
    node.property(name)?.as_str()
}

// Get u32 property
#[allow(dead_code)]
pub(super) fn prop_u32(node: &FdtNode, name: &str) -> Option<u32> {
    let p = node.property(name)?;
    let bytes: [u8; 4] = p.value.get(..4)?.try_into().ok()?;
    Some(u32::from_be_bytes(bytes))
}
