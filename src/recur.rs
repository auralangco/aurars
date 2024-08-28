use std::ops::ControlFlow;

/// An immutable loop control flow abstraction. Takes `initial`, every iteration calls `f` with the current value, 
/// and returns either a continue value or a break value. Once the break value is returned, the loop stops and the break value is returned.
/// 
/// The implementation of the Aura `loop` function. Takes a initial value, and a function that returns either a continue value or a break value.
pub fn recur<C, B, F>(initial: C, f: F) -> B
where
    F: Fn(C) -> ControlFlow<B, C>,
{
    let mut val = initial;

    loop {
        match f(val) {
            ControlFlow::Continue(c) => val = c,
            ControlFlow::Break(b) => return b,
        }
    }
}

/// Wraps a value in a [`std::ops::ControlFlow::Break`]
#[macro_export]
macro_rules! recur_break {
    ($expr:expr) => {
        std::ops::ControlFlow::Break($expr)
    };
}

/// Wraps a value in a [`std::ops::ControlFlow::Continue`]
#[macro_export]
macro_rules! recur_continue {
    ($expr:expr) => {
        std::ops::ControlFlow::Continue($expr)
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_recur() {
        let result = recur(0, |i| {
            if i < 10 {
                recur_continue!(i + 1)
            } else {
                recur_break!(i.to_string())
            }
        });

        assert_eq!(result, "10");
    }
}