use futures::future::{join3, maybe_done, MaybeDone};
use futures::{join, pin_mut};
use std::future::Future;
use std::os::unix::fs::FileExt;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

#[pin_project::pin_project]
pub struct Join2<F1, F2>
where
    F1: Future,
    F2: Future,
{
    #[pin]
    fu1: MaybeDone<F1>,
    #[pin]
    fu2: MaybeDone<F2>,
}

// impl <F1, O1, F2, O2>  !Unpin for Join2<F1, O1, F2, O2>;

impl<F1, F2> Join2<F1, F2>
where
    F1: Future,
    F2: Future,
{
    pub fn new(fu1: F1, fu2: F2) -> Join2<F1, F2> {
        Self {
            fu1: maybe_done(fu1),
            fu2: maybe_done(fu2),
        }
    }
}

impl<F1, F2> Future for Join2<F1, F2>
where
    F1: Future,
    F2: Future,
{
    type Output = (F1::Output, F2::Output);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        let mut all_ready = true;
        all_ready &= me.fu1.as_mut().poll(cx).is_ready();
        all_ready &= me.fu2.as_mut().poll(cx).is_ready();

        if all_ready {
            Poll::Ready((me.fu1.take_output().unwrap(), me.fu2.take_output().unwrap()))
        } else {
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::executor::block_on;
    use futures::future::{join, ok};

    #[test]
    fn test() {
        // let fu1 = Poll::Ready(1);
        // let fu2 = Poll::Ready(2.);
        // let mut f = Join2::new(fu1, fu2);
        let f1 = async { 1 };
        let f2 = async { 2 };
        // let f = join2(f);
        // let f = join(f1, f2);
        let f = Join2::new(f1, f2);
        assert_eq!(block_on(f), (1, 2));
    }
}
