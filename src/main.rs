use chrono::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Result;
mod calendar_data;

fn main () {
    let native_options = eframe::NativeOptions::default();
    let app_result = eframe::run_native(
        "My Test App",
        native_options,
        Box::new(|cc| Ok(Box::new(Calendar::new(cc))))
    );

    match app_result {
        Ok(_) => {},
        Err(_) => println!("Error building app"),
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Calendar {
    year: i32,
    months: HashMap<u8, CalendarMonth>
}

#[derive(Default, Serialize, Deserialize)]
struct CalendarMonth {
    name: String,
    days: HashMap<u8, CalendarDay>
}

#[derive(Default, Serialize, Deserialize)]
struct CalendarDay {
    weekday: String,
    events: Option<Vec<CalendarEvent>>
}

#[derive(Default, Serialize, Deserialize)]
struct CalendarEvent {
    name: String,
    time: String,
    location: String
}

impl Calendar {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        //can customize egui using cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style

        //get the current year
        let current_year = Local::now().year();

        //create a new empty calendar based on this year
        let mut calendar = Calendar {
            year: current_year,
            months : HashMap::new()
        };

        let mut current_month_in_process = Month::January;

        //iterate through all months in this year and create new structs for each
        loop {

            //create a new hashmap of empty calendar days corresponding to their days of the current month in the process
            let calendar_days = 

            //iterate from the 1st to the last day of the month
            (1..current_month_in_process.num_days(current_year).unwrap() + 1)
            
            //map each number day to a tuple of the number day and a CalendarDay struct
            .map(|d|
            
                (d, 
                CalendarDay {
                    weekday: NaiveDate::from_ymd_opt(
                        current_year, 
                        current_month_in_process.number_from_month(),
                        d.into())
                        .unwrap()
                        .weekday()
                        .to_string(),
                    events: None
                })

            )

            //convert this mapping into a HashMap for the CalendarMonth struct
            .collect::<HashMap<u8, CalendarDay>>();

            //create a new CalendarMonth for the current month and populate it with the above data
            let calendar_month = CalendarMonth {
                name: String::from(current_month_in_process.name()),
                days: calendar_days
            };

            //add the new CalendarMonth to the Calendar struct
            calendar.months.insert(
                current_month_in_process.number_from_month().try_into().unwrap(),
                calendar_month
            );

            //the loop ends after December has been processed
            if current_month_in_process == Month::December {
                break;
            }
            //move to the next month if not December
            else {
                current_month_in_process = current_month_in_process.succ();
            }
        }

        calendar
        //TODO: read year from saved file, if it is different than saved data year, create new data
    }
}

impl eframe::App for Calendar {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading(self.months.len().to_string());
        });
    }
}