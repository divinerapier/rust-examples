pub(crate) fn call_function<F>(f: F)
where
    F: Fn(),
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

fn main() {
    let x = 3;
    call_function(|| {
        println!("anonymous function. {}", x);
    });
    println!("x = {}", x);
    call_function(foo);

    call_function2(3, bar::<i32>);
    call_function2("3", bar::<&'static str>);
}
