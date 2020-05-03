
extern crate termion;

use termion::color;
//use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};
use termion::raw::*;

use crate::mpcscreen::*;
use crate::widgets::*;

use crate::mpc::*;

use crate::radio::*;


fn getBUTTONS_1() -> Vec<Button> {
    let mut v = Vec::new();
    v.push(Button::new("1-QUEUE", ItemState::Selected, b'1', color::Rgb(255,0,0), Some(Action::SwitchWindow(1))));
    v.push(Button::new("2-SEARCH", ItemState::NotSelected, b'2', color::Rgb(255,0,0), Some(Action::SwitchWindow(2))));
    v.push(Button::new("3-RADIO", ItemState::NotSelected, b'3', color::Rgb(255,0,0), Some(Action::SwitchWindow(3))));
    v.push(Button::new("4-INF", ItemState::NotSelected, b'4', color::Rgb(255,0,0), Some(Action::SwitchWindow(4))));
    v.push(Button::new("5-LISTS", ItemState::NotSelected, b'5', color::Rgb(255,0,0), Some(Action::SwitchWindow(5))));
    v.push(Button::new("6-PARAMS", ItemState::NotSelected, b'6', color::Rgb(255,0,0), Some(Action::SwitchWindow(6))));
    v
}

fn getBUTTONS_2() -> Vec<Button> {
    let mut v = Vec::new();
    v.push(Button::new("1-QUEUE", ItemState::Selected, b'1', color::Rgb(0,255,0), None));
    v.push(Button::new("2-SEARCH", ItemState::NotSelected, b'2', color::Rgb(0,255,0), None));
    v.push(Button::new("3-RADIO", ItemState::NotSelected, b'3', color::Rgb(0,255,0), None));
    v.push(Button::new("4-INF", ItemState::NotSelected, b'4', color::Rgb(0,255,0), None));
    v.push(Button::new("5-LISTS", ItemState::NotSelected, b'5', color::Rgb(0,255,0), None));
    v.push(Button::new("6-PARAMS", ItemState::NotSelected, b'6', color::Rgb(0,255,0), None));
    v
}

fn getBUTTONS_3() -> Vec<Button> {
    let mut v = Vec::new();
    v.push(Button::new("1-PLAY", ItemState::Selected, b'1', color::Rgb(0,0,255), Some(Action::Play)));
    v.push(Button::new("2-STOP", ItemState::NotSelected, b'2', color::Rgb(0,0,255), Some(Action::Stop)));
    v.push(Button::new("3-PAUSE", ItemState::NotSelected, b'3', color::Rgb(0,0,255), Some(Action::Pause)));
    v.push(Button::new("4-NEXT", ItemState::NotSelected, b'4', color::Rgb(0,0,255), None));
    v.push(Button::new("5-PREV", ItemState::NotSelected, b'5', color::Rgb(0,0,255), None));
    v
}

fn getBUTTONS_4() -> Vec<Button> {
    let mut v = Vec::new();
    v.push(Button::new("1-SEARCH", ItemState::Selected, b'1', color::Rgb(0,0,255), None));
    v
}


fn getKeyBoard() -> Keyboard {
    let mut l1 = Vec::new();
    l1.push(Button::new("A", ItemState::Selected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("Z", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("E", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("R", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("T", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("Y", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("U", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("I", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("O", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l1.push(Button::new("P", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    let mut l2 = Vec::new();
    l2.push(Button::new("Q", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("S", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("D", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("F", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("G", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("H", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("J", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("K", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("L", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l2.push(Button::new("M", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    let mut l3 = Vec::new();
    l3.push(Button::new("W", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("X", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("C", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("V", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("B", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("N", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("_", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("<", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("*", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    l3.push(Button::new("CR", ItemState::NotSelected, b'*', color::Rgb(0,0,255), None));
    Keyboard::new(l1, l2, l3)
}


fn getRED_MENU() -> ButtonPannelOneLine { ButtonPannelOneLine::new(getBUTTONS_1()) }

fn getGREEN_PAGE_1() -> ButtonPannelOneLine { ButtonPannelOneLine::new(getBUTTONS_2()) }

fn getBLUE_PAGE2() -> ButtonPannelOneLine { ButtonPannelOneLine::new(getBUTTONS_4()) }
fn getBLUE_PAGE1() -> ButtonPannelOneLine { ButtonPannelOneLine::new(getBUTTONS_3()) }


pub enum SubWindow {
  Red,
  Green,
  Blue
}

pub struct Window<'a>{
  panels: Vec<Box<Widget>>,
  screen: MpcScreen<'a>,
  red: usize,
  green: usize,
  blue: usize,
  current_color: SubWindow,
  mpc: &'a mut Mpc,
  idx_current_song: usize,
  current_song: String,
  radios: &'a mut RadioList
}

impl<'a> Window<'a> {

  pub fn new(out: &'a std::io::Stdout, mpc: &'a mut Mpc, radios: &'a mut RadioList) -> Window<'a> {
    let mut screen = MpcScreen::new(out);
    screen.clean();

    let mut panels: Vec<Box<Widget>> = Vec::new();
    panels.push(Box::new(getRED_MENU()));
    panels.push(Box::new(ListItemPannel::new(mpc.get_songs(), Some(Action::PlaySong(0)), None, None, Some(Action::PlaySong(0)))));
    panels.push(Box::new(getBLUE_PAGE1()));
    panels.push(Box::new(getBLUE_PAGE2()));
    panels.push(Box::new(getKeyBoard()));
    panels.push(Box::new(ListItemPannel::new(mpc.navigate(), Some(Action::UpSearch(0)), Some(Action::DownSearch(0)), None, Some(Action::SelSearch(0, false)) )));
    // radios
    panels.push(Box::new(ListItemPannel::new(radios.get_list(), None, None, None, Some(Action::SelRadio(0)) )));
    panels.push(Box::new(getKeyBoard()));

    //let red_controler = Controler::new(
    Window {panels, screen, red: 0, green: 1, blue: 2, current_color: SubWindow::Red, mpc, idx_current_song:0, current_song: String::from(""), radios }
  }

  pub fn stop(&mut self) {
    self.mpc.stop();
  }

  fn switch_window(&mut self, which: u16) {
    match which {
      // active queue
      1 => { self.green = 1; self.blue = 2; },
      // search
      2 => { self.green = 5; self.blue = 4; },
      // search radio
      3 => { self.green = 6; self.blue = 7; },
      _ => {}
    }
  }

  // play first song
  pub fn init(&mut self) {
    self.apply(Some(Action::PlaySong(0)));
  }

  // return true if something has changed and window
  // must be refreshed
  pub fn refreshable(&mut self) -> bool {
    let mut refr = false;

    if let Some(song) = self.mpc.current_song() {
        let title = if song.title.is_some() {song.title.clone().unwrap()} else {song.file.clone()};
        if (title != self.current_song) {
            debug!("{}", title);
            self.panels[1].set_current(&title);
            self.current_song = title;
            refr = true;
        }
    }

    refr
  }

  pub fn key(&mut self, key: u8) {
    match  key {
      b'r' => { self.current_color = SubWindow::Red; return; },
      b'g' => { self.current_color = SubWindow::Green; return; },
      b'b' => { self.current_color = SubWindow::Blue; return; },
      _ => {
        let action = match self.current_color {
          SubWindow::Red => self.panels[self.red].key(key),
          SubWindow::Green => self.panels[self.green].key(key),
          SubWindow::Blue => self.panels[self.blue].key(key)
        };
        self.apply(action);
      }
    }
  }

  pub fn apply(&mut self, action: Option<Action>) {
    debug!("apply");
    debug!("{:?}", action);
    match action {
      Some(Action::SwitchWindow(which)) => { self.switch_window(which); },
      Some(Action::PlaySong(which)) => {
        self.mpc.stop();
        self.mpc.play_song(which as u32);
      },
      Some(Action::Stop) => {
        self.mpc.stop();
      },
      Some(Action::Play) => {
        self.mpc.play();
      },
      Some(Action::Pause) => {
      },
      Some(Action::DownSearch(which)) => {
        let newlist = self.mpc.down(which);
        self.panels[5].refresh(newlist, 0)
      },
      Some(Action::UpSearch(which)) => {
        let newlist = self.mpc.up(which);
        let idx = self.mpc.get_idx_selected();
        self.panels[5].refresh(newlist, idx)
      },
      Some(Action::SelSearch(which, sel)) => {
        if sel {
          self.mpc.select(which);
          let newlist = self.mpc.get_songs();
          self.panels[1].refresh(newlist, 0);
          self.apply(Some(Action::PlaySong(0)));
        }
      },
      Some(Action::SelRadio(which)) => {
        debug!("selradio");
        let station = self.radios.get_station(which);
        let newlist = vec!( station );
        self.panels[1].refresh(newlist, 0);
        let url = self.radios.get_url(which);
        let station = self.radios.get_station(which);
        self.mpc.select_radio(station, url);
        self.mpc.stop();
        self.mpc.play_song(0);
      },
      Some(Action::Search(search)) => {
        // search in repository
        if self.green == 5 {
          let newlist = self.mpc.search(&search);
          self.panels[5].refresh(newlist, 0);
        }
        // search in radios
        if self.green == 6 {
          let newlist = self.radios.search(&search);
          self.panels[6].refresh(newlist, 0);
        }
      }
      _ => {},
    }
  }

  pub fn change_panel(&mut self, color: SubWindow, which_panel: usize) {
    match color {
      SubWindow::Red => self.red = which_panel,
      SubWindow::Green => self.green = which_panel,
      SubWindow::Blue => self.blue = which_panel,
    }
    self.draw();
  }

  pub fn clean(&mut self) {
    self.screen.clean();
  }

  pub fn draw(&mut self) {

    let mut scbox = ScreenBox::new(1, 1, 50, 1);
    self.screen.line(1, 1, &format!("{:1$}", " ", 50 as usize)[..], color::Rgb(0,0,0));
    self.panels[self.red].draw(&mut self.screen, scbox);

    scbox = ScreenBox::new(1, 3, 50, 10);
    for i in 3..14 {
      self.screen.line(1, i, &format!("{:1$}", " ", 50 as usize)[..], color::Rgb(0,0,0));
    }
    self.panels[self.green].draw(&mut self.screen, scbox);

    scbox = ScreenBox::new(1, 13, 50, 3);
    for i in 13..16 {
      self.screen.line(1, i, &format!("{:1$}", " ", 50 as usize)[..], color::Rgb(0,0,0));
    }
    self.panels[self.blue].draw(&mut self.screen, scbox);

    self.flush();
  }

  pub fn flush(&mut self) {
    self.screen.flush();
  }

}



