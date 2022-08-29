pub struct InteropLib {

}

impl InteropLib {
    fn start(&self) {
        println!("InteropLib: start()");
    }

    fn stop(&self) {
        println!("InteropLib: stop()");
    }

    fn call(&self) {
        println!("InteropLib: call()");
    }
}

fn wrap_instance(instance: *mut InteropLib) -> Box<InteropLib> {
    unsafe { Box::from_raw(instance) }
}

fn unwrap_instance(instance: Box<InteropLib>) {
    let _ = Box::into_raw(instance);
}

#[no_mangle]
pub extern fn interop_init() -> *mut InteropLib {
    println!("InteropLib: interop_init()");
    let instance = InteropLib{};
    let boxed = Box::new(instance);
    Box::into_raw(boxed)
}

#[no_mangle]
pub unsafe extern fn interop_destroy(instance: *mut InteropLib) {
    println!("InteropLib: interop_destroy()");
    let mut instance = Box::from_raw(instance);
    instance.stop();
}

#[no_mangle]
pub unsafe extern fn do_job(instance: *mut InteropLib, callback: unsafe extern fn(u32)) {
    println!("InteropLib: do_job()");
    let mut instance = wrap_instance(instance);
    instance.call();
    unwrap_instance(instance);
}