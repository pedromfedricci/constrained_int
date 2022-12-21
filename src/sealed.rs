#[doc(hidden)]
pub struct Constraints<const T: bool>;

#[doc(hidden)]
pub trait Guard {}
impl Guard for Constraints<true> {}
