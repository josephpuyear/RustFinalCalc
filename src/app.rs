/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct Calculator {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    //#[serde(skip)]

    display: String,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            display: "".to_owned(),
        }
    }
}

impl Calculator {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Calculator {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, display } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        *display = " ".to_owned();
                        _frame.close();
                    }
                }); 
            });
        });
        
        

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            //Numbers
            struct Number {
                zero : char,
                one : char,
                two : char,
                three : char,
                four : char,
                five : char,
                six : char,
                seven : char,
                eight : char,
                nine : char,
            }

            let number = Number {
                zero : '0',
                one : '1',
                two : '2',
                three : '3',
                four : '4',
                five : '5',
                six : '6',
                seven : '7',
                eight : '8',
                nine : '9',
            };


            //Operators
            struct Operator {
                dot : char,
                modd : char,
                add : char,
                sub : char,
                mult : char,
                div : char,
                lparen : char,
                rparen : char,
            }

            let operator = Operator {
                dot : '.',
                modd : '%',
                add : '+',
                sub : '-',
                mult : '*',
                div : '/',
                lparen : '(',
                rparen : ')',
            };


            //Make it so two operations in a row don't make it crash (match case?)
                //check by index, i.e. display[-1]? if last character an operator, then do nothing
            //Do not allow / by 0
            //Resize buttons
            //Change colors
            
        
            /* 
            use std::{thread, time};
            let ten_mills = time::Duration::from_millis(10);
            let x = 1;

            if display.chars().nth(x).unwrap() == add{
                let temp_display = display.clone();
                *display = "Please only type one operator at a time!".to_owned();
                thread::sleep(ten_mills);
                *display = temp_display.to_string();



            use egui::Vec2;
            pub const X: Vec2 = Vec2{ x: 1.0, y: 0.0,};

            if v[v.len()-2] == add | sub | mult | div | modd {
                        let temp_display = display.clone();
                        *display = "Please only type one operator at a time!".to_owned();
                        thread::sleep(ten_mills);
                        *display = temp_display.to_string();
                        }
                    }
                    else{ //add char to screen
                    }
            
            let button = ui.set_min_size(X);
            */
            
            ui.heading("Rust Calculator");
            ui.horizontal(|ui| {
                ui.label("Written by Joseph Puyear");
            });

            let response = ui.text_edit_singleline(display);
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                let mut result = eval::eval(&*display).unwrap().as_f64().unwrap();
                *display = result.to_string();
            }

            ui.horizontal(|ui| {
                if ui.add_sized([140.,40.], egui::Button::new("Clear")).clicked() {
                    *display = " ".to_owned();
                }
                if ui.add_sized([140.,40.], egui::Button::new("<-")).clicked() {
                    display.pop();
                }
            });

            ui.horizontal(|ui| {   
                if ui.add_sized([66.,40.], egui::Button::new("(")).clicked() {
                    display.push(operator.lparen);
                }
                if ui.add_sized([66.,40.], egui::Button::new(")")).clicked() {
                    display.push(operator.rparen);
                }         
                if ui.add_sized([66.,40.], egui::Button::new("%")).clicked() {
                    display.push(' ');
                    display.push(operator.modd);
                    display.push(' ');
                }
                if ui.add_sized([66.,40.], egui::Button::new("/")).clicked() {
                    display.push(' ');
                    display.push(operator.div);
                    display.push(' ');
                }
                
            });

            ui.horizontal(|ui| {
                if ui.add_sized([66.,40.], egui::Button::new("7")).clicked() { 
                    display.push(number.seven);
                }
                if ui.add_sized([66.,40.], egui::Button::new("8")).clicked() { 
                    display.push(number.eight);
                }
                if ui.add_sized([66.,40.], egui::Button::new("9")).clicked() { 
                    display.push(number.nine);
                }
                if ui.add_sized([66.,40.], egui::Button::new("*")).clicked() {
                    display.push(' ');
                    display.push(operator.mult);
                    display.push(' ');
                }
            });

            ui.horizontal(|ui| {
                if ui.add_sized([66.,40.], egui::Button::new("4")).clicked() { 
                    display.push(number.four);
                }
                if ui.add_sized([66.,40.], egui::Button::new("5")).clicked() { 
                    display.push(number.five);
                }
                if ui.add_sized([66.,40.], egui::Button::new("6")).clicked() { 
                    display.push(number.six);
                }
                if ui.add_sized([66.,40.], egui::Button::new("-")).clicked() {
                    display.push(' ');
                    display.push(operator.sub);
                    display.push(' ');
                }
            });

            ui.horizontal(|ui| {
                if ui.add_sized([66.,40.], egui::Button::new("1")).clicked() { 
                    display.push(number.one);
                }
                if ui.add_sized([66.,40.], egui::Button::new("2")).clicked() { 
                    display.push(number.two);
                }
                if ui.add_sized([66.,40.], egui::Button::new("3")).clicked() { 
                    display.push(number.three);
                }
                if ui.add_sized([66.,40.], egui::Button::new("+")).clicked() {
                    display.push(' ');
                    display.push(operator.add);
                    display.push(' ');
                }  
            });

        use eval::{eval, to_value};
        ui.horizontal(|ui|{  
            if ui.add_sized([140.,40.], egui::Button::new("0")).clicked(){
                display.push(number.zero);
            }
            
            if ui.add_sized([66.,40.], egui::Button::new(".")).clicked(){
                display.push(operator.dot);
            }

            if ui.add_sized([66.,40.], egui::Button::new("=")).clicked() {
                let mut result = eval::eval(&*display).unwrap().as_f64().unwrap();
                *display = result.to_string();
            }
        });

            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
