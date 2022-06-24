pub mod add;
pub mod divide;
pub mod multiply;
pub mod subtract;

#[doc(inline)]
pub use self::add::add;

#[doc(inline)]
pub use self::divide::divide;

#[doc(inline)]
pub use self::multiply::multiply;

#[doc(inline)]
pub use self::subtract::subtract;
