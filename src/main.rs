pub mod us_metar_components;
pub mod other_metar;
pub mod better_name;
use crate::better_name::*;
use crate::us_metar_components::*;
//TODO: fails to handle variable wind, deal with ownership/memory better, think about SPECI and amendments, region METAR was observed
fn parse_us(met: &str)  {
    let metar = String::from(met);
    let mut s = metar.split_whitespace();
    let mut a: Vec<String> = Vec::new();
    'outer: for s1 in s {
        if(s1 != "METAR" && s1 != "AUTO") {
            if(s1 == "RMK") {
                break 'outer; //TODO: functionality for parsing RMK section
            }
            a.push(String::from(s1));
        } else if(s1 == "AUTO") {
            println!("This METAR is an automated observation")
        }
    }
    let num_variable_components = a.len() - 6;
    //TODO: find solution to this other than clone
    println!("\n\nStation: {}", a.get(0).unwrap().clone());
    println!("{}", When::parse(a.get(1).unwrap().clone()));
    println!("{}", Wind::parse(a.get(2).unwrap().clone()));
    println!("{}", Visibility::parse(a.get(3).unwrap().clone()));
    let mut curr_index = 4;
    for i in (0..num_variable_components) {
        let element = a.get(curr_index).unwrap().clone();
        if(element.len() > 7) {
            println!("{}", Rvr::parse(element.clone()));
        } else if(element.len() == 6 || &element[0..1] == "S" || &element[0..1] == "C") {
            println!("{}", CloudLayer::parse(element.clone()));
        } else {
            println!("{}", Weather::parse(element.clone()));
        }
        curr_index += 1;
    }
    println!("{}", Temps::parse(a.get(curr_index).unwrap().clone()));
    curr_index += 1;
    println!("{}\n", Alt::parse(a.get(curr_index).unwrap().clone()));
}
fn main() {
    parse_us("KTKI 071653Z 20007KT 10SM CLR 35/18 A2999 RMK AO2 SLP147 T03500178");
    parse_us("KORD 071651Z 11004KT 10SM SCT032 SCT200 OVC250 28/19 A3002 RMK AO2 SLP163 T02830194");
    parse_us("KSEA 071653Z 24006KT 10SM FEW025 BKN070 BKN120 BKN200 19/12 A3005 RMK AO2 SLP180 T01890122");
    parse_us("KPIA 071654Z 00000KT 2SM VCTS RA BR BKN009 OVC016 23/22 A3004 RMK AO2 LTG DSNT S TSB30E45 SLP167 P0017 T02280222");
    parse_us("KDFW 011242Z 35022G28KT 1SM R17C/5500VP6000FT -SN BR SCT009 BKN017 OVC027 M06/M07 A2988 RMK AO2 PK WND 33040/1202 SFC VIS 1 3/4 PLE16 PRESFR");
}




