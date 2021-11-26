pub fn consumer_box<T>(b: Box<T>) {}

pub fn give_box<T>() -> Box<T>
where
    T: Default,
{
    Box::new(T::default())
}

pub fn update_box<T>(b: &mut Box<T>, v: T) {
    **b = v;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update() {
        let mut a = give_box::<i32>();
        update_box(&mut a, 3);
        assert_eq!(Box::new(3), a);
    }
}
