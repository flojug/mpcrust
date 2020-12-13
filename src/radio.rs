use std::net::TcpStream;
use std::borrow::Cow;
use std::cmp;

use indextree::Arena;

extern crate rusqlite;
use rusqlite::{params, named_params, Result};
use rusqlite::NO_PARAMS;

extern crate xdg;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

use serde_xml_rs;

#[derive(Debug)]
pub struct RadioList {
    current: usize,
    ids: Vec<u32>,
    conn: rusqlite::Connection,
}

#[derive(Deserialize, Debug)]
pub struct Directory {
    #[serde(rename="entry")]
    radios: Vec<Radio>,
}

#[derive(Deserialize, Debug)]
pub struct Radio {
    #[serde(rename="server_name")]
    station: String,
    listen_url: String,
    server_type: String,
    bitrate: String,
    channels: String,
    samplerate: String,
    genre: String,
    current_song: String,
}

impl Radio {
    pub fn new(station: String, listen_url: String) -> Radio  {
        Radio {station: station, listen_url:listen_url, server_type: String::from(""), bitrate:String::from(""), channels: String::from(""), samplerate: String::from(""), genre: String::from(""), current_song: String::from("") }
    }
}

impl RadioList {
    pub fn new() -> RadioList {

        let mut pop = false;

        let xdg_dirs = xdg::BaseDirectories::with_prefix("mpcrust").unwrap();
        let mut db = xdg_dirs.find_data_file("radios.sqlite");

        let radios_db = xdg_dirs.place_data_file("radios.sqlite").expect("Cannot create configuration directory");
        let conn = rusqlite::Connection::open(radios_db).unwrap();

        if db.is_none() {
            debug!("{:?}", "_create_database");
            RadioList::_create_database(&conn);
            pop = true;
        }

        // ici tester si le fichier yp.xml existe et si non
        // le créer en le récupérant depuis internet
        let mut xmlf = xdg_dirs.find_data_file("yp.xml");
        if xmlf.is_none() {
            let mut resp = reqwest::blocking::get("http://dir.xiph.org/yp.xml").unwrap();
            //let text = resp.text().unwrap();
            let radios_xml = xdg_dirs.place_data_file("yp.xml").unwrap();
            let mut f = File::create(radios_xml).unwrap();
            resp.copy_to(&mut f);
            pop = true;
        }

        // populate or refresh database
        if pop {
            RadioList::_popuplate(&xdg_dirs, &conn);
        }

        //debug!("{:?}", radios_file.unwrap().to_str());

        // let radio  = Radio::new(String::from("Test"), String::from("http://mscp2.live-streams.nl:8100/flac.flac"));
        // let mut radios = vec!();
        // radios.push(radio);

        RadioList { current: 0, conn, ids: vec!() }
    }

    pub fn _create_database(conn: &rusqlite::Connection) {
        conn.execute(
            "create table if not exists stations (
                 id integer primary key,
                 server_name text not null,
                 listen_url text not null,
                 server_type text null,
                 bitrate text null,
                 channels text null,
                 samplerate text null,
                 genre text null,
                 current_song text null
             )",
            NO_PARAMS,
        ).unwrap();
    }

    // test if database has to be popupalted from URL
    // http://dir.xiph.org/yp.xml
    // just once upon a week not to stress the server
    pub fn _popuplate(xdg_dirs: &xdg::BaseDirectories, conn: &rusqlite::Connection) {

        let mut radios_xml = xdg_dirs.find_data_file("yp.xml").expect("Fichier xml non trouvé.");
        let contents = fs::read_to_string(radios_xml.to_str().unwrap()).expect("Can't open radio file.");

        let directory: Directory = serde_xml_rs::deserialize(contents.as_bytes()).unwrap();

        for radio in directory.radios.iter() {
            conn.execute(
                "INSERT INTO stations (server_name, listen_url, server_type, bitrate, channels, samplerate, genre, current_song) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![radio.station, radio.listen_url, radio.server_type, radio.bitrate, radio.channels, radio.samplerate, radio.genre, radio.current_song]
            );
        }
    }

    pub fn get_list(&mut self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT id,server_name FROM stations ").unwrap();
        let mut rows = stmt.query(NO_PARAMS).unwrap();
        let mut stations = Vec::new();
        self.ids = Vec::new();
        while let Some(station) = rows.next().unwrap() {
            stations.push(station.get(1).unwrap());
            self.ids.push(station.get(0).unwrap());
        }
        stations
    }

    pub fn get_station(&mut self, which: usize) -> String {
        let sid = self.ids[which];
        let mut stmt = self.conn.prepare("SELECT server_name FROM stations where id=:id").unwrap();
        let mut rows = stmt.query_named(named_params!{ ":id": sid }).unwrap();
        let mut station = String::from("");
        if let Some(row) = rows.next().unwrap() {
            station = row.get(0).unwrap();
        }
        station
    }

    pub fn get_url(&mut self, which: usize) -> String {
        let sid = self.ids[which];
        let mut stmt = self.conn.prepare("SELECT listen_url FROM stations where id=:id").unwrap();
        let mut rows = stmt.query_named(named_params!{ ":id": sid }).unwrap();
        let mut url = String::from("");
        if let Some(row) = rows.next().unwrap() {
            url = row.get(0).unwrap();
        }
        url
    }

    pub fn search(&mut self, search: &str) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT id,server_name FROM stations WHERE server_name LIKE :search OR genre LIKE :search OR current_song LIKE :search ").unwrap();
        let mut rows = stmt.query_named(named_params!{ ":search": format!("%{}%", search) }).unwrap();
        let mut stations = Vec::new();
        self.ids = Vec::new();
        while let Some(station) = rows.next().unwrap() {
            stations.push(station.get(1).unwrap());
            self.ids.push(station.get(0).unwrap());
        }
        stations
    }

}
