use std::cell::Cell;

pub fn get_cell<T>(v: T) -> Cell<T> {
    Cell::new(v)
}

pub fn update_cell<T>(c: &Cell<T>, v: T) {
    c.set(v);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let c = get_cell(3);
        assert_eq!(c, Cell::new(3));
        update_cell(&c, 2);
        assert_eq!(c, Cell::new(2));
    }
}
