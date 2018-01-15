// MIT License

// Copyright (c) 2018 Jerome Froelich

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

pub fn boyer_moore<T: Eq>(stream: &[T]) -> Option<&T> {
    let n = stream.len();
    if n == 0 {
        return None;
    }

    let mut canidate = &stream[0];
    let mut count = 1;
    for current in &stream[1..] {
        if count == 0 {
            canidate = current;
            count += 1;
            continue;
        }

        if current == canidate {
            count += 1;
        } else {
            count -= 1;
        }
    }

    if count == 0 {
        return None;
    }

    let mut total = 0;
    for (i, current) in stream.iter().enumerate() {
        if current == canidate {
            total += 1;
            if total > n / 2 {
                return Some(&canidate);
            }
            continue;
        }

        // Return early if it is not possible for canidate to be the majority element.
        if total + (n - i - 1) <= n / 2 {
            return None;
        }
    }

    None
}

#[derive(Debug)]
struct Bucket<'a, T: 'a + Eq> {
    element: Option<&'a T>,
    count: usize,
}

impl<'a, T: 'a + Eq> Bucket<'a, T> {
    fn new() -> Bucket<'a, T> {
        Bucket {
            element: None,
            count: 0,
        }
    }

    fn push(&mut self, other: &'a T) {
        match self.element {
            Some(e) => {
                if e != other {
                    panic!("invalid push: element being pushed is not equal to current element \
                            of the bucket");
                }
                self.count += 1
            }
            None => {
                self.element = Some(other);
                self.count = 1;
            }
        }
    }

    fn pop(&mut self) -> &'a T {
        match self.element {
            Some(e) => {
                self.count -= 1;
                if self.count == 0 {
                    self.element = None
                }
                e
            }
            None => panic!("cannot pop an element from an empty bucket"),
        }
    }

    fn empty(&self) -> bool {
        match self.element {
            Some(_) => false,
            None => true,
        }
    }
}

pub fn fischer_salzberg<T: Eq>(stream: &[T]) -> Option<&T> {
    let n = stream.len();
    if n == 0 {
        return None;
    }

    let mut list: Vec<&T> = Vec::with_capacity(n);
    let mut bucket = Bucket::new();

    for current in stream {
        match list.last() {
            Some(canidate) => {
                if current == *canidate {
                    bucket.push(current);
                    continue;
                }
            }
            None => {}
        }

        list.push(current);
        if !bucket.empty() {
            list.push(bucket.pop());
        }
    }

    let mut skip_next = false;
    let canidate = list.last().unwrap();
    for current in list.iter().rev() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if current == canidate {
            skip_next = true;
            continue;
        }

        if bucket.empty() {
            return None;
        }
        bucket.pop();
    }

    return Some(canidate);
}

#[derive(Debug)]
struct Tuple<'a, T: 'a> {
    first: &'a T,
    second: &'a T,
    count: usize,
}

impl<'a, T: 'a> Tuple<'a, T> {
    fn new(first: &'a T, second: &'a T, count: usize) -> Tuple<'a, T> {
        Tuple {
            first: first,
            second: second,
            count: count,
        }
    }
}

// TODO: Implement Matula tournament algorithm.
fn matula_tournament<T: Eq>(stream: &[T]) -> Option<&T> {
    None
}

#[cfg(test)]
mod tests {
    use super::{boyer_moore, fischer_salzberg, matula_tournament};

    // TODO: Use property tests for the tests below.
    #[test]
    fn empty_vector() {
        let v: Vec<isize> = Vec::new();

        assert_eq!(None, boyer_moore(&v));
        assert_eq!(None, fischer_salzberg(&v));
        // assert_eq!(None, matula_tournament(&v));
    }

    #[test]
    fn vector_with_single_element() {
        let v = vec![3];

        let expected = 3;
        assert_eq!(Some(&expected), boyer_moore(&v));
        assert_eq!(Some(&expected), fischer_salzberg(&v));
        // assert_eq!(Some(&expected), matula_tournament(&v));
    }

    #[test]
    fn vector_with_majority() {
        let v = vec![1, 1, 4, 5, 6, 4, 4, 4, 8, 4, 4];

        let expected = 4;
        assert_eq!(Some(&expected), boyer_moore(&v));
        assert_eq!(Some(&expected), fischer_salzberg(&v));
        // assert_eq!(Some(&expected), matula_tournament(&v));
    }

    #[test]
    fn vector_with_no_majority() {
        let v = vec![3, 6, 3, 4, 3, 3, 7, 4, 8, 5, 6];

        assert_eq!(None, boyer_moore(&v));
        assert_eq!(None, fischer_salzberg(&v));
        // assert_eq!(None, matula_tournament(&v));
    }

    #[test]
    fn vector_with_alternating_majority() {
        let v = vec![1, 2, 1, 2, 1, 2, 1];

        let expected = 1;
        assert_eq!(Some(&expected), boyer_moore(&v));
        assert_eq!(Some(&expected), fischer_salzberg(&v));
        // assert_eq!(Some(&expected), matula_tournament(&v));
    }
}
