use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use time::format_description::well_known;
use time::{OffsetDateTime, UtcOffset};

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

#[derive(Deserialize)]
struct Game {
    start_date: String,
}

#[derive(Deserialize)]
struct Play {
    wallclock: String,
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

#[derive(Clone)]
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

    let token = env::var("CFB_TOKEN").unwrap();

    let mut headers = BTreeMap::new();
    headers.insert("accepts".to_string(), "application/json".to_string());
    headers.insert("Authorization".to_string(), format!("Bearer {token}"));
    let mut request = ehttp::Request::get(format!(
        "https://api.collegefootballdata.com/games?year={}&week={}&seasonType=regular&team={}",
        year, week, team.school
    ));
    request.headers = headers;

    ehttp::fetch(request, move |response| {
        let game = serde_json::from_str::<Vec<Game>>(response.unwrap().text().unwrap()).unwrap();

        if !game.is_empty() {
            let token = env::var("CFB_TOKEN").unwrap();

            let mut headers = BTreeMap::new();
            headers.insert("accepts".to_string(), "application/json".to_string());
            headers.insert("Authorization".to_string(), format!("Bearer {token}"));
            let mut request = ehttp::Request::get(format!("https://api.collegefootballdata.com/plays?seasonType=regular&year={}&week={}&team={}", year, week, team.school));
            request.headers = headers;

            let clone = results.clone();

            ehttp::fetch(request, move |response| {
                let start_time =
                    OffsetDateTime::parse(&game.first().unwrap().start_date, &well_known::Rfc3339)
                        .expect("Failed to parse start date");
                let start_time_trans =
                    start_time.to_offset(UtcOffset::from_hms(offset, 0, 0).unwrap());

                let start = format!("{:0>2}:{:0>2}", start_time.hour(), start_time.minute());
                let start_trans = format!(
                    "{:0>2}:{:0>2}",
                    start_time_trans.hour(),
                    start_time_trans.minute()
                );

                let plays =
                    serde_json::from_str::<Vec<Play>>(response.unwrap().text().unwrap()).unwrap();

                if !plays.is_empty() {
                    let first = plays.first().unwrap();
                    let last = plays.last().unwrap();
                    let kickoff_time =
                        OffsetDateTime::parse(&first.wallclock, &well_known::Rfc3339)
                            .expect("Failed to parse kickoff time");
                    let kickoff_time_trans =
                        kickoff_time.to_offset(UtcOffset::from_hms(offset, 0, 0).unwrap());

                    let kickoff =
                        format!("{:0>2}:{:0>2}", kickoff_time.hour(), kickoff_time.minute());
                    let kickoff_trans = format!(
                        "{:0>2}:{:0>2}",
                        kickoff_time_trans.hour(),
                        kickoff_time_trans.minute()
                    );

                    let end_time = OffsetDateTime::parse(&last.wallclock, &well_known::Rfc3339)
                        .expect("Failed to parse end time");
                    let end_time_trans =
                        end_time.to_offset(UtcOffset::from_hms(offset, 0, 0).unwrap());

                    let end = format!("{:0>2}:{:0>2}", end_time.hour(), end_time.minute());
                    let end_trans = format!(
                        "{:0>2}:{:0>2}",
                        end_time_trans.hour(),
                        end_time_trans.minute()
                    );

                    clone.lock().unwrap().push(Result {
                        team: team.school.clone(),
                        start,
                        kickoff,
                        end,
                        start_trans,
                        kickoff_trans,
                        end_trans,
                    });
                } else {
                    results.lock().unwrap().push(Result {
                        team: team.school.clone(),
                        start,
                        kickoff: "".to_string(),
                        end: "".to_string(),
                        start_trans,
                        kickoff_trans: "".to_string(),
                        end_trans: "".to_string(),
                    });
                }
            });
        }
    });
}
