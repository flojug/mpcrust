extern crate mpd;

use mpd::Client;
use mpd::Query;

use indextree::Arena;


#[derive(Debug)]
pub struct SongNode {
    name: String,
    song: Option<mpd::Song>
}

#[derive(Debug)]
pub struct Mpc {
    host: String,
    port: String,
    pub conn: mpd::client::Client,
    list: indextree::Arena<SongNode>,
    root_id: indextree::NodeId,
    root_search: indextree::NodeId,
    selected: Vec<usize>,
    current_level: usize,
}


// impl mpd::convert::ToQueuePlace for usize {
//     fn to_place(self) -> u32 {
//         self as u32
//     }
// }

impl Mpc {
    pub fn new(host: &str, port: &str) -> Mpc {
        let connstr = format!("{}:{}", host, port);
        let conn = Client::connect(connstr).unwrap();

        let li = Mpc::init_list();
        let shost = String::from(host);
        let sport = String::from(port);
        Mpc{ host: shost, port: sport, conn, list:li.0, root_id:li.1, root_search: li.2, selected:li.3, current_level: li.4 }
    }

    fn test_connect(&mut self) {
        if self.conn.status().is_err() {
            let connstr = format!("{}:{}", &self.host, &self.port);
            self.conn = Client::connect(connstr).unwrap();
        }
    }

    fn init_list() -> (indextree::Arena<SongNode>, indextree::NodeId, indextree::NodeId, Vec<usize>, usize) {
        let mut list = Arena::new();
        let root = SongNode {name: String::from("root"), song: None };
        let root_id = list.new_node(root);
        let root_search = root_id.clone();
        let selected = vec!();
        let current_level = 0;
        (list, root_id, root_search, selected, current_level)
    }

    // add a repository in the search tree
    fn add_song_rep(&mut self, root_id: indextree::NodeId, hierarchy: &Vec<&str>, index: usize) -> indextree::NodeId {
        if index >= hierarchy.len() {
            return root_id;
        }
        let next_rep = &hierarchy[index];
        let iter = root_id.children(&self.list);
        for node_id in iter {
            let node = &self.list[node_id];
            if node.get().name == *next_rep {
                return self.add_song_rep(node_id, hierarchy, index+1);
            }
        }
        // new rep
        let sn = SongNode{ name: next_rep.to_string().clone(), song: None };
        let sn_id = self.list.new_node(sn);
        root_id.append(sn_id, &mut self.list);
        return self.add_song_rep(sn_id, hierarchy, index+1);
    }

    // add song to the search tree
    fn add_song(&mut self, reps: &Vec<&str>, song: mpd::Song) {
        let root_id = self.root_id.clone();
        let place = self.add_song_rep(root_id, reps, 0);
        let name = song.title.clone().unwrap_or(song.file.clone());
        let sn = SongNode{ name: name, song: Some(song) };
        let sn_id = self.list.new_node(sn);
        place.append(sn_id, &mut self.list);
    }

    // get songs from queue
    pub fn get_songs(&mut self) -> Vec<String> {
        self.test_connect();
        let songs = if ! self.conn.queue().is_err() {self.conn.queue().unwrap()} else {vec!()};
        debug!("{:?}", songs);
        songs.iter().map(|s| {
            let title = s.name.clone().unwrap_or(s.title.clone().unwrap_or(s.file.clone()));
            //let title = if s.title.is_some() {s.title.clone().unwrap()} else {s.file.clone()};
            let mut _artist = String::from("");
            if s.tags.contains_key("Artist") {
                _artist = s.tags["Artist"].clone();
            }
            format!("{}", title)
        }).collect()
    }

    pub fn get_current_song(&mut self) -> Option<usize> {
        self.test_connect();
        if let Ok(Some(song)) = self.conn.currentsong() {
            let songs = if ! self.conn.queue().is_err() {self.conn.queue().unwrap()} else {vec!()};
            return songs.iter().position(|song2| song == *song2);
        }
        None
    }

    pub fn current_song(&mut self) -> Option<mpd::Song> {
        let song = self.conn.currentsong();
        if !song.is_err() {
            return song.unwrap();
        } else {
            return None;
        }
    }

    pub fn stop(&mut self) {
        self.test_connect();
        self.conn.stop().unwrap();
    }

    pub fn play(&mut self) {
        self.test_connect();
        self.conn.play().unwrap();
    }

    pub fn clear(&mut self) {
        self.test_connect();
        self.conn.clear().unwrap();
    }

    pub fn random(&mut self, value: bool) {
        self.test_connect();
        self.conn.random(value).unwrap();
    }

    pub fn consume(&mut self, value: bool) {
        self.test_connect();
        self.conn.consume(value).unwrap();
    }

    pub fn repeat(&mut self, value: bool) {
        self.test_connect();
        self.conn.repeat(value).unwrap();
    }

    pub fn rescan(&mut self) {
        self.test_connect();
        self.conn.rescan().unwrap();
    }

    pub fn single(&mut self, value: bool) {
        self.test_connect();
        self.conn.single(value).unwrap();
    }

    pub fn down(&mut self, idx: usize) -> Vec<String> {
        let mut iter = self.root_search.children(&self.list);
        if let Some(node_id) = iter.nth(idx) {
            let node = &self.list[node_id];
            // test if leaf
            if node.get().song.is_none() {
                self.selected[self.current_level] = idx;
                self.current_level = self.current_level + 1;
                self.root_search = node_id;
            }
        }
        return self.navigate();
    }

    // select files in search tree
    pub fn select(&mut self, idx: usize) {
        self.test_connect();
        self.conn.clear().unwrap();
        let mut iter = self.root_search.children(&self.list);
        // find selected node
        if let Some(node_id) = iter.nth(idx) {
            let node = &self.list[node_id];
            // test if leaf : no
            if node.get().song.is_none() {
                let iter_2 = node_id.traverse(&self.list);
                for node_edge in iter_2 {
                    if let indextree::NodeEdge::Start(node_id) = node_edge {
                       let node_2 = &self.list[node_id];
                       if node_2.get().song.is_some() {
                            self.conn.push(node_2.get().song.clone().unwrap()).unwrap();
                        }
                    }
                } // leaf : yes
            } else {
                let s = node.get().song.clone().unwrap();
                //s.file = String::from("http://uk3.internet-radio.com:8060/;stream");
                //s.file = String::from("http://dir.xiph.org/listen/150203/listen.m3u");
                //s.file = String::from("http://178.33.232.106:8046/autodj");
                self.conn.push(s).unwrap();
            }
        }
    }

    pub fn select_radio(&mut self, station: String, url: String) {
        self.test_connect();
        self.conn.clear().unwrap();
        let mut song = mpd::Song::default();
        song.file = url;
        song.title = Some(station);
        self.conn.push(song).unwrap();
    }

    pub fn up(&mut self, idx: usize) -> Vec<String> {
        let mut iter = self.root_search.children(&self.list);
        if let Some(node_id) = iter.nth(idx) {
            let node = &self.list[node_id];
            if let Some(node_id_parent) = node.parent() {
                let node_parent = &self.list[node_id_parent];
                if let Some(node_id_parent2) = node_parent.parent() {
                    self.current_level = self.current_level - 1;
                    self.root_search = node_id_parent2;
                }
            }
        }
        return self.navigate();
    }

    pub fn get_idx_selected(&mut self) -> usize {
        return self.selected[self.current_level];
    }

    fn populate_list(&mut self, search: &str)
    {
        //let quer = quer.and(mpd::Term::Any, "Soul");
        // let quer = quer.and(mpd::Term::Any, "Soul");
        // let quer = quer.and(mpd::Term::Tag(Cow::from("Artist")), "");
//        let quer = quer.and(mpd::Term::File, "Brassens");
        // quer.and(mpd::search::Term::File, "Noir");

        self.test_connect();
        let mut quer = Query::new();
        let quer = quer.and(mpd::Term::File, search);
        let songs = self.conn.search(&quer, None).unwrap();
        let mut maxd = 0;
        for s in songs.iter() {
            let split = s.file.split("/");
            let mut reps = split.collect::<Vec<&str>>();
            maxd = std::cmp::max(maxd, reps.len());
            reps.pop();
            self.add_song(&reps, s.clone());
        }
        for _ in 0..maxd {
            self.selected.push(0);
        }
    }

    pub fn search(&mut self, search: &str) -> Vec<String> {
        let mut sest = String::from(search);
        sest = sest.replacen("_", " ", 10);
        let sest = sest.trim();
        let li = Mpc::init_list();
        self.list = li.0;
        self.root_id = li.1;
        self.root_search = li.2;
        self.selected = li.3;
        self.current_level = li.4;
        self.populate_list(sest);

        let mut ret = vec!();
        let iter = self.root_search.children(&self.list);
        for node_id in iter {
            let node = &self.list[node_id];
            ret.push(node.get().name.clone());
        }
        ret
    }

    pub fn navigate(&mut self) -> Vec<String> {
        // init search list
        if self.list.count() == 1 {
            self.populate_list("");
        }
        let mut ret = vec!();
        let iter = self.root_search.children(&self.list);
        for node_id in iter {
            let node = &self.list[node_id];
            ret.push(node.get().name.clone());
        }
        ret
    }

    // pub fn get_current_song(&mut self) -> Option<String> {
    //     if let song = self.conn.currentsong().unwrap() {
    //         return String::from(song.title);
    //     } else {
    //         return Nothing;
    //     }
    // }

    pub fn play_song(&mut self, which: u32) {
        self.test_connect();
        // self.conn.stop();
        self.conn.switch(which).unwrap();
        self.conn.play().unwrap();
    }
}

    // let mut conn = Client::connect("192.168.2.20:6600").unwrap();
    // // conn.load("My Lounge Playlist", ..).unwrap();
    // conn.play().unwrap();
    // println!("Status: {:?}", conn.status());

    // let songs = conn.queue().unwrap();
    // let titles: Vec<String> = songs.iter().map(|s| String::from(s.title.clone().unwrap())).collect();
    // //    let str_titles = titles.join("\n");
