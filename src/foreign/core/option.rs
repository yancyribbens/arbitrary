use {
    crate::{size_hint, Arbitrary, ArbitraryInRange, Result, Unstructured},
    core::ops::RangeBounds,
};

impl<'a, A> ArbitraryInRange<'a> for Option<A>
where
    A: ArbitraryInRange<'a>,
{
    type Bound = A::Bound;

    fn arbitrary_in_range<R>(u: &mut Unstructured<'a>, range: &R) -> Result<Self>
    where
        R: RangeBounds<Self::Bound>,
    {
        Ok(if <bool as Arbitrary<'a>>::arbitrary(u)? {
            Some(A::arbitrary_in_range(u, range)?)
        } else {
            None
        })
    }
}

impl<'a, A> Arbitrary<'a> for Option<A>
where
    A: Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(if <bool as Arbitrary<'a>>::arbitrary(u)? {
            Some(A::arbitrary(u)?)
        } else {
            None
        })
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::and(
            <bool as Arbitrary>::size_hint(depth),
            size_hint::or((0, Some(0)), <A as Arbitrary>::size_hint(depth)),
        )
    }
}
