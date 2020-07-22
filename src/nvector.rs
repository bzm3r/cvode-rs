use std::os::raw::c_int;
use crate::bindings::{N_Vector, N_VectorContent_Serial, N_VNew_Serial};

unsafe fn nv_content_s(v: N_Vector) -> N_VectorContent_Serial {
    (*v).content as N_VectorContent_Serial
}

unsafe fn nv_length_s(v: N_Vector) -> c_int {
    (*nv_content_s(v)).length
}

unsafe fn nv_own_data_s(v: N_Vector) -> c_int {
    (*nv_content_s(v)).own_data
}

unsafe fn nv_data_s(v: N_Vector) -> *mut f32 {
    (*nv_content_s(v)).data
}

unsafe fn nv_ith_s(v: N_Vector, i: i32) -> f32 {
    *nv_data_s(v).offset(i as isize)
}

unsafe fn nv_set_data(v: N_Vector, dat: &[f32]) {
    let N = dat.len();
    assert_eq!(nv_length_s(v), N as i32);
    *nv_data_s(v).copy_from(dat as *const f32, N);
}

unsafe fn new_nvector(v: &[f32]) -> N_Vector {
    let y = N_VNew_Serial(v.len() as i32);
    nv_set_data(y, v);
    y
}