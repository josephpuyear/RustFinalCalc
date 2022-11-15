/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Calculator {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    display: String,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            // Example stuff:
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
                        _frame.close();
                    }
                }); 
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            
            let add : char = '+';
            let sub : char = '-';
            let mult : char = '*';
            let div : char = '/';

            let seven : char = '7';
            let eight : char = '8';
            let nine : char = '9';

            let four : char = '4';
            let five : char = '5';
            let six : char = '6';

            let one : char = '1';
            let two : char = '2';
            let three : char = '3';

            let zero : char = '0';


            ui.text_edit_singleline(display);

            if ui.button("Clear").clicked() {
                *display = " ".to_owned();
            }

            ui.horizontal(|ui| {
                    if ui.button("7").clicked() { 
                        display.push(seven);
                    }
                    if ui.button("8").clicked() { 
                        display.push(eight);
                    }
                    if ui.button("9").clicked() { 
                        display.push(nine);
                    }
                    if ui.button("*").clicked() {
                        display.push(' ');
                        display.push(mult);
                        display.push(' ');
                    }
            });

            ui.horizontal(|ui| {
                    if ui.button("4").clicked() { 
                        display.push(four);
                    }
                    if ui.button("5").clicked() { 
                        display.push(five);
                    }
                    if ui.button("6").clicked() { 
                        display.push(six);
                    }
                    if ui.button("-").clicked() {
                        display.push(' ');
                        display.push(sub);
                        display.push(' ');
                    }
            });

            ui.horizontal(|ui| {
                if ui.button("1").clicked() { 
                    display.push(one);
                }
                if ui.button("2").clicked() { 
                    display.push(two);
                }
                if ui.button("3").clicked() { 
                    display.push(three);
                }
                if ui.button("+").clicked() {  
                    display.push(' ');
                    display.push(add);
                    display.push(' ');
                }
        });

        
        use eval::{eval, to_value};
        ui.horizontal(|ui|{    
            if ui.button("0").clicked(){
                display.push(zero);
            }

            if ui.button("=").clicked() {
                let mut result = eval::eval(&*display).unwrap().as_i64().unwrap();
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
