/// Adds two values
///
/// # Examples
///
/// ```
/// use kuba::f;
///
/// let result = f::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

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
    if x == 0 {
        panic!("Divide by zero");
    }

    x / y
}
