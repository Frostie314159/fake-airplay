use std::{sync::{Arc, atomic::AtomicBool}, thread, time::Duration};

use fake_airplay::{create_service, DeviceType};

fn main() {
    let service = create_service(
        std::env::args()
            .nth(1)
            .expect("A service name is required!"),
        match std::env::args().nth(2) {
            Some(device_type) => DeviceType::from(device_type.as_str()),
            None => DeviceType::Other(None),
        },
    );
    let should_close = Arc::new(AtomicBool::new(false));
    {
        let should_close_clone = should_close.clone();
        ctrlc::set_handler(move|| should_close_clone.store(true, std::sync::atomic::Ordering::Relaxed)).unwrap();
    }
    while !should_close.load(std::sync::atomic::Ordering::Acquire) {
        thread::sleep(Duration::from_millis(500));
    }    
    service.kill();
}
