
use std::fs;
use std::fs::File;
use std::path::Path;

use std::io::{Read, Write, stdout, stdin};
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

use mpcrust::window::*;
use mpcrust::mpc::*;
use mpcrust::radio::*;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;

extern crate xdg;


fn main() {

    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("/tmp/mpcrust.log").unwrap()),
        ]
    ).unwrap();

    debug!("Launch mpcrust ==============");

    let mut mpc = Mpc::new("127.0.0.1", "6600");
    let mut radios = RadioList::new();

    let stdout = stdout();
    // mpc.clear();
    mpc.random(false);
    mpc.repeat(false);
    mpc.single(false);
    mpc.consume(false);

    // try to read config of keys from xdg keys.json file in data dir
    let xdg_dirs = xdg::BaseDirectories::with_prefix("mpcrust").unwrap();
    let touchs: [u8; 20] = match xdg_dirs.find_data_file("keys.json") {
        Some(keyspath) => {
            // if File::open(keyspath).is_err() {
            //     panic!("Unable to read {} file.", keyspath.to_str()
            // }
            let contents = match fs::read_to_string(keyspath) {
                Ok(contents) => contents,
                Err(_) => {
                    panic!("Unable to read keys.json file.");
                }
            };
            match serde_json::from_str(&contents) {
                Ok(touchs) => touchs,
                Err(_) => {
                    panic!("Unable to read keys.json file, bad format.");
                }
            }
        },
        None => [65, 66, 68, 67, 38, 169, 34, 39, 40, 45, 168, 95, 97, 160, 44, 110, 114, 103, 121, 98]
    };

    let mut wind = Window::new(&stdout, &mut mpc, &mut radios);
    let mut currentsong = String::from("");

    wind.clean();
    wind.draw();
    wind.init();

    let stdin_channel = spawn_stdin_channel();
    let touchst = TouchTranslator{touchs};

    loop {
        match stdin_channel.try_recv() {
            // Ok(b'q') => {
            //     wind.stop();
            //     return;
            // },
            Ok(value8) => {
                let touch = touchst.getValue(value8);
                wind.touch(touch);
                wind.draw();
            }
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        if (wind.refreshable()) {
            wind.draw();
        }
        sleep(100);
    }
}

fn spawn_stdin_channel() -> Receiver<u8> {
    let (tx, rx) = mpsc::channel::<u8>();
    thread::spawn(move || loop {
        let stdin = stdin();
        let stdin = stdin.lock();
        let mut bytes = stdin.bytes();
        let b = bytes.next().unwrap().unwrap();
        tx.send(b).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
