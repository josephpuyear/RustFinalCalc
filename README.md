# Rust Caculator

[![dependency status](https://deps.rs/repo/github/emilk/eframe_template/status.svg)](https://deps.rs/repo/github/emilk/eframe_template)
[![Build Status](https://github.com/emilk/eframe_template/workflows/CI/badge.svg)](https://github.com/emilk/eframe_template/actions?workflow=CI)

This was created using a template repo for [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).

# Features

This calculator has several basic functions including additon, subtraction, multiplication, division and modulo. 

Users can click buttons or type directly in the display bar. 

There is a backspace and a clear button if mistakes are made while typing. 

Things that will make it crash:
1. Dividing by 0
2. Clicking = with only an operator in the display
3. Clicking = with two operators next to each other 
    [This includes decimals and parentheses] 
    [Sometimes decimals or . can be put next to parentheses, as long as a number is the next character, such as (.5) ] 
    [Expressions can also be put next to each other, such as (1 + 2)(1 + 2), so long as both are in paranthesis]
4. Clicking = with a number next to a parenthese without another operator between