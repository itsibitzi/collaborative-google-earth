use std::ptr::null_mut;
use common::Vector2;
use libc::{dlopen, dlsym, RTLD_NOW};
use reqwest::Client;
use std::os::raw::{c_double, c_int};
use std::ffi::CString;

pub type FillRule = c_int;

pub type QReal = c_double;

#[repr(C)]
pub struct QPointF {
    x: QReal,
    y: QReal, 
}

#[no_mangle]
pub unsafe extern "C" fn _ZN8QPainter11drawPolygonEPK7QPointFiN2Qt8FillRuleE(point: *const QPointF, dunno: c_int, fill_rule: FillRule) {
    let request_url = "localhost:3000/drawPolygon";

    let vector = Vector2 {
        x: (*point).x,
        y: (*point).y,
    };

    let response = Client::new()
        .post(request_url)
        .json(&vector)
        .send();

    let shared_lib = CString::new("sharedlib.dylib").unwrap().as_ptr();
    let symbol = CString::new("_ZN8QPainter11drawPolygonEPK7QPointFiN2Qt8FillRuleE").unwrap().as_ptr();

    let handle = dlopen(shared_lib, RTLD_NOW);
    let real_function_ptr = dlsym(handle, symbol);

    if real_function_ptr == null_mut() {
        println!("Failed to get functions real function");
    } else {
        let real_function: extern "C" fn(*const QPointF, c_int, FillRule) = std::mem::transmute(real_function_ptr) ;
        (real_function)(point, dunno, fill_rule);
    }
}

