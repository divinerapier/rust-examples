use std::rc::Rc;

pub fn into_rc<T>(v: T) -> Rc<T> {
    Rc::new(v)
}

pub fn consumer_rc<T>(v: Rc<T>) {}

#[cfg(test)]
mod test {
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn test() {
        let v = into_rc(1);
        consumer_rc(v.clone());
        consumer_rc(v);
    }
}
