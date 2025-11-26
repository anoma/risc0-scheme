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

fn car_accessor(sexpr: &Sexpr) -> &Box<Sexpr> {
    if let Sexpr::Cons(hd, _tl) = sexpr {
        hd
    } else {
        panic!("not a pair")
    }
}

fn car_mutator(sexpr: &Sexpr, new_hd: &Sexpr) -> Box<Sexpr> {
    if let Sexpr::Cons(_hd, tl) = &sexpr {
        Box::new(Sexpr::Cons(Box::new(new_hd.clone()), tl.clone()))
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn car_field<'a>(accessor: &mut fn(&Sexpr) -> &Box<Sexpr>, mutator: &mut fn(&Sexpr, &Sexpr) -> Box<Sexpr>)  {
    *accessor = car_accessor;
    *mutator = car_mutator;
}

fn cdr_accessor(sexpr: &Sexpr) -> &Box<Sexpr> {
    if let Sexpr::Cons(_hd, tl) = sexpr {
        tl
    } else {
        panic!("not a pair")
    }
}

fn cdr_mutator(sexpr: &Sexpr, new_tl: &Sexpr) -> Box<Sexpr> {
    if let Sexpr::Cons(hd, _tl) = &sexpr {
        Box::new(Sexpr::Cons(hd.clone(), Box::new(new_tl.clone())))
    } else {
        panic!("not a pair")
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cdr_field(accessor: &mut fn(&Sexpr) -> &Box<Sexpr>, mutator: &mut fn(&Sexpr, &Sexpr) -> Box<Sexpr>) {
    *accessor = cdr_accessor;
    *mutator = cdr_mutator;
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

fn undefined_accessor(_: &T) -> &U {
    panic!("accessor undefined")
}

fn undefined_mutator(_: &T, _: &U) -> Box<T> {
    panic!("mutator undefined")
}

#[unsafe(no_mangle)]
pub extern "C" fn get<'a>(obj: &'a T, field: extern "C" fn(&mut fn(&T) -> &U, &mut fn(&T, &U) -> Box<T>)) -> U {
    let mut accessor: fn(&T) -> &U = undefined_accessor;
    let mut mutator: fn(&T, &U) -> Box<T> = undefined_mutator;
    field(&mut accessor, &mut mutator);
    *accessor(obj)
}

#[unsafe(no_mangle)]
pub extern "C" fn put<'a>(obj: &T, field: extern "C" fn(&mut fn(&T) -> &U, &mut fn(&T, &U) -> Box<T>), val: &U) -> Box<T> {
    let mut accessor: fn(&T) -> &U = undefined_accessor;
    let mut mutator: fn(&T, &U) -> Box<T> = undefined_mutator;
    field(&mut accessor, &mut mutator);
    mutator(obj, val)
}
