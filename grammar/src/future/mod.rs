use futures::future::join3;
use futures::{join, pin_mut};
use std::future::Future;
use std::os::unix::fs::FileExt;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

#[pin_project::pin_project]
pub struct Join2<F1, O1, F2, O2>
where
    F1: Future<Output = O1>,
    F2: Future<Output = O2>,
{
    #[pin]
    fu1: F1,

    #[pin]
    fu2: F2,

    r1: Option<O1>,
    r2: Option<O2>,
}

// impl <F1, O1, F2, O2>  !Unpin for Join2<F1, O1, F2, O2>;

impl<F1, O1, F2, O2> Join2<F1, O1, F2, O2>
where
    F1: Future<Output = O1>,
    F2: Future<Output = O2>,
{
    pub fn new(fu1: F1, fu2: F2) -> Join2<F1, O1, F2, O2> {
        Self {
            fu1,
            fu2,
            r1: None,
            r2: None,
        }
    }
}

impl<F1, O1, F2, O2> Future for Join2<F1, O1, F2, O2>
where
    F1: Future<Output = O1> + Unpin,
    F2: Future<Output = O2> + Unpin,
{
    type Output = (O1, O2);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        if me.r1.as_ref().is_none() {
            let r1 = ready!(me.fu1.poll(cx));
            *me.r1 = Some(r1);
        }
        if me.r2.as_ref().is_none() {
            let r2 = ready!(me.fu2.poll(cx));
            *me.r2 = Some(r2);
        }

        Poll::Ready((me.r1.take().unwrap(), me.r2.take().unwrap()))
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
        let f = join(f1, f2);
        assert_eq!(block_on(f), (1, 2));
    }
}
