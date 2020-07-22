use crate::bindings::{CV_ADAMS, CV_BDF, CVodeCreate, CVodeInit, N_Vector, CV_SUCCESS, CV_MEM_NULL, CV_MEM_FAIL, CV_ILL_INPUT};
use std::os::raw::c_void;

pub enum Method {
    Adams,
    Bdf
}

struct CvodeObj {
    lmm: *mut cvoid,
}

impl CvodeObj {
    unsafe fn new(method: Method) -> CvodeObj {
        // "linear multistep method"
        let lmm = match method {
            Method::Adams => CV_ADAMS,
            Method::Bdf => CV_BDF,
        };
        CvodeObj {
            lmm: CVodeCreate(lmm as i32),
        }
    }
}

pub struct Solver {
    lmm: CvodeObj,
}

impl Solver {
    pub unsafe fn init(method: Method, t0: f32, f: fn(t0: f32, y0: N_Vector, y_out: N_Vector, user_dat: *mut cvoid), y0: &N_Vector) -> Solver {
        let cvode_obj = CvodeObj::new(method);
        match CVodeInit(cvode_obj.lmm, Some(f), t0, y) as u32 {
            CV_SUCCESS => {},
            CV_MEM_NULL => panic!("Cvode solver memory is NULL."),
            CV_MEM_FAIL => panic!("Problem with allocating memory for CVode solver."),
            CV_ILL_INPUT => panic!("Problem with inputs.")
            _ => panic!("Solver init: Unexpected error."),
        }

        Solver {
            lmm,
        }
    }
}
