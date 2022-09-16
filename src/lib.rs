use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub mod app;

#[derive(Copy, Clone, PartialOrd, PartialEq, Default, Ord, Eq)]
#[repr(u8)]
pub enum TimeZone {
    #[default]
    Eastern,
    Central,
    Mountain,
    Pacific,
}

impl TimeZone {
    fn to_suffix(self) -> &'static str {
        match self {
            TimeZone::Eastern => "ET",
            TimeZone::Central => "CT",
            TimeZone::Mountain => "MT",
            TimeZone::Pacific => "PT",
        }
    }
}

impl Display for TimeZone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeZone::Eastern => write!(f, "Eastern"),
            TimeZone::Central => write!(f, "Central"),
            TimeZone::Mountain => write!(f, "Mountain"),
            TimeZone::Pacific => write!(f, "Pacific"),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Result {
    pub team: String,
    pub start: String,
    pub kickoff: String,
    pub end: String,
    pub start_trans: String,
    pub kickoff_trans: String,
    pub end_trans: String,
    pub date: String,
}

pub fn get_results(
    outlet: &str,
    year: u16,
    week: u8,
    target_tz: TimeZone,
    dst: bool,
    date: String,
    results: Arc<Mutex<Vec<Result>>>,
) {
    let mut offset = match target_tz {
        TimeZone::Eastern => -5,
        TimeZone::Central => -6,
        TimeZone::Mountain => -7,
        TimeZone::Pacific => -8,
    };

    if dst {
        offset += 1;
    }

    let request = ehttp::Request::get(format!(
        "https://war-helper.com/time?year={}&week={}&offset={}&outlet={}",
        year, week, offset, outlet
    ));

    let date = date.trim_end_matches("UTC").to_string();

    ehttp::fetch(request, move |response| {
        let mut result = serde_json::from_str::<Vec<Result>>(response.unwrap().text().unwrap()).unwrap();

        result = result.into_iter().filter_map(|mut r| {
            if r.date == date {
                r.start = format!("{} UTC", r.start);
                r.kickoff = format!("{} UTC", r.kickoff);
                r.end = format!("{} UTC", r.end);

                r.start_trans = format!("{} {}", r.start_trans, target_tz.to_suffix());
                r.kickoff_trans = format!("{} {}", r.kickoff_trans, target_tz.to_suffix());
                r.end_trans = format!("{} {}", r.end_trans, target_tz.to_suffix());
                Some(r)
            } else {
                None
            }
        }).collect();

        *results.lock().unwrap() = result;
    });
}
