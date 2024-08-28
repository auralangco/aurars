/// A type that represents one of two possible values.
/// 
/// This type is roughly equivalent to `Result` but without the error meaning.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Either<L, R> {
    /// Contains the `Left` value of a [`Either`].
    Left(L),
    /// Contains the `Right` value of a [`Either`].
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Returns `true` if the value is `Left`.
    pub fn is_left(&self) -> bool {
        match self {
            Either::Left(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the value is `Right`.
    pub fn is_right(&self) -> bool {
        match self {
            Either::Right(_) => true,
            _ => false,
        }
    }

    /// Converts from `Either<L, R>` to `Option<L>`.
    /// 
    /// Converts `self` into an `Option<L>`, consuming `self`,
    /// and discarding the `Right` value, if `Right`.
    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    /// Converts from `Either<L, R>` to `Option<R>`.
    /// 
    /// Converts `self` into an `Option<R>`, consuming `self`,
    /// and discarding the `Left` value, if `Left`.
    pub fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    /// Converts from `Option<L>` to `Either<L, ()>`.
    /// 
    /// Converts `option` into an `Either<L, ()>`, consuming `option`,
    pub fn as_left(option: Option<L>) -> Either<L, ()> {
        match option {
            Some(l) => Either::Left(l),
            None => Either::Right(()),
        }
    }

    /// Converts from `Option<R>` to `Either<(), R>`.
    /// 
    /// Converts `option` into an `Either<(), R>`, consuming `option`,
    pub fn as_right(option: Option<R>) -> Either<(), R> {
        match option {
            Some(r) => Either::Right(r),
            None => Either::Left(()),
        }
    }

    /// Unwraps the value if it is `Left`
    /// 
    /// # Panics
    /// 
    /// Panics if the value is `Right`
    pub fn unwrap_left(self) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => panic!("called `Either::unwrap_left()` on a `Right` value"),
        }
    }

    /// Unwraps the value if it is `Right`
    /// 
    /// # Panics
    /// 
    /// Panics if the value is `Left`
    pub fn unwrap_right(self) -> R {
        match self {
            Either::Left(_) => panic!("called `Either::unwrap_right()` on a `Left` value"),
            Either::Right(r) => r,
        }
    }

    /// Unwraps the value if it is `Left`, otherwise panics with `msg`
    /// 
    /// # Panics
    /// 
    /// Panics with `msg` if the value is `Right`
    pub fn expected_left(self, msg: &str) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => panic!("{}", msg),
        }
    }

    /// Unwraps the value if it is `Right`, otherwise panics with `msg`
    /// 
    /// # Panics
    /// 
    /// Panics with `msg` if the value is `Left`
    pub fn expected_right(self, msg: &str) -> R {
        match self {
            Either::Left(_) => panic!("{}", msg),
            Either::Right(r) => r,
        }
    }

    /// Maps the value of `Either<L, R>` to `Either<T, U>` by applying a function `f` to the `Left` value, if `Left`,
    /// or applying `g` to the `Right` value, if `Right`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aurars::either::Either;
    ///
    /// let left: Either<i32, String> = Either::Left(42);
    ///
    /// let mapped = left.map_either(|x| x * 2, |s| s.len());
    ///
    /// assert_eq!(mapped, Either::Left(84));
    /// ```
    pub fn map_either<T, U, F: FnOnce(L) -> T, G: FnOnce(R) -> U>(self, f: F, g: G) -> Either<T, U> {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(g(r)),
        }
    }

    /// Maps the value of `Either<L, R>` to `Either<U, R>` by applying a function to the `Left` value, if `Left`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aurars::either::Either;
    ///
    /// let left: Either<i32, ()> = Either::Left(42);
    /// let right: Either<(), &str> = Either::Right("hello");
    ///
    /// let mapped_left = left.map_left(|x| x * 2);
    /// let mapped_right = right.map_left(|x| x * 2);
    ///
    /// assert_eq!(mapped_left, Either::Left(84));
    /// assert_eq!(mapped_right, Either::Right("hello"));
    /// ```
    pub fn map_left<U, F: FnOnce(L) -> U>(self, f: F) -> Either<U, R> {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    /// Maps the value of `Either<L, R>` to `Either<L, U>` by applying a function to the `Right` value, if `Right`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aurars::either::Either;
    ///
    /// let left: Either<i32, ()> = Either::Left(42);
    /// let right: Either<(), &str> = Either::Right("hello");
    ///
    /// let mapped_left = left.map_right(|s| s.len());
    /// let mapped_right = right.map_right(|s| s.len());
    ///
    /// assert_eq!(mapped_left, Either::Left(42));
    /// assert_eq!(mapped_right, Either::Right(5));
    /// ```
    pub fn map_right<U, F: FnOnce(R) -> U>(self, f: F) -> Either<L, U> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(f(r)),
        }
    }

    /// Swaps the value of `Either<L, R>` to `Either<R, L>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aurars::either::Either;
    ///
    /// let left: Either<i32, String> = Either::Left(42);
    /// let right: Either<String, i32> = Either::Right(84);
    ///
    /// let swapped_left = left.swap();
    /// let swapped_right = right.swap();
    ///
    /// assert_eq!(swapped_left, Either::Right(42));
    /// assert_eq!(swapped_right, Either::Left(84));
    /// ```
    pub fn swap(self) -> Either<R, L> {
        match self {
            Either::Left(l) => Either::Right(l),
            Either::Right(r) => Either::Left(r),
        }
    }
}

impl<L, R> From<Result<L, R>> for Either<L, R> {
    fn from(result: Result<L, R>) -> Self {
        match result {
            Ok(l) => Either::Left(l),
            Err(r) => Either::Right(r),
        }
    }
}

impl<L, R> From<Either<L, R>> for Result<L, R> {
    fn from(either: Either<L, R>) -> Self {
        match either {
            Either::Left(l) => Ok(l),
            Either::Right(r) => Err(r),
        }
    }
}

/// A macro to match on an `Either::Left` value and return early if the value is `Right`.
/// 
/// Similar to `?` operator for `Result`.
#[macro_export]
macro_rules! try_left {
    ($expr:expr) => {
        match $expr {
            $crate::prelude::Left(val) => val,
            $crate::prelude::Right(val) => return $crate::prelude::Right(val),
        }
    };
}

/// A macro to match on an `Either::Right` value and return early if the value is `Left`.
/// 
/// Similar to `?` operator for `Result`.
#[macro_export]
macro_rules! try_right {
    ($expr:expr) => {
        match $expr {
            $crate::prelude::Right(val) => val,
            $crate::prelude::Left(val) => return $crate::prelude::Left(val),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_either() {
        let left: Either<i32, ()> = Either::Left(42);
        let right: Either<(), &str> = Either::Right("hello");

        assert_eq!(left, Either::Left(42));
        assert_eq!(right, Either::Right("hello"));
    }

    #[test]
    fn test_either_copy() {
        let e: Either<i32, ()> = Either::Left(42);
        let e_copy = e;

        assert_eq!(e, e_copy);
    }

    #[test]
    fn test_either_clone() {
        let e: Either<String, ()> = Either::Left(String::from("hello"));
        let e_clone = e.clone();
        let e_moved = e;
        
        assert_eq!(e_moved, e_clone);
    } 

    #[test]
    fn test_try_left() {
        use crate::prelude::*;
        let left: Either<i32, ()> = Either::Left(42);

        let left_val = (|| {
            let left = try_left!(left); 
            Left(left + 1)
        })();

        assert_eq!(left_val, Left(43));
    }
}