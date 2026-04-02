pub mod data{

    use chrono::prelude::*;
    use serde::{Serialize, Deserialize};

    use crate::calendar_read_write;
    
    #[derive(Default, Serialize, Deserialize)]
    pub struct Calendar {
        year: i32,
        months: Vec<CalendarMonth>
    }

    #[derive(Default, Serialize, Deserialize)]
    struct CalendarMonth {
        name: String,
        days: Vec<CalendarDay>
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

    pub fn create_calendar() -> Calendar {

        //read calendar data from file, if it exists
        let existing_calendar_data = calendar_read_write::file_io::read_calendar_from_file();

        match existing_calendar_data {
            Some(_) => return existing_calendar_data.unwrap(),
            None => {}
        }

        println!("No calendar data found, creating new data!");

        //get the current year
        let current_year = Local::now().year();

        //create a new empty calendar based on this year
        let mut calendar = Calendar {
            year: current_year,
            months : Vec::new()
        };

        let mut current_month_in_process = Month::January;

        //iterate through all months in this year and create new structs for each
        loop {

            //create a new hashmap of empty calendar days corresponding to their days of the current month in the process
            let calendar_days = 

            //iterate from the 1st to the last day of the month
            (1..current_month_in_process.num_days(current_year).unwrap() + 1)
            
            //map each number day to a tuple of the number day and a CalendarDay struct
            .map(|day|
            
                CalendarDay {
                    weekday: NaiveDate::from_ymd_opt(
                        current_year, 
                        current_month_in_process.number_from_month(),
                        day.into()
                    )
                    .unwrap()
                    .weekday()
                    .to_string(),
                    events: None
                }
            )

            //convert this mapping into a HashMap for the CalendarMonth struct
            .collect();

            //create a new CalendarMonth for the current month and populate it with the above data
            let calendar_month = CalendarMonth {
                name: String::from(current_month_in_process.name()),
                days: calendar_days
            };

            //add the new CalendarMonth to the Calendar struct
            calendar.months.push(calendar_month);

            //the loop ends after December has been processed
            if current_month_in_process == Month::December {
                break;
            }
            //move to the next month if not December
            else {
                current_month_in_process = current_month_in_process.succ();
            }
        }

        calendar_read_write::file_io::write_calendar_to_file(&calendar);

        calendar
    }
}