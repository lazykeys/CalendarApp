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
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //can customize egui using cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style
        set_styles(&cc.egui_ctx);
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

        //the frame of the main window that shows up when the app is run
        let main_frame = egui::containers::Frame {

            //the background color of the window
            fill: Color32::LIGHT_BLUE,
            ..Default::default()
        };

        //the main window
        CentralPanel::default().frame(main_frame).show_inside(ui, |ui| {

            let calendar_data = &self.data;

            //the name of the month
            ui.heading(calendar_data.months[0].name.to_string());
        });
    }
}

//sets the global styles of the app
fn set_styles(ctx: &Context) {
    let mut global_style = (*ctx.global_style()).clone();

    //sets custom styles for text elements
    global_style.text_styles = [

        //header elements
        (TextStyle::Heading, FontId::new(30.0, FontFamily::Monospace)),

        //body text elements
        (TextStyle::Body, FontId::new(18.0, FontFamily::Monospace)),

        //button elements
        (TextStyle::Button, FontId::new(22.0, FontFamily::Monospace)),

        //small text elements
        (TextStyle::Small, FontId::new(14.0, FontFamily::Monospace))
    ].into();

    ctx.set_global_style(global_style);
}