import Foundation

class InteropLib {
    let instance: UnsafeMutableRawPointer
    
    init() {
        instance = interop_init()
    }
    
    deinit {
        interop_destroy(instance)
    }
    
    func call() {
        do_job(instance, { (msg: UInt32) in
            print("SwiftLib: callback called");
            self.callback()
        })
    }
    
    func callback() {
        print("SwiftLib: callback()");
    }
}
