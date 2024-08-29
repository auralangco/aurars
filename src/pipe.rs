pub struct Pipe<T>(T);

impl<T> Pipe<T> {
    pub fn new(t: T) -> Self {
        Pipe(t)
    }

    pub fn pipe<B, F>(self, f: F) -> Pipe<B>
    where
        F: FnOnce(T) -> B,
    {
        Pipe(f(self.0))
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

#[macro_export]
macro_rules! pipe {
    {$expr:expr $(=> $f:expr)*} => {
        $crate::pipe::Pipe::new($expr) $(.pipe($f))*.into_inner()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pipe() {
        let result = pipe! {
            1
            => |x| x + 1
            => |x| x * 2
        };
        assert_eq!(result, 4);
    }
}
