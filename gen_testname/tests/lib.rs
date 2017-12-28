// questions: https://www.reddit.com/r/rust/comments/7m0uir/hey_rustaceans_got_an_easy_question_ask_here/drujmjj/

#![feature(test)]

// there are two functions: foo() and bar()
fn foo(n: u32)  -> u32 {
    n
}

fn bar(n: u32) -> u32 {
    n
}

#[cfg(test)]
mod test {
    use super::*;

    // I'd like to write test cases for both of them. I can write four tests to test
    // foo(1), bar(1), foo(2) and bar(2) as below.
    #[test]
    fn foo_1() {
        assert_eq!(1, foo(1));
    }

    #[test]
    fn bar_1() {
        assert_eq!(1, bar(1));
    }

    #[test]
    fn foo_2() {
        assert_eq!(2, foo(2));
    }

    #[test]
    fn bar_2() {
        assert_eq!(2, bar(2));
    }

    // But the above tests are too verbose. I'd like to write a macro to reduce
    // duplicate code. However, the following macro build failed. I googled and
    // found currently Rust didn't support create new ident in macros:
    // - Tracking issue for `concat_idents`: https://github.com/rust-lang/rust/issues/29599
    // - Eager expansion of macros: https://github.com/rust-lang/rfcs/pull/1628
    //
    // My question is how do you guys write tests elegantly in such situation?
    // How to you "generate" unique tests name based on tested functions and its
    // arguments? Thanks!

    macro_rules! test_function_with_n {
        ($func:ident, $n:expr) => {
            #[test]
            fn $func_$n {
                assert_eq!($n, $func($n));
            }
        }
    }
    test_function_with_n!(foo, 1);
    test_function_with_n!(bar, 1);
    test_function_with_n!(foo, 2);
    test_function_with_n!(bar, 2);

}
