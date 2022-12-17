

use radiobrowser::blocking::RadioBrowserAPI;


#[derive(Debug)]
pub struct RadioList {
    stations: Vec<radiobrowser::ApiStation>,
}

impl RadioList {
    pub fn new() -> RadioList {
        RadioList { stations: vec!() }
    }

    pub fn get_list(&mut self) -> Vec<String> {
        let names = self.stations.iter().map(|station|String::from(&station.name)).collect();
        names
    }

    pub fn get_station(&mut self, which: usize) -> String {
        let station = &self.stations[which];
        String::from(&station.name)
    }

    pub fn get_url(&mut self, which: usize) -> String {
        let station = &self.stations[which];
        String::from(&station.url_resolved)
    }

    pub fn search(&mut self, search: &str) -> Vec<String> {
        let api = RadioBrowserAPI::new().unwrap();
        self.stations = api
            .get_stations()
            .name(search)
            .send()
            .unwrap();
        let names = self.stations.iter().map(|station|String::from(&station.name)).collect();
        names
    }

}
