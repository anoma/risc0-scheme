use risc0_zkvm::guest::env;
use scheme_core::Sexpr;
use std::ffi::{c_char, CStr, CString};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn panic(ptr: *const c_char) -> u32 {
    panic!("{}", CStr::from_ptr(ptr).to_str().expect("invalid C string"))
}

#[unsafe(no_mangle)]
pub extern "C" fn read_sexpr() -> Box<Sexpr> {
    env::read()
}

#[unsafe(no_mangle)]
pub extern "C" fn commit_sexpr(sexpr: &Sexpr) -> &Sexpr {
    env::commit(sexpr);
    sexpr
}

#[unsafe(no_mangle)]
pub extern "C" fn cons(car: &Sexpr, cdr: &Sexpr) -> Box<Sexpr> {
    Box::new(Sexpr::Cons(Box::new(car.clone()), Box::new(cdr.clone())))
}

#[unsafe(no_mangle)]
pub extern "C" fn car(sexpr: &Sexpr) -> &Sexpr {
    if let Sexpr::Cons(hd, _tl) = sexpr {
        hd
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cdr(sexpr: &Sexpr) -> &Sexpr {
    if let Sexpr::Cons(_hd, tl) = sexpr {
        tl
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn car_field(sexpr: &Sexpr) -> &Box<Sexpr> {
    if let Sexpr::Cons(hd, _tl) = sexpr {
        hd
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cdr_field(sexpr: &Sexpr) -> &Box<Sexpr> {
    if let Sexpr::Cons(_hd, tl) = sexpr {
        tl
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_car<'a>(sexpr: &mut Sexpr, new_hd: &'a Sexpr) -> &'a Sexpr {
    if let Sexpr::Cons(hd, _tl) = sexpr {
        *hd = Box::new(new_hd.clone());
        new_hd
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_cdr<'a>(sexpr: &mut Sexpr, new_tl: &'a Sexpr) -> &'a Sexpr {
    if let Sexpr::Cons(_hd, tl) = sexpr {
        *tl = Box::new(new_tl.clone());
        new_tl
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn is_pair(sexpr: &Sexpr) -> bool {
    matches!(sexpr, Sexpr::Cons(_, _))
}

#[unsafe(no_mangle)]
pub extern "C" fn is_null(sexpr: &Sexpr) -> bool {
    matches!(sexpr, Sexpr::Null)
}

#[unsafe(no_mangle)]
pub extern "C" fn is_integer(sexpr: &Sexpr) -> bool {
    matches!(sexpr, Sexpr::Integer(_))
}

#[unsafe(no_mangle)]
pub extern "C" fn is_string(sexpr: &Sexpr) -> bool {
    matches!(sexpr, Sexpr::String(_))
}

#[unsafe(no_mangle)]
pub extern "C" fn drop_sexpr(sexpr: Box<Sexpr>) -> u32 {
    std::mem::drop(sexpr);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn integer(value: i32) -> Box<Sexpr> {
    Box::new(Sexpr::Integer(value))
}

#[unsafe(no_mangle)]
pub extern "C" fn null() -> &'static Sexpr {
    &Sexpr::Null
}

#[unsafe(no_mangle)]
pub extern "C" fn string(s: *const c_char) -> Box<Sexpr> {
    unsafe {
        Box::new(Sexpr::String(CStr::from_ptr(s).to_str().expect("invalid UTF-8").to_owned()))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn as_integer(sexpr: &Sexpr) -> i32 {
    if let Sexpr::Integer(value) = sexpr {
        *value
    } else {
        panic!("not an integer")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn alloc_string(sexpr: &Sexpr) -> *mut c_char {
    if let Sexpr::String(value) = sexpr {
        let cstr = CString::new(value.as_str()).expect("string should not contain null");
        cstr.into_raw()
    } else {
        panic!("not an integer")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn drop_string(s: *mut c_char) -> u32 {
    std::mem::drop(unsafe { CString::from_raw(s)});
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn is_equal(sexpr1: &Sexpr, sexpr2: &Sexpr) -> bool {
    sexpr1 == sexpr2
}

#[unsafe(no_mangle)]
pub extern "C" fn read_vector() -> Box<Vec<u32>> {
    env::read()
}

#[unsafe(no_mangle)]
pub extern "C" fn commit_vector(vec: &Vec<u32>) -> &Vec<u32> {
    env::commit(vec);
    vec
}

#[unsafe(no_mangle)]
pub extern "C" fn make_vector(k: u32) -> Box<Vec<u32>> {
    Box::new(vec![0; k.try_into().expect("specified vector length too large")])
}

#[unsafe(no_mangle)]
pub extern "C" fn vector_length(vec: &Vec<u32>) -> u32 {
    vec.len().try_into().expect("specified vector is too large")
}

#[unsafe(no_mangle)]
pub extern "C" fn vector_ref(vec: &Vec<u32>, k: u32) -> u32 {
    *vec.get(usize::try_from(k).expect("specified index too large")).expect("specified index is invalid")
}

#[unsafe(no_mangle)]
pub extern "C" fn vector_set(vec: &mut Vec<u32>, k: u32, obj: u32) -> u32 {
    *vec.get_mut(usize::try_from(k).expect("specified index too large")).expect("specified index is invalid") = obj;
    obj
}

#[unsafe(no_mangle)]
pub extern "C" fn vector_fill(vec: &mut Vec<u32>, fill: u32) -> u32 {
    vec.fill(fill);
    fill
}

#[unsafe(no_mangle)]
pub extern "C" fn drop_vector(vec: Box<Vec<u32>>) -> u32 {
    std::mem::drop(vec);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn read_integer() -> u32 {
    env::read()
}

#[unsafe(no_mangle)]
pub extern "C" fn commit_integer(int: u32) -> u32 {
    env::commit(&int);
    int
}

pub struct T;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct U(u32);

#[unsafe(no_mangle)]
pub extern "C" fn get(obj: &T, accessor: extern "C" fn(&T) -> &U) -> U {
    *accessor(obj)
}

#[unsafe(no_mangle)]
pub extern "C" fn put(obj: &mut T, mutator: extern "C" fn(&mut T) -> &mut U, val: U) -> &mut T {
    *mutator(obj) = val;
    obj
}
