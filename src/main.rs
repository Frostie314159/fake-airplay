use fake_airplay::{create_service, DeviceType};

fn main() {
    create_service(
        std::env::args()
            .nth(1)
            .expect("A service name is required!"),
        match std::env::args().nth(2) {
            Some(device_type) => DeviceType::from(device_type.as_str()),
            None => DeviceType::Other(None),
        },
    )
    .join()
    .unwrap();
}
