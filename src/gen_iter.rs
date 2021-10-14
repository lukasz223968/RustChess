

#![feature(generators, generator_trait)]

// slightly modified gen-iter crate by tinaun: https://github.com/tinaun/gen-iter

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

// MIT License

// Copyright (c) [year] [fullname]

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.