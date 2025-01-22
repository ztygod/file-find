//只针对Unix系统
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnerFilter {
    uid: Check<u32>,
    gid: Check<u32>,
}
