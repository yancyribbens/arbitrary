//! Implementations of [`Arbitrary`] for foreign types.
//!
//! [`Arbitrary`]: crate::Arbitrary

macro_rules! implement_from_iter {
    ($outer:ident <$inner:ty> $(: $($bound:ident),+)?) => {
        impl<'a, A> crate::ArbitraryInRange<'a> for $outer<$inner>
        where
            A: crate::ArbitraryInRange<'a> $($(+ $bound)+)?,
        {
            type Bound = A::Bound;

            fn arbitrary_in_range<R>(
                u: &mut crate::Unstructured<'a>,
                range: &R,
            ) -> crate::Result<Self>
            where
                R: core::ops::RangeBounds<Self::Bound>,
            {
                u.arbitrary_in_range_iter(range)?.collect()
            }
        }

        impl<'a, A> crate::Arbitrary<'a> for $outer<$inner>
        where
            A: crate::Arbitrary<'a> $($(+ $bound)+)?,
        {
            fn arbitrary(u: &mut crate::Unstructured<'a>) -> crate::Result<Self> {
                u.arbitrary_iter()?.collect()
            }

            fn arbitrary_take_rest(u: crate::Unstructured<'a>) -> crate::Result<Self> {
                u.arbitrary_take_rest_iter()?.collect()
            }

            #[inline]
            fn size_hint(_depth: usize) -> (usize, Option<usize>) {
                (0, None)
            }
        }
    };
}

macro_rules! implement_new {
    ($ty:ty: $bound:ty) => {
        impl<'a> ArbitraryInRange<'a> for $ty {
            type Bound = $bound;

            fn arbitrary_in_range<R>(u: &mut Unstructured<'a>, range: &R) -> Result<Self>
            where
                R: core::ops::RangeBounds<Self::Bound>,
            {
                <$bound as ArbitraryInRange<'a>>::arbitrary_in_range(u, range).map(Self::new)
            }
        }

        impl<'a> Arbitrary<'a> for $ty {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                <$bound as Arbitrary<'a>>::arbitrary(u).map(Self::new)
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <$bound as Arbitrary<'a>>::size_hint(depth)
            }
        }
    };
}

macro_rules! implement_wrapped_new {
    ($outer:ident) => {
        implement_wrapped_new! {
            $outer @
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <A as crate::Arbitrary<'a>>::size_hint(depth)
            }
        }
    };
    ($outer:ident!) => {
        implement_wrapped_new! {
            $outer @
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                crate::size_hint::recursion_guard(depth, <A as Arbitrary>::size_hint)
            }
        }
    };
    ($outer:ident @ $size_hint:item) => {
        impl<'a, A> crate::ArbitraryInRange<'a> for $outer<A>
        where
            A: crate::ArbitraryInRange<'a>,
        {
            type Bound = A::Bound;

            fn arbitrary_in_range<R>(
                u: &mut crate::Unstructured<'a>,
                range: &R,
            ) -> crate::Result<Self>
            where
                R: core::ops::RangeBounds<Self::Bound>,
            {
                A::arbitrary_in_range(u, range).map($outer::new)
            }

            fn arbitrary_in_range_take_rest<R>(
                u: crate::Unstructured<'a>,
                range: &R,
            ) -> crate::Result<Self>
            where
                R: core::ops::RangeBounds<Self::Bound>,
            {
                A::arbitrary_in_range_take_rest(u, range).map($outer::new)
            }
        }

        impl<'a, A> crate::Arbitrary<'a> for $outer<A>
        where
            A: crate::Arbitrary<'a>,
        {
            fn arbitrary(u: &mut crate::Unstructured<'a>) -> crate::Result<Self> {
                A::arbitrary(u).map($outer::new)
            }

            fn arbitrary_take_rest(u: crate::Unstructured<'a>) -> crate::Result<Self> {
                A::arbitrary_take_rest(u).map($outer::new)
            }

            #[inline]
            $size_hint
        }
    };
}

mod alloc;
mod core;
mod std;
