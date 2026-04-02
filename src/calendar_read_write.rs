pub mod file_io {

    use crate::calendar::data::Calendar;
    use std::io::{Read, Write};
    use std::path::Path;
    use std::fs::File;

    pub fn read_calendar_from_file() -> Option<Calendar>
    {
        //try to find pre-existing calendar data, if any
        let base_path = Path::new("data");
        let file_path = base_path.join("calendar.txt");

        let file_result = File::open(file_path);

        let mut file = match file_result {
            Ok(_) => file_result.unwrap(),
            Err(_) => return None
        };
            
        let mut buffer = String::new();
        
        match file.read_to_string(&mut buffer) {
            Ok(_) => {}
            Err(_) => return None
        };

        let calendar: Calendar = serde_json::from_str(buffer.as_str()).expect("JSON cannot be read!");

        Some(calendar)
    }

    pub fn write_calendar_to_file(calendar: &Calendar)
    {
        //serialize the given calendar to a JSON string
        let calendar_json = serde_json::to_string_pretty(&calendar)
        .expect("Failed to serialize calendar to JSON!");

        let base_path = Path::new("data");
        let file_path = base_path.join("calendar.txt");

        let mut file = File::create(file_path)
        .expect("Failed to create calendar file!");

        file.write_all(calendar_json.as_bytes())
        .expect("Failed to write JSON to file");
        println!("JSON data written to file!");
    }
}