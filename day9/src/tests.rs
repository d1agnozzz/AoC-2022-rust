use super::*;

#[test]
fn test_vec_normalization() {
    assert_eq!(MoveVector(2, 2).normalize(), MoveVector(1, 1));
    assert_eq!(MoveVector(2, -2).normalize(), MoveVector(1, -1));
    assert_eq!(MoveVector(-2, 2).normalize(), MoveVector(-1, 1));
    assert_eq!(MoveVector(-2, -2).normalize(), MoveVector(-1, -1));

    assert_eq!(MoveVector(-5, -10).normalize(), MoveVector(-1, -1));
    assert_eq!(MoveVector(0, -1).normalize(), MoveVector(0, -1));
}

#[test]
fn test_mvec_inverse() {
    assert_eq!(MoveVector(-1, 1), MoveVector(1, -1) * -1);
    assert_eq!(MoveVector(0, -1), MoveVector(0, 1) * -1);
}

#[test]
fn test_arithmetic() {
    assert_eq!(Position(3, 7), Position(-3, 2) + MoveVector(6, 5).into());
}
