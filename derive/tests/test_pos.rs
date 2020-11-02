use srcpos::*;
use srcpos_get::*;

#[derive(GetPos)]
struct A {
    pos: Pos,
}

#[test]
fn test_a() {
    let a = A {
        pos: posof!(0),
    };
    let v = a.pos();
    assert_eq!(v, posof!(0));
}

#[derive(GetPos)]
struct B {
    #[pos]
    a: Pos,
}

#[test]
fn test_b() {
    let b = B {
        a: posof!(0),
    };
    let v = b.pos();
    assert_eq!(v, posof!(0));
}

#[derive(GetPos)]
struct C(Pos);

#[test]
fn test_c() {
    let c = C(posof!(0));
    let v = c.pos();
    assert_eq!(v, posof!(0));
}

#[derive(GetPos)]
struct D(u8, #[pos] Pos);

#[test]
fn test_d() {
    let d = D(0, posof!(0));
    let v = d.pos();
    assert_eq!(v, posof!(0));
}

#[derive(GetPos)]
enum E {
    A(Pos),
    B(u8, #[pos] Pos),
    C {
        pos: Pos,
    },
    D {
        #[pos]
        a: Pos,
        _b: u8,
    },
}

#[test]
fn test_e_a() {
    let e = E::A(posof!(0));
    let v = e.pos();
    assert_eq!(v, posof!(0));
}

#[test]
fn test_e_b() {
    let e = E::B(0, posof!(0));
    let v = e.pos();
    assert_eq!(v, posof!(0));
}

#[test]
fn test_e_c() {
    let e = E::C {
        pos: posof!(0),
    };
    let v = e.pos();
    assert_eq!(v, posof!(0));
}

#[test]
fn test_e_d() {
    let e = E::D {
        a: posof!(0),
        _b: 0,
    };
    let v = e.pos();
    assert_eq!(v, posof!(0));
}
