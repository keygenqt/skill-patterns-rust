
use std::{thread, time};

use crate::singleton::Singleton;

mod singleton {

    use std::sync::{atomic::{AtomicUsize, Ordering}, LazyLock, Mutex, MutexGuard};

    static SINGLETON: LazyLock<Mutex<Singleton>> = LazyLock::new(|| Mutex::new(Singleton {
        id: AtomicUsize::new(1).fetch_add(1, Ordering::Relaxed) as u64
    }));

    pub struct Singleton {
        pub id: u64,
    }

    impl Singleton {
        pub fn instance() -> MutexGuard<'static, Singleton> {
            SINGLETON.lock().unwrap()
        }
    }

    unsafe impl Send for Singleton {}
}

fn main() {
    // async
    let handle_singleton3 = thread::spawn(move || {
        loop {
            // delay
            thread::sleep(time::Duration::from_millis(500));
            // impl
            let singleton3 = Singleton::instance();
            println!("singleton3 ID: {}", singleton3.id);
            // end
            break;
        }
    });
    // scope
    {
        // impl
        let singleton1 = Singleton::instance();
        println!("singleton1 ID: {}", singleton1.id);
    }
    {
        // impl
        let singleton2 = Singleton::instance();
        println!("singleton2 ID: {}", singleton2.id);
    }
    // load async
    handle_singleton3.join().unwrap();
}
