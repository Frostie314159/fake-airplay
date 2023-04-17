#[doc = include_str!("../README.md")]
use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use log::{error, info};
use rand::Rng;
use tokio::sync::mpsc::{error::TryRecvError, Sender};
use zeroconf::{
    prelude::TEventLoop, service::TMdnsService, txt_record::TTxtRecord, MdnsService, ServiceType,
    TxtRecord,
};

const DEFAULT_TXT_RECORD: [(&str, &str); 3] = [
    ("srcvers", "377.17.24.6"), // This isn't default, but known to work.
    ("flags", "0x244"),         // Same as the srcvers
    ("features", "0x7F8AD0,0x38BCB46"),
];
/// This decides, how the service will apear on the apple device.
pub enum DeviceType {
    AppleTV,
    Other(Option<String>),
}
impl ToString for DeviceType {
    fn to_string(&self) -> String {
        match self {
            DeviceType::AppleTV => "AppleTV".to_string(), // For some reason this is enough.
            DeviceType::Other(model) => model.clone().unwrap_or("ApertureTV".to_string()),
        }
    }
}
impl From<&str> for DeviceType {
    fn from(value: &str) -> Self {
        match value {
            "AppleTV" => Self::AppleTV,
            _ => Self::Other(Some(value.to_string())),
        }
    }
}
pub struct Service {
    handle: JoinHandle<()>,
    tx: Sender<()>,
}
impl Service {
    pub fn new(handle: JoinHandle<()>, tx: Sender<()>) -> Self {
        Self { handle, tx }
    }
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
    pub fn kill(self) {
        match self.tx.blocking_send(()) {
            Ok(_) => info!("Send thread shutdown cmd."),
            Err(_) => {
                error!("Failed to send shutdown cmd, thread might already be dead.");
                return;
            }
        }
        match self.handle.join() {
            Ok(_) => info!("Successfully shutdown service thread."),
            Err(_) => {
                error!("Failed to join thread, it might be dead already.");
                return;
            }
        }
    }
}
/// Creates the service thread and returns the handle.
pub fn create_service(name: String, device_type: DeviceType) -> Service {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
    let handle = std::thread::Builder::new().name("ServiceThread".to_string()).spawn(move || {
        let mut service = MdnsService::new(
            ServiceType::new("airplay", "tcp")
                .expect("Failed to register service. Is the avahi-daemon running?"),
            rand::thread_rng().gen_range(0..u16::MAX),
        );
        service.set_name(&name);
        service.set_registered_callback(Box::new(|_, _| {}));

        let mut txt_record = TxtRecord::new();
        txt_record
            .insert(
                "deviceid",
                &format!(
                    "00:80:41:13:37:{:2x}",
                    rand::thread_rng().gen_range(0..u8::MAX)
                ),
            )
            .unwrap();
        txt_record
            .insert("model", &device_type.to_string())
            .unwrap();
        DEFAULT_TXT_RECORD
            .iter()
            .for_each(|(k, v)| txt_record.insert(k, v).unwrap());
        service.set_txt_record(txt_record);

        let ev_loop = match service.register() {
            Ok(ev_loop) => {
                info!("Successfully registered service: {name}", );
                ev_loop
            },
            Err(err) => {
                error!("Failed to register service: {name}.");
                panic!("{}", err.to_string())
            },
        };

        while let Err(TryRecvError::Empty) = rx.try_recv() {
            ev_loop.poll(Duration::from_millis(0)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    let handle = match handle {
        Ok(handle) => {
            info!("Successfully initalized service thread.");
            handle
        },
        Err(err) => {
            error!("Failed to initialize service thread.");
            panic!("{err}")
        },
    };
    Service::new(handle, tx)
}
