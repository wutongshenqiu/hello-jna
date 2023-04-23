#[repr(C)]
#[derive(Debug)]
pub struct SimpleStruct {
    v: u32,
}

#[no_mangle]
pub unsafe extern "C" fn construct_ss_ref(v: u32) -> *mut SimpleStruct {
    Box::into_raw(Box::new(SimpleStruct { v }))
}

#[no_mangle]
pub unsafe extern "C" fn construct_ss_val(v: u32) -> SimpleStruct {
    SimpleStruct { v }
}

#[no_mangle]
pub unsafe extern "C" fn getv_by_ref(ss: *const SimpleStruct) -> u32 {
    (*ss).v
}

#[no_mangle]
pub unsafe extern "C" fn getv_by_val(ss: SimpleStruct) -> u32 {
    ss.v
}

#[no_mangle]
pub unsafe extern "C" fn sum_by_refs(ss: *const *const SimpleStruct, len: usize) -> u32 {
    let mut res = 0;
    for i in 0..len {
        res += (*(*ss.add(i))).v;
    }

    res
}

#[no_mangle]
pub unsafe extern "C" fn sum_by_vals(ss: *const SimpleStruct, len: usize) -> u32 {
    let mut res = 0;
    for i in 0..len {
        res += (*ss.add(i)).v;
    }

    res
}

#[repr(C)]
pub struct NestedStruct {
    s: SimpleStruct
}

#[no_mangle]
pub unsafe extern "C" fn construct_ns_ref(v: u32) -> *mut NestedStruct {
    let ns = NestedStruct {
        s: SimpleStruct { v }
    };

    Box::leak(Box::new(ns))
}

#[repr(C)]
pub struct NestedStructRef {
    s: *mut SimpleStruct
}

#[no_mangle]
pub unsafe extern "C" fn construct_nsr_ref(v: u32) -> *mut NestedStructRef {
    let nsr = NestedStructRef {
        s: Box::leak(Box::new(SimpleStruct {v}))
    };

    Box::leak(Box::new(nsr))
}

#[repr(C)]
pub struct StructArray {
    ss: *mut SimpleStruct,
    len: usize
}

#[no_mangle]
pub unsafe extern "C" fn construct_sa_ref(vs: *const u32, len: usize) -> *mut StructArray {
    let mut sv = Vec::with_capacity(len);
    for i in 0..len {
        sv.push(SimpleStruct {
            v: *vs.add(i)
        })
    }

    let sa = StructArray {
        ss: sv.leak().as_mut_ptr(),
        len: len
    };

    Box::leak(Box::new(sa))
}