use crate::{get_results, TimeZone};
use chrono::{Date, Utc};
use eframe::egui::{vec2, Context};
use eframe::{egui, Frame};
use egui_extras::{Size, TableBuilder};
use std::sync::{Arc, Mutex};

pub struct CfbTimePuller {
    outlet: String,
    year: u16,
    week: u8,
    desired_timezone: TimeZone,
    dst: bool,
    results: Arc<Mutex<Vec<crate::Result>>>,
    date: Date<chrono::Utc>,
}

impl CfbTimePuller {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            outlet: String::new(),
            year: 2022,
            week: 1,
            desired_timezone: TimeZone::Eastern,
            dst: false,
            results: Arc::new(Mutex::new(Vec::new())),
            date: Utc::today(),
        }
    }
}

impl eframe::App for CfbTimePuller {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    egui::Grid::new("filters")
                        .num_columns(2)
                        .spacing(vec2(5.0, 5.0))
                        .show(ui, |ui| {
                            ui.label("Outlet: ");
                            egui::ComboBox::from_id_source("cfb_team")
                                .width(150.0)
                                .selected_text(&self.outlet)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.outlet, "ABC".to_string(), "ABC");
                                    ui.selectable_value(
                                        &mut self.outlet,
                                        "ACCN".to_string(),
                                        "ACCN",
                                    );
                                    ui.selectable_value(&mut self.outlet, "BTN".to_string(), "BTN");
                                    ui.selectable_value(
                                        &mut self.outlet,
                                        "CBSSN".to_string(),
                                        "CBSSN",
                                    );
                                    ui.selectable_value(&mut self.outlet, "CBS".to_string(), "CBS");
                                    ui.selectable_value(
                                        &mut self.outlet,
                                        "ESPN".to_string(),
                                        "ESPN",
                                    );
                                    ui.selectable_value(
                                        &mut self.outlet,
                                        "ESPN2".to_string(),
                                        "ESPN2",
                                    );
                                    ui.selectable_value(&mut self.outlet, "NBC".to_string(), "NBC");
                                    ui.selectable_value(
                                        &mut self.outlet,
                                        "NFLN".to_string(),
                                        "NFLN",
                                    );
                                    ui.selectable_value(
                                        &mut self.outlet,
                                        "PAC12".to_string(),
                                        "PAC12",
                                    );
                                    ui.selectable_value(&mut self.outlet, "FOX".to_string(), "FOX");
                                    ui.selectable_value(&mut self.outlet, "FS1".to_string(), "FS1");
                                    ui.selectable_value(
                                        &mut self.outlet,
                                        "SECN".to_string(),
                                        "SECN",
                                    );
                                });
                            ui.end_row();
                            ui.label("Date: ");
                            ui.add(
                                egui_extras::DatePickerButton::new(&mut self.date)
                                    .id_source("dt_picker")
                                    .calendar(true),
                            );
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
                        let clone = self.results.clone();
                        get_results(
                            &self.outlet,
                            self.year,
                            self.week,
                            self.desired_timezone,
                            self.dst,
                            self.date.to_string(),
                            clone,
                        );
                    }
                });

                ui.vertical(|ui| {
                    TableBuilder::new(ui)
                        .columns(Size::remainder().at_least(30.0), 8)
                        .resizable(true)
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Game");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Date");
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
                                    ui.heading("Conv. Start");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Conv. Kickoff");
                                });
                            });
                            header.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.heading("Conv. End");
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
                                        ui.label(&result.date);
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
