
use std::fs;
use std::fs::File;

use std::io::{Read, stdout, stdin};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

use mpcrust::window::*;
use mpcrust::mpc::*;
use mpcrust::radio::*;

use std::collections::BTreeMap;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;

extern crate xdg;


fn main() {

    let xdg_dirs = xdg::BaseDirectories::with_prefix("mpcrust").unwrap();
    let str_config = match xdg_dirs.find_config_file("mpcrust.ini") {
        Some(path) => {
            match fs::read_to_string(path) {
                Ok(dummy) => dummy,
                Err(_) => {
                    panic!("Unable to read keys.json file.");
                }
            }
        },
        None => String::from("---\nserver: 127.0.0.1\nport: 6600\ndebug: true\ndebug_file: /tmp/mpcrust.log\n")
    };

    let config: BTreeMap<String, String> = serde_yaml::from_str(&str_config).unwrap();

    if config.get("debug").unwrap_or(&String::from("true")) == "true" {
        let def_log_file = String::from("/tmp/mpcrust.log");
        let log_file = config.get("debug_file").unwrap_or(&def_log_file);
        CombinedLogger::init(
            vec![
                WriteLogger::new(LevelFilter::Debug, Config::default(), File::create(log_file).unwrap()),
            ]
        ).unwrap();
    }

    debug!("Launch mpcrust ==============");
    debug!("config : {:?}", config);

    let mut mpc = Mpc::new(config.get("server").unwrap_or(&String::from("127.0.0.1")), config.get("port").unwrap_or(&String::from("6600")));
    let mut radios = RadioList::new();

    let stdout = stdout();
    // mpc.clear();
    mpc.random(false);
    mpc.repeat(false);
    mpc.single(false);
    mpc.consume(false);

    // try to read config of keys from xdg keys.json file in data dir.
    // Example :
    // $ cat ~/.local/share/mpcrust/keys.json
    // [66, 65, 64, 67, 38, 169, 34, 39, 40, 45, 168, 95, 97, 160, 44, 110, 114, 103, 121, 98]
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
        // default keys for my configuration
        None => [
            65, // 65, // TOUCH_UP,
            66, //66, // TOUCH_DOWN,
            68, //68, // TOUCH_LEFT,
            67, //67, // TOUCH_RIGHT,
            49, //38, // TOUCH_1,
            50, //169, // TOUCH_2,
            51, //34, // TOUCH_3,
            52, //39, // TOUCH_4,
            53, //40, // TOUCH_5,
            54, //45, // TOUCH_6,
            55, //168, // TOUCH_7,
            56, //95, // TOUCH_8,
            57, //97, // TOUCH_9,
            48, //160, // TOUCH_0,
            43, //44, // TOUCH_PLAY,
            13, //110, // TOUCH_OK,
            114, // TOUCH_RED,
            103, // TOUCH_GREEN,
            121, // TOUCH_YELLOW,
            98, // TOUCH_BLUE,
            ]
    };

    let mut wind = Window::new(&stdout, &mut mpc, &mut radios);
    //let mut currentsong = String::from("");

    wind.clean();
    wind.draw();
    wind.init();

    let stdin_channel = spawn_stdin_channel();
    let touchst = TouchTranslator{touchs};

    loop {
        match stdin_channel.try_recv() {
            Ok(b'q') => {
                wind.stop();
                return;
            },
            Ok(value8) => {
                debug!("{:?}", value8);
                let touch = touchst.get_value(value8);
                wind.touch(touch);
                wind.draw();
            }
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        if wind.refreshable() {
            wind.draw();
        }
        sleep(100);
    }
}

fn spawn_stdin_channel() -> Receiver<u8> {
    let (tx, rx) = mpsc::channel::<u8>();
    thread::spawn(move || {
        let stdin = stdin();
        let stdin = stdin.lock();
        let mut bytes = stdin.bytes();
        loop {
            let b = bytes.next().unwrap().unwrap();
            tx.send(b).unwrap();
        }
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
