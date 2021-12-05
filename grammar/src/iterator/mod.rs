mod from_iterator;
mod iter;

pub struct Fibonacci {
    current: u64,
    next: u64,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        let next = self.current.wrapping_add(self.next);
        self.current = self.next;
        self.next = next;
        Some(current)
    }
}

pub struct EvenIndexSelectedIterator<I> {
    iterator: I,
}

impl<I> Iterator for EvenIndexSelectedIterator<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()?;
        self.iterator.next()
    }
}

impl<I> EvenIndexSelectedExtension for I where I: Iterator + Sized {}

pub trait EvenIndexSelectedExtension: Sized {
    fn into_even_index_selected_iterator(self) -> EvenIndexSelectedIterator<Self>
    where
        Self: Sized,
    {
        EvenIndexSelectedIterator { iterator: self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let fibo = Fibonacci::default().into_even_index_selected_iterator();
        let mut count = 0;
        for i in fibo {
            count += 1;
            println!("{}", i);
            if count >= 100 {
                break;
            }
        }
    }

    #[test]
    fn test2() {
        let iter = (0..100).into_even_index_selected_iterator();
        for i in iter {
            println!("{}", i);
        }
    }
}
