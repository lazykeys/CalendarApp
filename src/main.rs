use egui::*;
use eframe::*;

use crate::calendar::data::{Calendar, CalendarMonth};
mod calendar;
mod calendar_read_write;

fn main () {
    //settings for the window
    let native_options = NativeOptions{
        viewport: ViewportBuilder::default()

        //makes window resizable
        .with_resizable(true)

        //sets the size of the window
        .with_inner_size([
            960.0, //height 
            540.0  //width
        ])

        //sets the minimum size of the window
        .with_min_inner_size([
            480.0, 
            270.0
        ]),

        ..Default::default()
    };

    run_native(
        "My Test App",
        native_options,
        Box::new(|cc| Ok(Box::new(CalendarApp::new(cc))))
    )
    .expect("Error building app!");
}

struct CalendarApp
{
    data: Calendar,
    current_month_num: usize
}

impl CalendarApp{
    fn new(cc: &CreationContext<'_>) -> Self {
        set_styles(&cc.egui_ctx);
        set_visuals(&cc.egui_ctx);

        //initialize app with calendar data
        Self { data: calendar::data::create_calendar(), current_month_num: 0 }
    }

    //displays the given CalendarMonth and its related data in the central panel
    fn show_month_grid(&mut self, ui: &mut Ui, month: &CalendarMonth) {        
        //creates a grid for all of the days in the given CalendarMonth
        Grid::new("month_grid")

        //controls sizing of columns and rows
        .min_col_width(100.0)
        .max_col_width(100.0)
        .min_row_height(100.0)

        //controls spacing between columns and rows
        .spacing(Vec2::new(0.0, 0.0))

        //displays the grid and its contents
        .show(ui, |ui| {

            //find the amount of "dummy days", being the empty calendar day cells before real days start
            let mut dummy_count = 0;
            for _dummy_days in 0..month.days[0].days_from_sunday {
                dummy_count += 1;

                get_day_frame(10).show(ui, |ui| {
                    ui.label("");
                });
            }

            //add a new day cell to the calendar
            for day in &month.days {

                get_day_frame(10).show(ui, |ui| {
                    ui.label(format!("{}, {}", day.day_of_the_week.to_string(), day.number.to_string()));
                });

                //ends the row after every 7 cells, including dummies, are added
                let day_grid_cell_number = day.number + dummy_count;
                if day_grid_cell_number > 0 && day_grid_cell_number % 7 == 0 {
                    ui.end_row();
                }
            }
        });
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
            fill: Color32::from_rgb(
                84,  //red 
                168, //green
                144  //blue
            ),
            ..Default::default()
        };

        //the main window
        CentralPanel::default().frame(main_frame).show_inside(ui, |ui| {

            let current_month = self.data.months[self.current_month_num].clone();

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                
                egui::Frame::default().inner_margin(50).show(ui, |ui| {
                    ui.heading(&current_month.name);
                });

                egui::Frame::default().show(ui, |ui| {
                    self.show_month_grid(ui, &current_month);
                });
            });

            if ui.button("Next").clicked() {

                if self.current_month_num == self.data.months.len() - 1 {
                    self.current_month_num = 0;
                }
                else {
                    self.current_month_num += 1;
                }
            }

            if ui.button("Previous").clicked() {
                
                if self.current_month_num == 0 {
                    self.current_month_num = self.data.months.len() - 1;
                }
                else {
                    self.current_month_num -= 1;
                }
            }
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

fn set_visuals(ctx: &Context) {
    let mut global_style = (*ctx.global_style()).clone();

    global_style.visuals = Visuals {
        override_text_color: Some(Color32::from_rgb(
            5,
            5,
            5
        )),
        ..Default::default()
    };

    ctx.set_global_style(global_style);
}

fn get_day_frame(size: i8) -> egui::Frame {
    egui::Frame{

        //size of the frame
        inner_margin: Margin{
            left: size,
            right: size,
            top: size,
            bottom: size,
        },

        outer_margin: Margin { left: 0, right: 0, top: 0, bottom: 0 },

        //outline of the frame
        stroke: Stroke{
            width: 1.0, 
            color: Color32::BLACK
        },

        ..Default::default()
    }
}