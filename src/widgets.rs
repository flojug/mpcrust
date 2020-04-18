
extern crate termion;

use crate::mpcscreen::*;
use termion::color;
use termion::raw::*;

use std::cmp;

#[derive(Clone)]
#[derive(Debug)]
pub struct ScreenBox {
    x: u16, y: u16, w: u16, h: u16
}

impl ScreenBox {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> ScreenBox { ScreenBox{x, y, w, h} }
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Action {
    SwitchWindow(u16),
    PlaySong(usize),
    SelSearch(usize, bool),
    UpSearch(usize),
    DownSearch(usize),
    Play,
    Stop,
    Pause,
    Search(String),
}


pub trait Widget {
    fn draw(&mut self, sc: &mut MpcScreen, scbox: ScreenBox);
    fn key(&mut self, key: u8) -> Option<Action>;
    fn refresh(&mut self, items: Vec<String>, idx: usize) {}
    fn set_current(&mut self, s: &String) {}
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ItemState {
    Selected,
    NotSelected
}


#[derive(Debug)]
pub struct Button {
    item: String,
    state: ItemState,
    key: u8,
    col: color::Rgb,
    action: Option<Action>
}

impl Button {
    pub fn new(item: &str, state: ItemState, key: u8, col: color::Rgb, action: Option<Action>) -> Button {
        Button { item: String::from(item), state, key, col, action }
    }

    pub fn get_len(&self) -> u16 {
        return self.item.len() as u16;
    }

    pub fn get_item(&self) -> String {
        self.item.clone()
    }
}

impl Widget for Button {
    fn draw(&mut self, sc: &mut MpcScreen, scbox: ScreenBox) {
      sc.line(scbox.x, scbox.y, &format!("{:1$}", " ", scbox.w as usize)[..], self.col.clone());
      if self.state == ItemState::Selected {
        sc.bgline(scbox.x, scbox.y, &self.item[..], self.col.clone());
      } else {
        sc.line(scbox.x, scbox.y, &self.item[..], color::Rgb(255,255,255));
      }
    }

    fn key(&mut self, key: u8) -> Option<Action> {
        None
    }
}

pub type Item = Button;

#[derive(Debug)]
pub struct ListItemPannel {
    items: Vec<Item>,
    first_line: u16,
    left_action: Option<Action>,
    right_action: Option<Action>,
    select_action: Option<Action>,
    apply_action: Option<Action>,
}

impl ListItemPannel {
    pub fn new(stritems: Vec<String>, left_action: Option<Action>, right_action: Option<Action>, select_action: Option<Action>, apply_action: Option<Action>) -> ListItemPannel {
        let items = ListItemPannel::init_items(stritems, 0);
        ListItemPannel {items, first_line: 0, left_action, right_action, select_action, apply_action }
    }

    fn init_items(stritems: Vec<String>, idx: usize) -> Vec<Item> {
        let mut items = vec!();
        for stritem in stritems {
            let item = Item {item: stritem, state: ItemState::NotSelected, key: b'p', col: color::Rgb(0,255,0), action: None };
            items.push(item);
        }
        if items.len() > idx {
            items[idx].state = ItemState::Selected;
        }
        items
    }
}

impl Widget for ListItemPannel {

    fn draw(&mut self, sc: &mut MpcScreen, scbox: ScreenBox) {

        if let Some(idx_over) = self.items.iter().position(|item| (*item).state == ItemState::Selected) {
            let idx = idx_over as u16;
            //sc.line(20, 15, &format!("{} {} {}", idx, self.first_line, scbox.h)[..], color::Rgb(255,255,255));
            if idx < self.first_line {
                self.first_line = idx;
            } else if idx >= self.first_line + scbox.h {
//                self.first_line = self.first_line + idx - scbox.h + 1;
                self.first_line = self.first_line + 1;
//                sc.line(20, 15, &format!("ICI    {}", self.first_line)[..], color::Rgb(255,255,255));
            }
        }

        //let min = scbox.h as usize; //cmp::min(self.items.len(), scbox.h as usize);
        for idxrow in 0..scbox.h {
            let mut scbox2 = scbox.clone();
            scbox2.y = scbox2.y + idxrow as u16;
            let idx = (self.first_line + idxrow) as usize;
            if idx < self.items.len() {
                self.items[idx].draw(sc, scbox2);
            }
        }
    }

    // set current item with given value
    fn set_current(&mut self, s: &String) {
        debug!("set_current {}", *s);
        debug!("{:?}", self.items);
        if let Some(idxcur) = self.items.iter().position(|item| (*item).item == *s) {
            for idx in 0..self.items.len() {
                self.items[idx].state = ItemState::NotSelected;
            }
            debug!("{}", idxcur);
            self.items[idxcur].state = ItemState::Selected;
        }
    }

    fn key(&mut self, key: u8) -> Option<Action> {
        let mut ret = None;

        // find index which button is over
        // per default it is the selected
        let mut idx: usize = 0;
        let mut idx_sel: usize = 0;
        idx = 0;
        for button in &self.items {
            if (*button).state == ItemState::Selected {
                break;
            }
            idx = idx + 1;
            idx_sel = idx;
        }

        if (key == b'x') && (idx < self.items.len()-1) {  // down
            self.items[idx].state = ItemState::NotSelected;
            self.items[idx+1].state = ItemState::Selected;
            return None;
        }

        if (key == b'e') && (idx > 0) {  // up
            self.items[idx].state = ItemState::NotSelected;
            self.items[idx-1].state = ItemState::Selected;
            return None;
        }

        if (key == b's') {  // left
            if let Some(idx) = self.items.iter().position(|item| (*item).state == ItemState::Selected) {
                match self.left_action {
                    Some(Action::UpSearch(_))=> return Some(Action::UpSearch(idx)),
                    _=> return None
                }
            } else {
                return None;
            }
        }

        if (key == b'd') {  // right
            if let Some(idx) = self.items.iter().position(|item| (*item).state == ItemState::Selected) {
                match self.right_action {
                    Some(Action::DownSearch(_))=> return Some(Action::DownSearch(idx)),
                    _=> return None
                }
            } else {
                return None;
            }
        }

        if key == b'p' {  // select
            if let Some(idx) = self.items.iter().position(|item| (*item).state == ItemState::Selected) {
                match self.select_action {
                    _=> return None
                }
            } else {
                return None;
            }
        }

        if key == b'a' {
            if let Some(idx) = self.items.iter().position(|item| (*item).state == ItemState::Selected) {
                match self.apply_action {
                    Some(Action::PlaySong(_))=> return Some(Action::PlaySong(idx)),
                    Some(Action::SelSearch(_, _)) => return Some(Action::SelSearch(idx, true)),
                    _=> return None
                };
            }
        }

        ret
    }

    fn refresh(&mut self, stritems: Vec<String>, idx: usize) {
        let items = ListItemPannel::init_items(stritems, idx);
        self.first_line = idx as u16;
        if self.first_line > 2 {
            self.first_line = self.first_line - 2;
        }
        self.items = items;
    }
}

pub type Key = Button;

#[derive(Debug)]
pub struct Keyboard {
    first_line: Vec<Key>,
    second_line: Vec<Key>,
    third_line: Vec<Key>,
    search: [char; 10],
    idx_search: usize
}

impl Keyboard {
    pub fn new(first_line: Vec<Key>, second_line: Vec<Key>, third_line: Vec<Key>) -> Keyboard {
        let search = [ '_'; 10];
        Keyboard {first_line, second_line, third_line, search, idx_search: 0 }
    }

    fn get_selected(&self) -> Option<String> {
        if let Some(idx) = self.first_line.iter().position(|item| (*item).state == ItemState::Selected) {
            Some(self.first_line[idx].get_item())
        } else if let Some(idx) = self.second_line.iter().position(|item| (*item).state == ItemState::Selected) {
            Some(self.second_line[idx].get_item())
        } else if let Some(idx) = self.third_line.iter().position(|item| (*item).state == ItemState::Selected) {
            Some(self.third_line[idx].get_item())
        } else {
            return None;
        }
    }
}

impl Widget for Keyboard {
    fn draw(&mut self, sc: &mut MpcScreen, scbox: ScreenBox) {

        let mut draw_line = |line: &mut Vec<Key>, idx_line: u16| {
            sc.line(scbox.x, scbox.y+idx_line, &format!("{:1$}", " ", scbox.w as usize)[..], color::Rgb(0,0,0));
            let nb_butt: u16 = (*line).len() as u16;
            let rest: u16 = (scbox.w - 2*nb_butt) / 2;
            let mut idx: u16 = rest - 10;
            for key in line.iter_mut() {
                let mut scbox2 = scbox.clone();
                scbox2.w = 1;
                scbox2.x = scbox2.x + idx;
                scbox2.y = scbox2.y + idx_line;
                key.draw(sc, scbox2);
                idx = idx + 2;
            }
        };

        draw_line(&mut self.first_line, 0);
        draw_line(&mut self.second_line, 1);
        draw_line(&mut self.third_line, 2);

        for i in 0..self.search.len() {
            let idx = (i as u16);// * 2;
            sc.line(scbox.x+30+idx, scbox.y+1, &self.search[i].to_string(), color::Rgb(255,255,255));
        }
    }

    fn key(&mut self, key: u8) -> Option<Action> {
        let mut ret = None;

        // find postition in keyboard
        let mut line;
        let mut next_line;
        let mut prev_line;
        let mut index = 0;

        if let Some(idx) = self.first_line.iter().position(|item| (*item).state == ItemState::Selected) {
            index = idx;
            line = &mut self.first_line;
            next_line = Some(&mut self.second_line);
            prev_line = None;
        } else if let Some(idx) = self.second_line.iter().position(|item| (*item).state == ItemState::Selected) {
            index = idx;
            line = &mut self.second_line;
            next_line = Some(&mut self.third_line);
            prev_line = Some(&mut self.first_line);
        } else if let Some(idx) = self.third_line.iter().position(|item| (*item).state == ItemState::Selected) {
            index = idx;
            line = &mut self.third_line;
            next_line = None;
            prev_line = Some(&mut self.second_line);
        } else {
            return None;
        }

        if (key == b's') && (index > 0) {  // left
            line[index].state = ItemState::NotSelected;
            line[index-1].state = ItemState::Selected;
        }
        if (key == b'd') && (index+1 < line.len()) {  // right
            line[index].state = ItemState::NotSelected;
            line[index+1].state = ItemState::Selected;
        }

        if (key == b'x') && (!next_line.is_none()) {  // down
            line[index].state = ItemState::NotSelected;
            next_line.unwrap()[index].state = ItemState::Selected;
        }

        if (key == b'e') && (!prev_line.is_none()) {  // up
            line[index].state = ItemState::NotSelected;
            prev_line.unwrap()[index].state = ItemState::Selected;
        }

        if (key == b'a') || (key == b'p') {  // apply
            if let Some(key) = self.get_selected() {
                let car = (&key[..]).chars().next().unwrap();
                if key == String::from("CR") {
                    let mut s = String::from("");
                    for i in 0..self.search.len() {
                        s.push(self.search[i]);
                    }
                    return Some(Action::Search(s));
                }
                else if car == '*' {
                    for i in 0..self.search.len() {
                        self.search[i] = '_';
                    }
                    self.idx_search = 0;
                } else if car == '<' {
                    if self.idx_search > 0 {
                        self.idx_search = self.idx_search - 1;
                        self.search[self.idx_search] = '_';
                    }
                }
                else if self.idx_search < self.search.len() {
                    self.search[self.idx_search] = car;
                    self.idx_search = self.idx_search + 1;
                }
            }
        }

        ret
    }
}

#[derive(Debug)]
pub struct ButtonPannelOneLine {
    buttons: Vec<Button>,
}

impl ButtonPannelOneLine {
    pub fn new(buttons: Vec<Button>) -> ButtonPannelOneLine {
        ButtonPannelOneLine {buttons}
    }
}

impl Widget for ButtonPannelOneLine {
    fn draw(&mut self, sc: &mut MpcScreen, scbox: ScreenBox) {

        let len_scbox = self.buttons.iter_mut().fold(0, |acc, button| acc + (*button).get_len());
        let nb_butt: u16 = self.buttons.len() as u16;
        let len_space: u16 = (scbox.w - len_scbox) / (nb_butt + 1);
        let rest: u16 = (len_scbox % (nb_butt + 1)) / 2;

        let mut idx: u16 = rest + len_space;
        for button in &mut self.buttons {
            let mut scbox2 = scbox.clone();
            scbox2.x = scbox2.x + idx;
            (*button).draw(sc, scbox2);
            idx = idx + len_space + (*button).get_len();
        }
    }

    fn key(&mut self, key: u8) -> Option<Action> {

        let mut ret = None;

        // find index which button is over
        // per default it is the selected
        let mut idx: usize = 0;
        idx = 0;
        for button in &self.buttons {
            if (*button).state == ItemState::Selected {
                break;
            }
            idx = idx + 1;
        }

        if (key == b's') && (idx > 0) {  // left
            self.buttons[idx].state = ItemState::NotSelected;
            self.buttons[idx-1].state = ItemState::Selected;
            return None;
        }
        if (key == b'd') && (idx < self.buttons.len()-1) {  // right
            self.buttons[idx].state = ItemState::NotSelected;
            self.buttons[idx+1].state = ItemState::Selected;
            return None;
        }

        if key == b'p' {
            if let Some(idx) = self.buttons.iter().position(|item| (*item).state == ItemState::Selected) {
                match self.buttons[idx].action {
                    Some(Action::SwitchWindow(which))=> return Some(Action::SwitchWindow(which)),
                    Some(Action::Play) => return Some(Action::Play),
                    Some(Action::Stop) => return Some(Action::Stop),
                    Some(Action::Pause) => return Some(Action::Pause),
                    _=> return None
                }
                return self.buttons[idx].action;
            } else {
                return None;
            }
        }

        // test if button known
        let test: Vec<&mut Button> = self.buttons.iter_mut()
            .filter(|button| (*button).key == key).collect();

        if test.len() == 0 {
            return None;
        }

        self.buttons.iter_mut()
            .filter(|button| (*button).state == ItemState::Selected)
            .for_each(|button| (*button).state = ItemState::NotSelected);

        self.buttons.iter_mut()
            .filter(|button| (*button).key == key)
            .for_each(|button| (*button).state = ItemState::Selected);


        if let Some(butt) = self.buttons.iter_mut().find(|button|(*button).state == ItemState::Selected ) {
            ret = butt.action.clone();
        }
        ret
    }

}
