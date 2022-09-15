use crate::{get_results, get_teams, Team, TimeZone};
use eframe::egui::{vec2, Context};
use eframe::{egui, Frame};
use egui_extras::{Size, TableBuilder};
use std::sync::{Arc, Mutex};

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Default)]
pub struct CfbTimePuller {
    teams: Arc<Mutex<Vec<Team>>>,
    team: Option<Team>,
    year: u16,
    week: u8,
    desired_timezone: TimeZone,
    dst: bool,
    results: Arc<Mutex<Vec<crate::Result>>>,
    auth: String,
}

impl CfbTimePuller {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            teams: Arc::new(Mutex::new(Vec::new())),
            team: None,
            year: 2022,
            week: 1,
            desired_timezone: TimeZone::Eastern,
            dst: false,
            results: Arc::new(Mutex::new(Vec::new())),
            auth: "".to_string(),
        }
    }
}

impl eframe::App for CfbTimePuller {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.text_edit_singleline(&mut self.auth);
                    ui.add_space(5.0);
                    if ui.button("Authenticate").clicked() {
                        get_teams(self.auth.clone(), self.teams.clone());
                    }
                    ui.add_space(10.0);
                    egui::Grid::new("filters")
                        .num_columns(2)
                        .spacing(vec2(5.0, 5.0))
                        .show(ui, |ui| {
                            ui.label("Team: ");
                            egui::ComboBox::from_id_source("cfb_team")
                                .width(150.0)
                                .selected_text(if let Some(team) = &self.team {
                                    &team.school
                                } else {
                                    "None"
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.team, None, "None");
                                    for team in self.teams.lock().unwrap().iter() {
                                        ui.selectable_value(
                                            &mut self.team,
                                            Some(team.clone()),
                                            &team.school,
                                        );
                                    }
                                });
                            ui.end_row();
                            ui.label("Year: ");
                            ui.add(egui::DragValue::new(&mut self.year).clamp_range(0..=2022));
                            ui.end_row();
                            ui.label("Week: ");
                            ui.add(egui::DragValue::new(&mut self.week).clamp_range(1..=52));
                            ui.end_row();
                            ui.label("Timezone: ");
                            egui::ComboBox::from_id_source("cmb_tz")
                                .selected_text(format!("{}", self.desired_timezone))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.desired_timezone,
                                        TimeZone::Eastern,
                                        "Eastern",
                                    );
                                    ui.selectable_value(
                                        &mut self.desired_timezone,
                                        TimeZone::Central,
                                        "Central",
                                    );
                                    ui.selectable_value(
                                        &mut self.desired_timezone,
                                        TimeZone::Mountain,
                                        "Mountain",
                                    );
                                    ui.selectable_value(
                                        &mut self.desired_timezone,
                                        TimeZone::Pacific,
                                        "Pacific",
                                    );
                                });
                            ui.end_row();
                            ui.label("Daylight Savings Time: ");
                            ui.checkbox(&mut self.dst, "");
                        });
                    ui.add_space(10.0);
                    let button = ui.add_sized(vec2(200.0, 15.0), egui::Button::new("Submit"));
                    if button.clicked() {
                        if let Some(team) = &self.team {
                            let clone = self.results.clone();
                            get_results(
                                self.auth.clone(),
                                team.clone(),
                                self.year,
                                self.week,
                                self.desired_timezone,
                                self.dst,
                                clone,
                            );
                        }
                    }
                });

                ui.vertical(|ui| {
                    TableBuilder::new(ui)
                        .columns(Size::remainder().at_least(30.0), 7)
                        .resizable(true)
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("School");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Start");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Kickoff");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("End");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Converted Start");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Converted Kickoff");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Converted End");
                                });
                            });
                        })
                        .body(|body| {
                            let results = self.results.lock().unwrap();
                            body.rows(15.0, results.len(), |index, mut row| {
                                let result = &results[index];
                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(&result.team);
                                    });
                                });
                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(&result.start);
                                    });
                                });
                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(&result.kickoff);
                                    });
                                });
                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(&result.end);
                                    });
                                });
                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(&result.start_trans);
                                    });
                                });
                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(&result.kickoff_trans);
                                    });
                                });
                                row.col(|ui| {
                                    ui.centered_and_justified(|ui| {
                                        ui.label(&result.end_trans);
                                    });
                                });
                            });
                        });
                });
            });
        });
    }
}
