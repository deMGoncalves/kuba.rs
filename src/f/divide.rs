/// Divides two numbers
///
/// # Examples
///
/// ```
/// use kuba::f;
///
/// let result = f::divide(3, 2);
/// assert_eq!(result, 1);
/// ```
///
/// # Panic
///
/// The function panics if the second argument is zero
///
/// ```rust,should_panic
/// use kuba::f;
///
/// let result = f::divide(3, 0);
/// ```
pub fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("Divide by zero");
    }

    x / y
}
