use egui::*;

use crate::calendar::data::Calendar;
mod calendar;
mod calendar_read_write;

fn main () {
    //settings for the window
    let native_options = eframe::NativeOptions{
        viewport: eframe::egui::ViewportBuilder::default()

        //makes window resizable
        .with_resizable(true)

        //sets the size of the window
        .with_inner_size([1920.0, 1080.0]),

        ..Default::default()
    };

    eframe::run_native(
        "My Test App",
        native_options,
        Box::new(|cc| Ok(Box::new(CalendarApp::new(cc))))
    )
    .expect("Error building app!");
}

struct CalendarApp
{
    data: Calendar
}


impl CalendarApp{
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        //can customize egui using cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style
        
        //initialize app with calendar data
        Self { data: calendar::data::create_calendar() }
    }
}

impl eframe::App for CalendarApp {
    fn ui(
        &mut self, 
        ui: &mut Ui, 
        _frame: &mut eframe::Frame
    ) {
        CentralPanel::default().show_inside(ui, |ui| {
            
            // ui.heading(&self.data.months[0].name.to_string());

            // for day in &self.months[0].days {
            //     ui.label(day.weekday.to_string());
            // }

            todo!();
        });
    }
}