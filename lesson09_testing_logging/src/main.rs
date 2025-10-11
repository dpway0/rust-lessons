use env_logger::Env;
use log::{debug, error, info};

#[macro_export]
macro_rules! check {
    ($cond:expr) => {
        if !$cond {
            ::log::info!(
                "check failed: `{}` @{}:{}",
                ::core::stringify!($cond),
                ::core::file!(),
                ::core::line!()
            );
            panic!("check failed: {}", ::core::stringify! {$cond});
        }
    };
}

#[macro_export]
macro_rules! check_eq {
    // 1) Pattern: capture two arbitrary expressions
    ($left:expr, $right:expr) => {{
        // 2) Evaluate each expression exactly once and keep *references*
        //    - prevents double evaluation (side effects / expensive calls)
        //    - avoids moving ownership; borrowing is cheap
        let (__l, __r) = (&$left, &$right);

        // 3) Compare the *values* (dereference the references)
        if *__l != *__r {
            // 4) Log a concise failure message with call site info
            ::log::info!(
                "check_eq failed @{}:{}; {} != {}; left={:?}, right={:?}",
                // 5) file!()/line!() point to the macro *call site*
                ::core::file!(),
                ::core::line!(),
                // 6) stringify! prints the original source expressions
                ::core::stringify!($left),
                ::core::stringify!($right),
                // 7) Print evaluated values (requires Debug)
                __l,
                __r
            );

            // 8) Fail like assert_eq! (unwind/abort per your build config)
            panic!("check_eq failed");
        }
    }};
}

fn add(a: i32, b: i32) -> i32 {
    info!("Adding {a} and {b}");
    a + b
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn multiply(a: i32, b: i32) -> i32 {
    info!("Multiplying {a} and {b}");
    a * b
}

fn main() {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    info!("Starting app");
    let sum = add(4, 5);
    debug!("Sum = {sum}");

    match divide(10, 0) {
        Some(v) => info!("Result = {v}"),
        None => error!("Division failed"),
    }

    let prod = multiply(2, 3);
    info!("Prod = {prod}");

    dbg!(sum, prod);
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static INIT: Lazy<()> = Lazy::new(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });

    #[test]
    fn test_add() {
        Lazy::force(&INIT);
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_divide_by_zero() {
        Lazy::force(&INIT);
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    fn test_multiply_basic() {
        Lazy::force(&INIT);
        assert_eq!(multiply(2, 3), 6);
    }

    #[test]
    fn test_check_pass() {
        Lazy::force(&INIT);
        let x = 3;
        check!(x > 0);
    }

    #[test]
    #[should_panic(expected = "check failed")]
    fn test_check_fail() {
        Lazy::force(&INIT);
        let x = 3;
        check!(x == 4);
    }

    #[test]
    fn test_check_eq_pass() {
        Lazy::force(&INIT);
        check_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic(expected = "check_eq failed")]
    fn test_check_eq_fail() {
        Lazy::force(&INIT);
        check_eq!(multiply(2, 3), 5);
    }
}
