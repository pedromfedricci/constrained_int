use constrained_int::i8::{ConstrainedI8, ConstrainedI8Error};

// Lower bound = -5, upper bound = 10, default = -1.
type Constrained = ConstrainedI8<-5, 10, -1>;
type ConstrainedError = ConstrainedI8Error<-5, 10>;

fn main() -> Result<(), ConstrainedError> {
    // Gets the default value.
    let mut constrained = Constrained::default();
    assert_eq!(constrained.get(), -1);

    // Sets within inclusive range, succeeds.
    constrained.set(-5)?;
    assert_eq!(constrained.get(), -5);

    // Below lower bound, fails.
    assert_eq!(constrained.checked_sub(1), None);
    assert_eq!(constrained.get(), -5);

    // Saturates at upper bound.
    constrained = constrained.saturating_add(100);
    assert_eq!(constrained.get(), 10);

    // Sets out of bound, fails.
    assert!(constrained.set(11).is_err());

    // Wraps around the upper bound.
    constrained = constrained.wrapping_add(1);
    assert_eq!(constrained.get(), -5);

    Ok(())
}
