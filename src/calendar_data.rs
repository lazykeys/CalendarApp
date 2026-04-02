// pub mod file_io {
//     use crate::Calendar;
//     use std::io::{self, Read};
//     use std::path::Path;
//     use std::fs::File;
//     use serde::{Serialize, Deserialize};

//     pub fn read() -> Option<Calendar>
//     {
//         //try to find pre-existing calendar data, if any
//         let base_path = Path::new("data");
//         let file_path = base_path.join("calendar.txt");

//         let file_result = File::open(file_path);

//         let file = match file_result {
//             Ok(file) => file,
//             Err(error) => return None
//         };

//         let mut buffer = String::new();
//         file.read_to_string(&mut buffer);
//         println!("File content:\n{}", buffer);
//     }

//     pub fn write(calendar: Calendar)
//     {
        
//     }
// }