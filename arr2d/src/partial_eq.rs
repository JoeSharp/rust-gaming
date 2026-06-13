use crate::Arr2d;

impl<T> PartialEq for Arr2d<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.contents == other.contents
    }
}

#[cfg(test)]
mod tests {
    use crate::Arr2d;
    use crate::test_fixtures::TestBool;

    #[test]
    fn test_eq() {
        let a: Arr2d<TestBool> =
            Arr2d::from_2d_array(vec![vec![TestBool::from(true), TestBool::from(false)]]);
        let b: Arr2d<TestBool> =
            Arr2d::from_2d_array(vec![vec![TestBool::from(true), TestBool::from(false)]]);

        assert_eq!(a, b);
    }

    #[test]
    fn test_neq() {
        let a: Arr2d<TestBool> =
            Arr2d::from_2d_array(vec![vec![TestBool::from(true), TestBool::from(false)]]);
        let b: Arr2d<TestBool> = Arr2d::new();

        assert_ne!(a, b);
    }
}
