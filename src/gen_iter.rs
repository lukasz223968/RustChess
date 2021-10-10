

#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::iter::Iterator;
use std::marker::Unpin;
use std::pin::Pin;

/// a iterator that holds an internal generator representing
/// the iteration state
#[derive(Copy, Clone, Debug)]
pub struct GenIter<T>(pub T)
where
    T: Generator<Return = ()> + Unpin;

impl<T> Iterator for GenIter<T>
where
    T: Generator<Return = ()> + Unpin,
{
    type Item = T::Yield;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match Pin::new(&mut self.0).resume(()) {
            GeneratorState::Yielded(n) => Some(n),
            GeneratorState::Complete(()) => None,
        }
    }
}

impl<G> From<G> for GenIter<G>
where
    G: Generator<Return = ()> + Unpin,
{
    #[inline]
    fn from(gen: G) -> Self {
        GenIter(gen)
    }
}


#[macro_export]
macro_rules! gen_iter {
    ($block: block) => {
        $crate::gen_iter::GenIter(move || $block)
    }
}
