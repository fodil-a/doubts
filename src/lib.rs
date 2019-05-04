/// Convenience, to write more explicit tests
/// # Requirements
/// The type of the expression that you are testing should derive "Debug"
///
/// # Check your own property
/// You can write assert_that to check the value of a property (using an operator between the property name and the expected value)
/// ```
///  #[test]
///  fn test() {
///      let v = vec![1];
///      assert_that!(v, has len <= 2)
///  }
/// ```
/// And if the specified property is wrong, a clean message would be printed. For example:
/// ```
///  #[test]
///  fn test() {
///      let v = vec![1];
///      assert_that!(v, has len >= 2)
///  }
/// ```
/// would fail with a message like  ``Expected `vec![1]`=[1] to have len >= 2, but len = 1.``
///
/// This macro works with two conventions: methods names should follow name patterns like `is_${property}` and `${property}s(...)`
/// That way, it is possible to have more readable messages.
///
/// # Check an existing property for the struct
/// First, the methods with name like `is_${property}()`, for example `is_empty` from Vec. These methods take 0 arguments, and returns as property.
/// For example:
/// ```
///  #[test]
///  fn test() {
///      let v = vec![1];
///      assert_that!(v, is_empty)
///  }
/// ```
///  Would fail with a message like ``Expected `v`=[1] to be empty.``
///
/// # Check an existing property for the struct with a given list of arguments
/// In the same way, any method telling if a property is right for a given list of arguments, should follow the pattern `${property}s(...)`, for example `contains` from Vec. These methods take at least one argument and give a bool as a result.
/// For example:
/// ```
///  #[test]
///  fn test() {
///      let v = vec![1];
///      assert_that!(v, contains &2)
///  }
///  ```
/// Would fail with a message like ``Expected `v`=[1] to contain 2.``
///
/// You can pass as much arguments as needed (at least one).
/// For example testing a struct like Pair:
/// ```
/// #[derive(Debug)]
/// struct Pair(i32,i32);
/// impl Pair{
///     fn contains(&self, i:i32, j:i32) -> bool{
///         // ...
///         false
///     }
/// }
/// ```
/// You can now write the following test:
/// ```
/// #[test]
/// fn test() {
///     let s = Pair(1, 2);
///     assert_that!(s, contains 2, 3)
/// }
/// ```
/// Would fail with a message like ``Expected `s`=Pair(1, 2) to contain 2,3.``

#[macro_export]
macro_rules! assert_that {
    ($e: expr, has $i:ident $tt:tt $n: expr) => {
        {
            let x = $e.$i();
            assert!(x $tt $n, "Expected `{}`={:?} to have {} {} {}, but {} = {}.",
                stringify!($e), $e,
                stringify!($i), stringify!($tt), $n,
                stringify!($i), x)
        }
    };
    ($e: expr, $i:ident) => {
        {
            let method_name = stringify!($i);
             {
                assert!($e.$i(), "Expected `{}`={:?} to be {}.",
                    stringify!($e), $e,
                    if method_name.starts_with("is_") {&method_name[3..]}else{method_name}
                )
            }
        }
    };
    ($e: expr, $i:ident $($n: expr),+) => {
        {
            let method_name = stringify!($i);
             {
                assert!($e.$i($($n),+), "Expected `{}`={:?} to {} {}.",
                    stringify!($e), $e,
                    if method_name.ends_with('s'){&method_name[..method_name.len()-1]}else{method_name},
                    &([$($n.to_string()),+]).join(",")
                )
            }
        }
    };
}