use srcpos::*;
use srcpos_get::*;

#[derive(GetLoc)]
struct A {
    loc: Loc,
}

#[test]
fn test_a() {
    let a = A {
        loc: locof!(0, 0, 0, 0),
    };
    let v = a.loc();
    assert_eq!(v, locof!(0, 0, 0, 0));
}

#[derive(GetLoc)]
struct B {
    #[loc]
    a: Loc,
}

#[test]
fn test_b() {
    let b = B {
        a: locof!(0, 0, 0, 0),
    };
    let v = b.loc();
    assert_eq!(v, locof!(0, 0, 0, 0));
}

#[derive(GetLoc)]
struct C(Loc);

#[test]
fn test_c() {
    let c = C(locof!(0, 0, 0, 0));
    let v = c.loc();
    assert_eq!(v, locof!(0, 0, 0, 0));
}

#[derive(GetLoc)]
struct D(u8, #[loc] Loc);

#[test]
fn test_d() {
    let d = D(0, locof!(0, 0, 0, 0));
    let v = d.loc();
    assert_eq!(v, locof!(0, 0, 0, 0));
}

#[derive(GetLoc)]
enum E {
    A(Loc),
    B(u8, #[loc] Loc),
    C {
        loc: Loc,
    },
    D {
        #[loc]
        a: Loc,
        _b: u8,
    },
}

#[test]
fn test_e_a() {
    let e = E::A(locof!(0, 0, 0, 0));
    let v = e.loc();
    assert_eq!(v, locof!(0, 0, 0, 0));
}

#[test]
fn test_e_b() {
    let e = E::B(0, locof!(0, 0, 0, 0));
    let v = e.loc();
    assert_eq!(v, locof!(0, 0, 0, 0));
}

#[test]
fn test_e_c() {
    let e = E::C {
        loc: locof!(0, 0, 0, 0),
    };
    let v = e.loc();
    assert_eq!(v, locof!(0, 0, 0, 0));
}

#[test]
fn test_e_d() {
    let e = E::D {
        a: locof!(0, 0, 0, 0),
        _b: 0,
    };
    let v = e.loc();
    assert_eq!(v, locof!(0, 0, 0, 0));
}
