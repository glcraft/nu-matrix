#[inline]
pub fn parent_id() -> Option<u32> {
    Some(std::os::unix::process::parent_id())
}