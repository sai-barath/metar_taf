//TODO: cleanup- reduce repetitive calls, better on borrowing/refs, better error handling
//TODO: major refactor- do away with structs/enums, combine name_needed and constructors into one fn that returns what name_needed currently returns
pub use std::ops::Range;
pub struct When {
    day_of_month: i32,
    zulu_time: i32, 
}
impl When {
    pub fn new(info: String) -> Self {
        Self {
            day_of_month: info[..2].parse::<i32>().unwrap(),
            zulu_time: info[2..6].parse::<i32>().unwrap(),
        }
    }
    pub fn get_day(&self) -> i32 {
        self.day_of_month
    }
    pub fn get_time(&self) -> i32 {
        self.zulu_time
    }
}
pub struct Wind {
    pub spd: Range<u32>,
    pub dir: Range<u32>,
}
impl Wind {
    pub fn new(info: String) -> Self {
        let d = info[..3].parse::<u32>(); // err here means dir was "VRB"
        let sp = info.find(" "); // some here means variable dir was included
        let g = info.find("G"); // some here means there is gust
        let dir1: Range<u32>;
        let spd1: Range<u32>;
        if let Ok(direction) = d {
            if let Some(i_s) = sp {
                dir1 = (info[i_s + 1..info.find("V").unwrap()].parse::<u32>().expect("one")..info[info.find("V").unwrap() + 1..].parse::<u32>().expect("two"))
            } else {
                dir1 = (info[0..3].parse::<u32>().unwrap()..info[0..3].parse::<u32>().unwrap())
            }
        } else {
            dir1 = (999..999);
        }
        if let Some(gust) = g {
            spd1 = (info[3..gust].parse::<u32>().expect("1")..info[gust + 1..info.find("K").unwrap()].parse::<u32>().expect("2"));
        } else {
            spd1 = (info[3..info.find("K").unwrap()].parse::<u32>().unwrap()..info[3..info.find("K").unwrap()].parse::<u32>().unwrap());
        }
        Self {
            spd: spd1,
            dir: dir1,
        }
    }
}
//TODO: fractions
#[derive(Debug)]
pub enum Visibility {
    Plus(i32),
    Exact(i32), 
    Less(i32), 
}
impl Visibility {
    pub fn new(info: String) -> Self {
        if(&info[0..1] == "P") {
            Visibility::Plus(info[1..info.find("S").unwrap()].parse::<i32>().unwrap())
        } else if(&info[0..1] == "M") {
            Visibility::Less(info[1..info.find("S").unwrap()].parse::<i32>().unwrap())
        } else {
            Visibility::Exact(info[0..info.find("S").unwrap()].parse::<i32>().unwrap())
        }
    }
    pub fn visibility_for_rvr(info: String) -> Self {
        match &info[0..1] {
            "M" => Visibility::Less(info[info.find("M").unwrap() + 1..].parse::<i32>().unwrap()),
            "P" => Visibility::Plus(info[info.find("P").unwrap() + 1..].parse::<i32>().unwrap()),
            _ => Visibility::Exact(info[0..].parse::<i32>().unwrap()),
        }
    }
}
//TODO: vertical visib, clr/skc
pub enum Cloud_layer {
    Few(i32),
    Sct(i32),
    Bkn(i32),
    Ovc(i32),
    ClrSkc(i32),
}
impl Cloud_layer {
    pub fn new(info: String) -> Self {
        match &info[0..3] {
            "OVC" => Cloud_layer::Ovc(info[3..6].parse::<i32>().unwrap() * 100),
            "BKN" => Cloud_layer::Bkn(info[3..6].parse::<i32>().unwrap() * 100),
            "SCT" => Cloud_layer::Sct(info[3..6].parse::<i32>().unwrap() * 100),
            "FEW" => Cloud_layer::Few(info[3..6].parse::<i32>().unwrap() * 100),
            _ => Cloud_layer::ClrSkc(0),
        }
    }
} 
pub struct Alt {
   alt_inhg: f64,
}
impl Alt { 
    pub fn new(info: String) -> Self {
        Self {
            alt_inhg: (info[1..].parse::<f64>().unwrap()) / 100.0,
        }
    }
    pub fn get_alt_inhg(&self) -> f64 {
        self.alt_inhg
    }
}
pub struct Temps {
    temp_celsius: i32,
    dewpoint_celsius: i32
}
impl Temps {
    pub fn new(info: String) -> Self {
        match info.len() {
            5 => {
                Self {
                    temp_celsius: info[..2].parse::<i32>().unwrap(),
                    dewpoint_celsius: info[3..].parse::<i32>().unwrap(),
                }
            },
            6 => {  
                Self {
                    temp_celsius: info[0..2].parse::<i32>().unwrap(),
                    dewpoint_celsius: info[4..].parse::<i32>().unwrap() - (2 * info[4..].parse::<i32>().unwrap()),
                }
            },
            7 => {
                Self {
                    temp_celsius: info[1..3].parse::<i32>().unwrap() - (2 * info[1..3].parse::<i32>().unwrap()),
                    dewpoint_celsius: info[5..].parse::<i32>().unwrap() - (2 * info[5..].parse::<i32>().unwrap()),
                }
            },
            _ => panic!("AAAA")
        }
    }
    pub fn get_temp(&self) -> i32 {
        self.temp_celsius
    }
    pub fn get_dewpoint(&self) -> i32 {
        self.dewpoint_celsius
    }
}
//TODO: use range for rvr
pub struct Rvr {
    rwy: String,
    vis: Visibility,
    upper_bound: Option<Visibility>,
}
impl Rvr {
    pub fn new(info: String) -> Self {
        // look below for better solution to everwhere there's unwrap
        let slash = info.find("/").expect("Couldn't parse rvr measurement: \"/\" not found where expected");
        let f = info.find("F").expect("Couldn't parse rvr measurement: \"FT\" not found where expected");
        let v = info.find("V");
        if let Some(i_v) = v {
            Self {
                rwy: String::from(&info[1..slash]),
                vis: Visibility::visibility_for_rvr(String::from(&info[slash + 1..i_v])),
                upper_bound: Some(Visibility::visibility_for_rvr(String::from(&info[i_v + 1..f]))), 
            }
        } else {
            Self {
                rwy: String::from(&info[1..slash]),
                vis: Visibility::visibility_for_rvr(String::from(&info[slash + 1..f])),
                upper_bound: None,
            }
        }
    }
    pub fn get_rwy(&self) -> &String {
        &self.rwy
    }
    pub fn get_vis(&self) -> &Visibility {
        &self.vis
    }
    pub fn get_upper_bound(&self) -> &Option<Visibility> {
        &self.upper_bound
    }
}
pub struct Weather {
    intensity: u8,
    proximity: u8,
    desc: u8,
    precip: u8,
    obscuration: u8,
    misc: u8,
}
impl Weather {
    pub fn new(info: String) -> Self {
        let mut i: u8 = 0;
        let mut po: u8 = 0;
        let mut d: u8 = 0;
        let mut pr: u8 = 0;
        let mut ob: u8 = 0;
        let mut m: u8 = 0;
        let mut current_index = 0;
        match &info[0..1] {
            "+" => {
                i = 3;
                current_index = 1;
            },
            "-" => {
                i = 1;
                current_index = 1;
            },
            _ => i = 2,
        }
        //TODO: make match more efficient
        while current_index < info.len() {
            match &info[current_index..=current_index + 1] {
                "VC" => po = 1,
                "MI" => d = 1,
                "PR" => d = 2,
                "BC" => d = 3,
                "DR" => d = 4,
                "BL" => d = 5,
                "SH" => d = 6,
                "TS" => d = 7, 
                "FZ" => d = 8,
                "DZ" => pr = 1,
                "RA" => pr = 2,
                "SN" => pr = 3,
                "SG" => pr = 4,
                "IC" => pr = 5,
                "PL" => pr = 6,
                "GR" => pr = 7,
                "GS" => pr = 8,
                "UP" => pr = 9,
                "BR" => ob = 1,
                "FG" => ob = 2,
                "FU" => ob = 3,
                "VA" => ob = 4,
                "DU" => ob = 5, 
                "SA" => ob = 6,
                "HZ" => ob = 7,
                "PY" => ob = 8,
                "PO" => misc = 1,
                "SQ" => misc = 2,
                "FC" => misc = 3,
                "SS" => misc = 4,
                "DS" => misc = 5, 
                _ => (),
            }
            current_index += 2;
        }
        Self {
            intensity: i,
            proximity: po,
            desc: d,
            precip: pr,
            obscuration: ob,
            misc: m,
        }
    }
}
mod tests {
    use crate::us_metar_components::*;
    //paused due to upcoming refactor
    #[test]
    fn check_when() {
        let w = When::new(String::from("291314Z"));
        assert_eq!(w.day_of_month, 29);
        assert_eq!(w.zulu_time, 1314);
    }
    #[test]
    fn check_wind() {
        let a = Wind::new(String::from("08717G24KT"));
        assert_eq!(a.spd, (17..24));
        assert_eq!(a.dir, (087..087));
        let b = Wind::new(String::from("08717KT"));
        assert_eq!(b.spd, (17..17));
        assert_eq!(b.dir, (087..087));
        let c = Wind::new(String::from("08717G24KT 086V088"));
        assert_eq!(c.spd, (17..24));
        assert_eq!(c.dir, (086..088));
        let d = Wind::new(String::from("VRB03G05KT"));
        assert_eq!(d.spd, (3..5));
        assert_eq!(d.dir, (999..999));
    }
    #[test]
    fn check_alt() {
        let a = Alt::new(String::from("A2973"));
        assert_eq!(a.alt_inhg, 29.73);
    }
    #[test] 
    fn check_temps() {
        let a = Temps::new(String::from("17/14"));
        let b = Temps::new(String::from("07/M03"));
        let c = Temps::new(String::from("M09/M10"));
        assert_eq!(a.temp_celsius, 17);
        assert_eq!(a.dewpoint_celsius, 14);
        assert_eq!(b.temp_celsius, 7);
        assert_eq!(b.dewpoint_celsius, -3);
        assert_eq!(c.temp_celsius, -9);
        assert_eq!(c.dewpoint_celsius, -10);
    }
    #[test]
    fn check_weather() {

    }
    #[test]
    fn check_visibility() {
        let a = Visibility::new(String::from("9SM"));
        let b = Visibility::new(String::from("M6SM"));
        let c = Visibility::new(String::from("P4SM"));
        let get_range = |p: Visibility| -> i32 {
            match p {
                Visibility::Plus(a) => a + 1,
                Visibility::Exact(b) => b,
                Visibility::Less(c) => c - 1,
            }
        };
        assert_eq!(9, get_range(a));
        assert_eq!(5, get_range(b));
        assert_eq!(5, get_range(c));
    }
    #[test]
    //needs rewrite
    fn check_clouds() {
        let a = Cloud_layer::new(String::from("SCT036"));
        assert_eq!({
            match a {
                Cloud_layer::Sct(b) => b,
                _ => 0
            }
        }, 3600);
    }
    #[test]
    fn check_rvr() {
        let t_v = |a: Visibility| -> String {
            match a {
                Visibility::Plus(e) => format!("+{}", e),
                Visibility::Exact(f) => format!("{}", f),
                Visibility::Less(g) => format!("-{}", g),
            }
        };
        let a = Rvr::new(String::from("R05L/1600FT"));
        assert_eq!(a.rwy, String::from("05L"));
        assert_eq!(t_v(a.vis), String::from("1600"));
        let b = Rvr::new(String::from("R27/1500V1700FT"));
        assert_eq!(b.rwy, String::from("27"));
        assert_eq!(t_v(b.vis), String::from("1500"));
        assert_eq!(t_v(b.upper_bound.unwrap()), String::from("1700"));
        let c = Rvr::new(String::from("R31/M1400VP1600FT"));
        assert_eq!(c.rwy, String::from("31"));
        assert_eq!(t_v(c.vis), String::from("-1400"));
        assert_eq!(t_v(c.upper_bound.unwrap()), String::from("+1600"));
    }
}