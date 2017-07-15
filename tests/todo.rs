// DONE static_methods
// DONE generic_static_methods
// TODO static_methods_for_generic_structs
// TODO generic_static_methods_of_generic_structs

// TODO methods
// TODO generic_methods
// TODO methods_of_generic_structs
// TODO generic_methods_of_generic_structs

// TODO static_methods_of_traits
// TODO generic_static_methods_of_traits
// TODO static_methods_for_generic_structs_of_traits
// TODO generic_static_methods_of_generic_structs_of_traits

// TODO methods_of_traits
// TODO generic_methods_of_traits
// TODO methods_of_generic_structs_of_traits
// TODO generic_methods_of_generic_structs_of_traits

// TODO static_methods_of_generic_traits
// TODO generic_static_methods_of_generic_traits
// TODO static_methods_for_generic_structs_of_generic_traits
// TODO generic_static_methods_of_generic_structs_of_generic_traits

// TODO methods_of_generic_traits
// TODO generic_methods_of_generic_traits
// TODO methods_of_generic_structs_of_generic_traits
// TODO generic_methods_of_generic_structs_of_generic_traits

// TODO +default

// fns                   ~ 2 generic combinations * 1 self variants = 2
// methods               ~ 4 generic combinations * 4 self variants = 16
// trait methods         ~ 8 generic combinations * 4 self variants = 32
// default trait methods ~ 8 generic combinations * 4 self variants = 32
// directory + mod file      file                     test          = 82

// each test needs Return and Continue versions
// &self and &mut self methods check that self is not modified when mocked

// tested fn has <self if method> + 1 + <1 if fn generic> + <1 if trait generic> Display args
// fns returns formatted str with args
// Return mocked fns return same str, but prefixed with "mocked"
// methods return str starting with self field
// methods are 1 field named tuples
// &mut self methods modify own field

// fn - function struct Struct trait Trait

// generics:
// fn, struct, trait
// each generic should check no leak of type and leak of lifetime
//
// self:
// none, val, ref, mut ref

use std::fmt::Display;

#[test]
fn static_methods() {
    U::f(23u32);
}


struct U<T> (pub T);

impl<T: Display> U<T> {
    fn f(t: T) {
        println!("f {}", t);
        Self::g(t);
        U::<f64>::g(1.2f64);
    } // Self is already monomorphised

    fn g(t: T) {
        println!("g {}", t)
    }
}

#[test]
fn methods() {
    assert_eq!(200, S(2).f());
    assert_eq!(300, S(3).f_mut());
    assert_eq!(400, S(4).f_ref(&S(400)));
    assert_eq!(500, S(5).f_ref_mut(&mut S(500)));
}

struct S(u32);

use std::mem;

impl <'a> S {
    fn f(self) -> u32 {
        unsafe {
            *(&self as *const Self as *mut Self) = mem::replace(&mut*(&self as *const Self as *mut Self), mem::uninitialized()).s();
        }
        self.0
    }

    fn f_mut(mut self) -> u32 {
        self = self.s();
        self.0
    }

    fn f_ref(&'a self, other: &'a Self) -> u32 {
        unsafe {
            *(&self as *const &Self as *mut &Self) = mem::replace(&mut*(&self as *const &Self as *mut &Self), mem::uninitialized()).t(other);
        }
        self.0
    }

    fn f_ref_mut(&'a mut self, other: &'a mut Self) -> u32 {
        unsafe {
            *(&self as *const &mut Self as *mut &mut Self) = mem::replace(&mut*(&self as *const &mut Self as *mut &mut Self), mem::uninitialized()).u(other);
        }
        self.0
    }

    fn s(self) -> Self {
        S(self.0 * 100)
    }

    fn t(&'a self, other: &'a Self) -> &'a Self {
        other
    }

    fn u(&'a mut self, other: &'a mut Self) -> &'a mut Self {
        other
    }
}