use core::mem::{align_of, size_of};
use core::slice;

pub(crate) fn sliceit<T>(t: &T) -> &[u8] {
    unsafe { slice::from_raw_parts(t as *const _ as _, size_of::<T>()) }
}

pub(crate) fn unsliceit2<T>(t: &[u8]) -> (&T, &[u8]) {
    assert!(t.len() > size_of::<T>());
    assert!(t.as_ptr() as usize % align_of::<T>() == 0);
    (unsafe { &*(t.as_ptr() as *const T) }, &t[size_of::<T>()..])
}

pub(crate) fn unsliceit<T>(t: &[u8]) -> &T {
    unsliceit2(t).0
}

pub(crate) fn meh<T>(t: T) -> T {
    t
}

pub(crate) fn slice8(x: &[u32]) -> &[u8] {
    let len = x.len() * 4;
    unsafe { slice::from_raw_parts(x.as_ptr() as _, len) }
}

pub(crate) fn slice8_mut(x: &mut [u32]) -> &mut [u8] {
    let len = x.len() * 4;
    unsafe { slice::from_raw_parts_mut(x.as_mut_ptr() as _, len) }
}

pub(crate) fn slice32(x: &[u8]) -> &[u32] {
    assert!(x.len() % 4 == 0);
    assert!(x.as_ptr() as usize % 4 == 0);
    let len = x.len() / 4;
    unsafe { slice::from_raw_parts(x.as_ptr() as _, len) }
}

pub(crate) fn slice32_mut(x: &mut [u8]) -> &mut [u32] {
    assert!(x.len() % 4 == 0);
    assert!(x.as_ptr() as usize % 4 == 0);
    let len = x.len() / 4;
    unsafe { slice::from_raw_parts_mut(x.as_ptr() as _, len) }
}
