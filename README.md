# Rust Caculator

This was created using a template repo for [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).


# Features

This calculator has several basic functions including additon, subtraction, multiplication, division and modulo. 

Users can click buttons or type directly in the display bar. 

There is a backspace and a clear button if mistakes are made while typing. 


# How it works

This calculator uses egui and eframe to create containers which house widgets. There are many different kinds of containers and widgets, but I will breifly discuss which ones are used here. 

My program primarily uses the Window container. This container is movable (if enabled) within the window/browser it is being run in and is very easy to size. 

For the calculator component, the Button widget is used the most. This widget allows for input detection (such as clicking), and can allow me to write messages to the display. 
- The display itself is made up of the text_edit_singleline widget, which can be typed in or just used as a display. 
- The previous input and result displays are also made up of the same widget. 

Formatting is done through a number of ways, but I used mostly the horizontal function which allows widgets to be placed next to each other in a row. In a few other uses, I used the collapsing format, which allows for a collapsable tab (that can be repeated to form trees). 

There are also many ways of styling widgets, but there are almost too many different functions to go into. Some of the main ones include coloring, font styles, and window sizing. I mainly focused on the highest functionality that I could achieve, but I plan on continuing to work on this to make it look much nicer than its current version. 

My explaination video will also go into a few more specifics about how the code works, although I must say some of it certainly seems like magic to me. The eframe framework makes it incredibly easy to compile this program to Web-Assembly and run inside the web browser, so the specific details of that are still somewhat of a mystery to me. 

That video can be found here: *video not yet made*

# Ongoing issues:

1. A few operators being put next to each other will make it crash. For example:
- A decimal between any two other operators.
- A single paranthase (or multiple) between any other operators without numbers.
- A few other edge cases that I won't list here. 
2. Overflow is an issue. Multiplying very large numbers with each other will cause the integers to get confused and end up at negative numbers. Not incredibly realistic to fix however, considering that the numbers have to be quite large. Doesn't crash though which I guess is a plus?
3. Too many operations can (sometimes?) cause a crash.