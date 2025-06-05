// pub fn cadence_parser(s: String) -> u8 {
// s.strip_suffix("rpm").unwrap().trim().parse().unwrap()
// }

// pub fn hr_parser(s: String) -> u8 {
//     s.strip_suffix("bpm").unwrap().trim().parse().unwrap()
// }
//
// pub fn grade_parser(s: String) -> f64 {
//     s.strip_suffix("%").unwrap().trim().parse().unwrap()
// }
//
// pub fn meters_parser(s: String) -> f64 {
//     s.strip_suffix("m").unwrap().trim().parse().unwrap()
// }
//
// pub fn ms_parser(s: String) -> f64 {
//     s.strip_suffix("m/s").unwrap().trim().parse().unwrap()
// }
//
// pub fn temp_parser(s: String) -> i8 {
//     s.strip_suffix("C").unwrap().trim().parse().unwrap()
// }
//
// pub fn semicircles_parser(s: String) -> i32 {
//     s.strip_suffix("semicircles")
//         .unwrap()
//         .trim()
//         .parse()
//         .unwrap()
// }

// pub fn power_parser(s: String) -> u16 {
//     s.strip_suffix("watts").unwrap().trim().parse().unwrap()
// }
//
// pub fn kcal_parser(s: String) -> f64 {
//     s.strip_suffix("kcal").unwrap().trim().parse().unwrap()
// }
//
// pub fn second_parser(s: String) -> f64 {
//     s.strip_suffix("s").unwrap().trim().parse().unwrap()
// }

pub fn unitless_u16_parser(s: String) -> u16 {
    s.trim().parse().unwrap()
}

pub fn triple_u16_array(s: String) -> (u16, u16, u16) {
    let a: Vec<u16> = s
        .strip_suffix("]")
        .unwrap()
        .strip_prefix("[")
        .unwrap()
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| {
            v.trim()
                .strip_prefix("UInt16(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .parse::<u16>()
                .unwrap()
        })
        .collect();
    if a.len() == 3 {
        (a[0], a[1], a[2])
    } else {
        unreachable!()
    }
}
