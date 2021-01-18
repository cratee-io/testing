// This macro is ported from 
// https://github.com/dtolnay/inventory/blob/0.1.10/src/lib.rs#L370
#[macro_export]
macro_rules! submit {
    ($($value:tt)*) => {
        $crate::inventory_impl::submit! {
            $($value)*
        }
    }
}