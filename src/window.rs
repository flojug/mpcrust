
extern crate termion;
use termion::color;
use crate::mpcscreen::*;
use crate::widgets::*;
use crate::mpc::*;
use crate::radio::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Touch {
    TouchUp,
    TouchDown,
    TouchLeft,
    TouchRight,
    Touch1,
    Touch2,
    Touch3,
    Touch4,
    Touch5,
    Touch6,
    Touch7,
    Touch8,
    Touch9,
    Touch0,
    TouchPlay,
    TouchOk,
    TouchRed,
    TouchGreen,
    TouchYellow,
    TouchBlue,
    TouchNone
}

#[derive(Debug)]
pub struct TouchTranslator {
  pub touchs: [u8; 20]
}

impl TouchTranslator {
  pub fn get_value(&self, key: u8) -> Touch {
    if key == self.touchs[0] { return Touch::TouchUp; }
    else if key == self.touchs[1] { return Touch::TouchDown; }
    else if key == self.touchs[2] { return Touch::TouchLeft; }
    else if key == self.touchs[3] { return Touch::TouchRight; }
    else if key == self.touchs[4] { return Touch::Touch1; }
    else if key == self.touchs[5] { return Touch::Touch2; }
    else if key == self.touchs[6] { return Touch::Touch3; }
    else if key == self.touchs[7] { return Touch::Touch4; }
    else if key == self.touchs[8] { return Touch::Touch5; }
    else if key == self.touchs[9] { return Touch::Touch6; }
    else if key == self.touchs[10] { return Touch::Touch7; }
    else if key == self.touchs[11] { return Touch::Touch8; }
    else if key == self.touchs[12] { return Touch::Touch9; }
    else if key == self.touchs[13] { return Touch::Touch0; }
    else if key == self.touchs[14] { return Touch::TouchPlay; }
    else if key == self.touchs[15] { return Touch::TouchOk; }
    else if key == self.touchs[16] { return Touch::TouchRed; }
    else if key == self.touchs[17] { return Touch::TouchGreen; }
    else if key == self.touchs[18] { return Touch::TouchYellow; }
    else if key == self.touchs[19] { return Touch::TouchBlue; }
    Touch::TouchNone
  }
}


fn get_buttons_1() -> Vec<Button> {
    let mut v = Vec::new();
    v.push(Button::new("1-QUEUE", ItemState::Selected, Touch::Touch1, color::Rgb(255,0,0), Some(Action::SwitchWindow(1))));
    v.push(Button::new("2-SEARCH", ItemState::NotSelected, Touch::Touch2, color::Rgb(255,0,0), Some(Action::SwitchWindow(2))));
    v.push(Button::new("3-RADIO", ItemState::NotSelected, Touch::Touch3, color::Rgb(255,0,0), Some(Action::SwitchWindow(3))));
    v.push(Button::new("4-INF", ItemState::NotSelected, Touch::Touch4, color::Rgb(255,0,0), Some(Action::SwitchWindow(4))));
    v.push(Button::new("5-LISTS", ItemState::NotSelected, Touch::Touch5, color::Rgb(255,0,0), Some(Action::SwitchWindow(5))));
    v.push(Button::new("6-PARAMS", ItemState::NotSelected, Touch::Touch6, color::Rgb(255,0,0), Some(Action::SwitchWindow(6))));
    v
}

// fn get_buttons_2() -> Vec<Button> {
//     let mut v = Vec::new();
//     v.push(Button::new("1-QUEUE", ItemState::Selected, Touch::Touch1, color::Rgb(0,255,0), None));
//     v.push(Button::new("2-SEARCH", ItemState::NotSelected, Touch::Touch2, color::Rgb(0,255,0), None));
//     v.push(Button::new("3-RADIO", ItemState::NotSelected, Touch::Touch3, color::Rgb(0,255,0), None));
//     v.push(Button::new("4-INF", ItemState::NotSelected, Touch::Touch4, color::Rgb(0,255,0), None));
//     v.push(Button::new("5-LISTS", ItemState::NotSelected, Touch::Touch5, color::Rgb(0,255,0), None));
//     v.push(Button::new("6-PARAMS", ItemState::NotSelected, Touch::Touch6, color::Rgb(0,255,0), None));
//     v
// }

fn get_buttons_3() -> Vec<Button> {
    let mut v = Vec::new();
    v.push(Button::new("1-PLAY", ItemState::Selected, Touch::Touch1, color::Rgb(255,255,0), Some(Action::Play)));
    v.push(Button::new("2-STOP", ItemState::NotSelected, Touch::Touch2, color::Rgb(255,255,0), Some(Action::Stop)));
    v.push(Button::new("3-PAUSE", ItemState::NotSelected, Touch::Touch3, color::Rgb(255,255,0), Some(Action::Pause)));
    v.push(Button::new("4-NEXT", ItemState::NotSelected, Touch::Touch4, color::Rgb(255,255,0), None));
    v.push(Button::new("5-PREV", ItemState::NotSelected, Touch::Touch5, color::Rgb(255,255,0), None));
    v
}

fn get_buttons_4() -> Vec<Button> {
    let mut v = Vec::new();
    v.push(Button::new("1-SEARCH", ItemState::Selected, Touch::Touch1, color::Rgb(255,255,0), None));
    v
}


fn get_keyboard() -> Keyboard {
    let mut l1 = Vec::new();
    l1.push(Button::new("A", ItemState::Selected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("Z", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("E", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("R", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("T", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("Y", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("U", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("I", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("O", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l1.push(Button::new("P", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    let mut l2 = Vec::new();
    l2.push(Button::new("Q", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("S", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("D", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("F", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("G", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("H", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("J", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("K", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("L", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l2.push(Button::new("M", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    let mut l3 = Vec::new();
    l3.push(Button::new("W", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("X", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("C", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("V", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("B", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("N", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("_", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("<", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("*", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    l3.push(Button::new("CR", ItemState::NotSelected, Touch::TouchNone, color::Rgb(255,255,0), None));
    Keyboard::new(l1, l2, l3)
}


fn get_red_menu() -> ButtonPannelOneLine { ButtonPannelOneLine::new(get_buttons_1()) }

//fn get_green_page_1() -> ButtonPannelOneLine { ButtonPannelOneLine::new(get_buttons_2()) }

fn get_yellow_page2() -> ButtonPannelOneLine { ButtonPannelOneLine::new(get_buttons_4()) }
fn get_yellow_page1() -> ButtonPannelOneLine { ButtonPannelOneLine::new(get_buttons_3()) }


pub enum SubWindow {
  Red,
  Green,
  Yellow
}


pub struct Window<'a>{
  panels: Vec<Box<dyn Widget>>,
  screen: MpcScreen<'a>,
  red: usize,
  green: usize,
  yellow: usize,
  current_color: SubWindow,
  mpc: &'a mut Mpc,
  idx_current_song: usize,
  current_song: String,
  radios: &'a mut RadioList,
}

impl<'a> Window<'a> {

  pub fn new(out: &'a std::io::Stdout, mpc: &'a mut Mpc, radios: &'a mut RadioList) -> Window<'a> {
    let mut screen = MpcScreen::new(out);
    screen.clean();

    let mut panels: Vec<Box<dyn Widget>> = Vec::new();
    panels.push(Box::new(get_red_menu()));
    panels.push(Box::new(ListItemPannel::new(mpc.get_songs(), Some(Action::PlaySong(0)), None, None, Some(Action::PlaySong(0)))));
    panels.push(Box::new(get_yellow_page1()));
    panels.push(Box::new(get_yellow_page2()));
    panels.push(Box::new(get_keyboard()));
    panels.push(Box::new(ListItemPannel::new(mpc.navigate(), Some(Action::UpSearch(0)), Some(Action::DownSearch(0)), None, Some(Action::SelSearch(0, false)))));
    // radios
    panels.push(Box::new(ListItemPannel::new(radios.get_list(), None, None, None, Some(Action::SelRadio(0)))));
    panels.push(Box::new(get_keyboard()));

    //let red_controler = Controler::new(
    Window {panels, screen, red: 0, green: 1, yellow: 2, current_color: SubWindow::Red, mpc, idx_current_song:0, current_song: String::from(""), radios }
  }

  pub fn stop(&mut self) {
    self.mpc.stop();
  }

  fn switch_window(&mut self, which: u16) {
    match which {
      // active queue
      1 => { self.green = 1; self.yellow = 2; },
      // search
      2 => { self.green = 5; self.yellow = 4; },
      // search radio
      3 => { self.green = 6; self.yellow = 7; },
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
        if title != self.current_song {
            debug!("{}", title);
            self.panels[1].set_current(&title);
            self.current_song = title;
            refr = true;
        }
    }

    refr
  }

  pub fn touch(&mut self, touch: Touch) {
    // debug
    //self.screen.line(1, 15, &format!("{:04}", touch), color::Rgb(255,255,255));
    match  touch {
      Touch::TouchRed => { self.current_color = SubWindow::Red; return; },
      Touch::TouchGreen => { self.current_color = SubWindow::Green; return; },
      Touch::TouchYellow => { self.current_color = SubWindow::Yellow; return; },
      _ => {
        let action = match self.current_color {
          SubWindow::Red => self.panels[self.red].touch(touch),
          SubWindow::Green => self.panels[self.green].touch(touch),
          SubWindow::Yellow => self.panels[self.yellow].touch(touch)
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
      SubWindow::Yellow => self.yellow = which_panel,
    }
    self.draw();
  }

  pub fn clean(&mut self) {
    self.screen.clean();
  }

  pub fn draw(&mut self) {

    let mut scbox = ScreenBox::new(1, 1, 50, 1);
    // self.screen.line(1, 1, &format!("{:1$}", " ", 50 as usize)[..], color::Rgb(0,0,0));
    self.panels[self.red].draw(&mut self.screen, scbox);

    scbox = ScreenBox::new(1, 3, 50, 10);
    // for i in 3..14 {
    //   self.screen.line(1, i, &format!("{:1$}", " ", 50 as usize)[..], color::Rgb(0,0,0));
    // }
    self.panels[self.green].draw(&mut self.screen, scbox);

    scbox = ScreenBox::new(1, 13, 50, 3);
    // for i in 13..16 {
    //   self.screen.line(1, i, &format!("{:1$}", " ", 50 as usize)[..], color::Rgb(0,0,0));
    // }
    self.panels[self.yellow].draw(&mut self.screen, scbox);

    self.flush();
  }

  pub fn flush(&mut self) {
    self.screen.flush();
  }

}



