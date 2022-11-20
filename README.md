# Rust Caculator

[![dependency status](https://deps.rs/repo/github/emilk/eframe_template/status.svg)](https://deps.rs/repo/github/emilk/eframe_template)
[![Build Status](https://github.com/emilk/eframe_template/workflows/CI/badge.svg)](https://github.com/emilk/eframe_template/actions?workflow=CI)

This was created using a template repo for [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).

# Features

This calculator has several basic functions including additon, subtraction, multiplication, division and modulo. 

Users can click buttons or type directly in the display bar. 

There is a backspace and a clear button if mistakes are made while typing. 

Ongoing issues:
1. A few operators being put next to each other will make it crash. For example:
- A decimal between any two other operators.
- A single paranthase (or multiple) between any other operators without numbers.
- A few other edge cases that I won't list here. 
2. Overflow is an issue. Multiplying very large numbers with each other will cause the integers to get confused and end up at negative numbers. Not incredibly realistic to fix however, considering that the numbers have to be quite large. Doesn't crash though which I guess is a plus?