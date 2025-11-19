use risc0_zkvm::guest::env;
use scheme_core::Sexpr;
use std::ffi::{c_char, CStr, CString};

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
pub extern "C" fn set_car(sexpr: &mut Sexpr, new_hd: &Sexpr) {
    if let Sexpr::Cons(hd, _tl) = sexpr {
        *hd = Box::new(new_hd.clone())
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_cdr(sexpr: &mut Sexpr, new_tl: &Sexpr) {
    if let Sexpr::Cons(_hd, tl) = sexpr {
        *tl = Box::new(new_tl.clone())
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
pub extern "C" fn drop(sexpr: Box<Sexpr>) {
    std::mem::drop(sexpr)
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
pub extern "C" fn drop_string(s: *mut c_char) {
    std::mem::drop(unsafe { CString::from_raw(s)})
}

#[unsafe(no_mangle)]
pub extern "C" fn is_equal(sexpr1: &Sexpr, sexpr2: &Sexpr) -> bool {
    sexpr1 == sexpr2
}
