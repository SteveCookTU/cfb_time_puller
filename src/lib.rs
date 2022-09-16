use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub mod app;

#[derive(Deserialize, Clone)]
pub struct Team {
    pub id: u16,
    pub school: String,
}

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
}

pub fn get_results(
    team: Team,
    year: u16,
    week: u8,
    target_tz: TimeZone,
    dst: bool,
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
        "http://18.191.220.43:8080/time?year={}&week={}&offset={}&team={}",
        year, week, offset, team.school
    ));

    ehttp::fetch(request, move |response| {
        let mut result = serde_json::from_str::<Result>(response.unwrap().text().unwrap()).unwrap();
        result.start = format!("{} UTC", result.start);
        result.kickoff = format!("{} UTC", result.kickoff);
        result.end = format!("{} UTC", result.end);

        result.start_trans = format!("{} {}", result.start_trans, target_tz.to_suffix());
        result.kickoff_trans = format!("{} {}", result.kickoff_trans, target_tz.to_suffix());
        result.end_trans = format!("{} {}", result.end_trans, target_tz.to_suffix());
        results.lock().unwrap().push(result);
    });
}
