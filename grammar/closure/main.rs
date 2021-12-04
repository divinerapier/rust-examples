pub(crate) fn call_function(f: fn()) {
    println!("before call_function::call_function");
    f();
    println!("after call_function::call_function");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}

pub(crate) fn call_function2<T>(arg: T, f: fn(T)) {
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
    call_function(|| {
        println!("anonymous function");
    });
    call_function(foo);

    call_function2(3, bar::<i32>);
    call_function2("3", bar::<&'static str>);
}
