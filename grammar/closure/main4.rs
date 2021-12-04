pub(crate) fn call_function<F>(mut f: F)
where
    F: FnOnce(),
{
    println!("before call_function::call_function");
    f();
    println!("after call_function::call_function");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}

pub(crate) fn call_function2<F, T>(arg: T, f: F)
where
    F: Fn(T),
{
    println!("before call_function::call_function");
    f(arg);
    println!("after call_function::call_function");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}

fn foo() {
    println!("function foo");
}

fn bar<T>(t: T) {
    println!("type: {}", std::any::type_name::<T>())
}

fn baz<T>(t: T)
where
    T: 'static,
{
    println!("type: {}", std::any::type_name::<T>())
}

fn main() {
    let mut x = Box::new(3);
    call_function(|| {
        *x = 9;
        println!("anonymous function. {}", x);
        baz(x);
    });
    call_function(foo);

    call_function2(3, bar::<i32>);
    call_function2("3", bar::<&'static str>);
}
