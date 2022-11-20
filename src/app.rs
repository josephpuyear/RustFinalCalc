/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct Calculator {
// Variables used
    label: String,                          // This is used for testing
    display: String,
}


impl Default for Calculator {
    fn default() -> Self {
        Self {
            label: "".to_owned(),           // This is used for testing
            display: "".to_owned(),
        }
    }
}


impl Calculator {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look and feel of the gui

        // Sets gui to dark mode
        cc.egui_ctx.set_visuals(egui::Visuals::dark());


    // Load previous app state (if any) 
    // Disabled for now because I found it annoying during testing
        /* 
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        */

        Default::default()
    }
}


// Creates a new App Calculator
impl eframe::App for Calculator {


    // Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }


    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put the widgets into a `SidePanel`, `TopsPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, display } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!


    // Tops panel allows for a menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        *display = "".to_owned();
                        _frame.close();
                    }
                });
            });
        });


        egui::Window::new("Hello and welcome!").resizable(false).default_pos(egui::pos2(0.0, 0.0)).show(ctx, |ui| {
            ui.label("Hello! My name is Joseph Puyear and this is my calculator that I created using Rust and the egui framework. You can click and drag the windows to move them or you can collapse them at any time by clicking the arrow on the top left of each frame. Please feel free to give this calculator a test drive but know that there are still many bugs that need to be fixed. Hope you enjoy playing around with my Rust Calculator!");
        });


        egui::Window::new("GitHub Link").resizable(false).default_pos(egui::pos2(0.0, 160.0)).show(ctx, |ui| {
            ui.hyperlink("https://github.com/josephpuyear/RustFinalCalc");
        });


    // Adds a "Central Panel" to the gui, which is where everything is located for the calculator
        //#[cfg(target_arch = "wasm32")]
        //egui::CentralPanel::default().show(ctx, |ui| {    
        egui::Window::new("Rust Calculator").resizable(false).default_pos(egui::pos2(355.0, 0.0)).show(ctx, |ui| {         
        

    // Numbers
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

            let nums = Number {
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


    // Operators
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

            let ops = Operator {
                dot : '.',
                modd : '%',
                add : '+',
                sub : '-',
                mult : '*',
                div : '/',
                lparen : '(',
                rparen : ')',
            };

    
            ui.label("");

    // This allows the user to press "Enter", rather than click "=" to get a result
            let response = ui.text_edit_singleline(display);
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                let mut result = eval::eval(&*display).unwrap().as_f64().unwrap();
                *display = result.to_string();
            }


    // Clear and Backspace buttons        
            ui.horizontal(|ui| {
                if ui.add_sized([140.,40.], egui::Button::new("Clear")).clicked() {
                    *display = "".to_owned();
                }
                if ui.add_sized([140.,40.], egui::Button::new("<---")).clicked() {
                    display.pop();
                }
            });


    // Parantheses, Modulo and Divide buttons
            ui.horizontal(|ui| {   
                if ui.add_sized([66.,40.], egui::Button::new("(")).clicked() {
                    display.push(ops.lparen);
                }
                if ui.add_sized([66.,40.], egui::Button::new(")")).clicked() {
                    display.push(ops.rparen);
                }         
                if ui.add_sized([66.,40.], egui::Button::new("Mod")).clicked() {
                    display.push(' ');
                    display.push(ops.modd);
                    display.push(' ');
                }
                if ui.add_sized([66.,40.], egui::Button::new("/")).clicked() {
                    display.push(' ');
                    display.push(ops.div);
                    display.push(' ');
                }
                
            });

    
    // Seven, Eight, Nine, Multiply buttons
            ui.horizontal(|ui| {
                if ui.add_sized([66.,40.], egui::Button::new("7")).clicked() { 
                    display.push(nums.seven);
                }
                if ui.add_sized([66.,40.], egui::Button::new("8")).clicked() { 
                    display.push(nums.eight);
                }
                if ui.add_sized([66.,40.], egui::Button::new("9")).clicked() { 
                    display.push(nums.nine);
                }
                if ui.add_sized([66.,40.], egui::Button::new("*")).clicked() {
                    display.push(' ');
                    display.push(ops.mult);
                    display.push(' ');
                }
            });


    // Four, Five, Six, Subtract buttons
            ui.horizontal(|ui| {
                if ui.add_sized([66.,40.], egui::Button::new("4")).clicked() { 
                    display.push(nums.four);
                }
                if ui.add_sized([66.,40.], egui::Button::new("5")).clicked() { 
                    display.push(nums.five);
                }
                if ui.add_sized([66.,40.], egui::Button::new("6")).clicked() { 
                    display.push(nums.six);
                }
                if ui.add_sized([66.,40.], egui::Button::new("-")).clicked() {
                    display.push(' ');
                    display.push(ops.sub);
                    display.push(' ');
                }
            });

            
    // One, Two, Three, Add buttons
            ui.horizontal(|ui| {
                if ui.add_sized([66.,40.], egui::Button::new("1")).clicked() { 
                    display.push(nums.one);
                }
                if ui.add_sized([66.,40.], egui::Button::new("2")).clicked() { 
                    display.push(nums.two);
                }
                if ui.add_sized([66.,40.], egui::Button::new("3")).clicked() { 
                    display.push(nums.three);
                }
                if ui.add_sized([66.,40.], egui::Button::new("+")).clicked() {
                    display.push(' ');
                    display.push(ops.add);
                    display.push(' ');
                }
            });


    // Zero, Decimal and Equals buttons
            use eval::{eval, to_value};
            ui.horizontal(|ui|{  
                if ui.add_sized([140.,40.], egui::Button::new("0")).clicked() {
                    display.push(nums.zero);
                }
                
                if ui.add_sized([66.,40.], egui::Button::new(".")).clicked() {
                    display.push(ops.dot);
                }

                if ui.add_sized([66.,40.], egui::Button::new("=")).clicked() {
                    if *display == ops.dot.to_string(){
                        *display = "".to_owned();
                    }
                    if *display == " + "{
                        *display = "".to_owned();
                    }
                    if *display == " - "{
                        *display = "".to_owned();
                    }
                    if *display == " * "{
                        *display = "".to_owned();
                    }
                    if *display == " / "{
                        *display = "".to_owned();
                    }
                    if *display == " % "{
                        *display = "".to_owned();
                    }
                    if *display == " " || *display == "" {
                        *display = "".to_owned();
                    }
                    else {
                    let mut result = eval::eval(&*display).unwrap().as_f64().unwrap();
                    *display = result.to_string();
                    }
                }
        

    // Corrects multiple opss and divide by zero 
            if display.contains(" / 0") {
                *display = display.clone().replace(" / 0", "");
            }

            if display.contains("/ 0") {
                *display = display.clone().replace("/ 0", "");
            }
            

    // Additions corrections
            if display.contains("+  +"){
                *display = display.clone().replace("+  +", "+");
            }
            
            if display.contains("+  -"){
                *display = display.clone().replace("+  -", "+");
            }

            if display.contains("+  *"){
                *display = display.clone().replace("+  *", "+");
            }

            if display.contains("+  /"){
                *display = display.clone().replace("+  /", "+");
            }

            if display.contains("+  %"){
                *display = display.clone().replace("+  %", "+");
            }


    // Subtraction corrections
            if display.contains("-  -"){
                *display = display.clone().replace("-  -", "-");
            }
    
            if display.contains("-  +"){
                *display = display.clone().replace("-  +", "-");
            }
            
            if display.contains("-  *"){
                *display = display.clone().replace("-  *", "-");
            }

            if display.contains("-  /"){
                *display = display.clone().replace("-  /", "-");
            }

            if display.contains("-  %"){
                *display = display.clone().replace("-  %", "-");
            }


    // Multiplication corrections
            if display.contains("*  *"){
                *display = display.clone().replace("*  *", "*");
            }

            if display.contains("*  +"){
                *display = display.clone().replace("*  +", "*");
            }

            if display.contains("*  -"){
                *display = display.clone().replace("*  -", "*");
            }

            if display.contains("*  /"){
                *display = display.clone().replace("*  /", "*");
            }

            if display.contains("*  %"){
                *display = display.clone().replace("*  %", "*");
            }


    // Division corrections 
            if display.contains("/  /"){
                *display = display.clone().replace("/  /", "/");
            }

            if display.contains("/  +"){
                *display = display.clone().replace("/  +", "/");
            }

            if display.contains("/  -"){
                *display = display.clone().replace("/  -", "/");
            }

            if display.contains("/  *"){
                *display = display.clone().replace("/  *", "/");
            }

            if display.contains("/  %"){
                *display = display.clone().replace("/  %", "/");
            }


    // Decimal correction
            if display.contains(".."){
                *display = display.clone().replace("..", ".");
            }

            if display.contains(" . "){
                *display = display.clone().replace(" . ", ".");
            }

            if display.contains("(.)"){
                *display = display.clone().replace("(.)", ".");
            }


    // Modulo corrections        
            if display.contains("%  %"){
                *display = display.clone().replace("%  %", "%");
            }

            if display.contains("%  +"){
                *display = display.clone().replace("%  +", "%");
            }

            if display.contains("%  -"){
                *display = display.clone().replace("%  -", "%");
            }

            if display.contains("%  *"){
                *display = display.clone().replace("%  *", "%");
            }

            if display.contains("%  /"){
                *display = display.clone().replace("%  /", "%");
            }
    

    // Parantheses corrections
            if display.contains("()"){
                *display = display.clone().replace("()", "(");
            }

            if display.contains(")("){
                *display = display.clone().replace(")(", ")");
            }

            if display.contains(").("){
                *display = display.clone().replace(").(", ")");
            }

    // Obviously this is a ridiculous amount of if-statements. I wanted to do a match case
    // but it got confusing since match can't know what's inside of display.contains() without
    // me telling it what I'm looking for. Feedback on how I might be able to do this would
    // be very much appreciated. 

        });

            //egui::warn_if_debug_build(ui);
    });
    /*
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
            
        }
    */
    }
}
