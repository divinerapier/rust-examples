use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub fn send_box() {
    let v = Box::new(1);
    std::thread::spawn(move || {
        let a = v;
    });
}

pub fn send_cell() {
    let v = Cell::new(1);
    std::thread::spawn(move || {
        let a = v;
    });
}

pub fn send_refcell() {
    let v = RefCell::new(1);
    std::thread::spawn(move || {
        let a = v;
    });
}

// pub fn send_rc() {
//     let v = Rc::new(1);
//     std::thread::spawn(move || {
//         let a = v;
//     });
// }

#[cfg(test)]
mod test {
    use super::*;
}
