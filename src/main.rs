
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
use std::fs::File;


fn main() {


    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("/tmp/mpcrust.log").unwrap()),
        ]
    ).unwrap();

    debug!("Lancement mpcrust ==============");

    let mut mpc = Mpc::new("127.0.0.1", "6600");
    let mut radios = RadioList::new();

    let stdout = stdout();
    // mpc.clear();
    mpc.random(false);
    mpc.repeat(false);
    mpc.single(false);
    mpc.consume(false);
    let mut wind = Window::new(&stdout, &mut mpc, &mut radios);
    let mut currentsong = String::from("");

    wind.clean();
    wind.draw();
    wind.init();

    let stdin_channel = spawn_stdin_channel();

    loop {
        match stdin_channel.try_recv() {
            Ok(b'q') => {
                wind.stop();
                return;
            },
            Ok(valu8) => {
                wind.key(valu8);
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
