Edgy-Table
==========

Simple table library, opposite of comfy-table, which you should probably be using instead.
Handles multi-colors, and variable width characters slightly better than comfy, but then also
does a lot less than comfy.

Example Usage:

```
# use edgy_table::{seg, row, cell, table};
# use edgy_table::{cell::Cell, renderer::Renderer, renderer::Color, theme::Theme};
# use owo_colors::AnsiColors;
let christmas_cell = Cell::from(vec![
    seg!["\t"],
    seg!["H", AnsiColors::Red],
    seg!["E", AnsiColors::Green],
    seg!["L", AnsiColors::Blue],
    seg!["L", AnsiColors::Green],
    seg!["O", AnsiColors::Red],
]);

let christmas_cell2 = Cell::from(vec![
    seg!["H", AnsiColors::Red],
    seg!["E", AnsiColors::Green],
    seg!["L", AnsiColors::Blue],
    seg!["L", AnsiColors::Green],
    seg!["O", AnsiColors::Red],
    seg!["\n", AnsiColors::Red],
    seg!["W", AnsiColors::Magenta],
    seg!["O", AnsiColors::Cyan],
    seg!["R", AnsiColors::Yellow],
    seg!["L", AnsiColors::Cyan],
    seg!["D", AnsiColors::Magenta],
]);

let table = table![
    row![christmas_cell, cell!["Hello\n\n\n\tWorld"],],
    row![cell!["Ｈｅｌｌｏ, ｗｏｒｌｄ!"], christmas_cell2,]
];
let mut renderer = Renderer::new(&Color::Off, Theme::basic());
table.render(&mut renderer).unwrap();
renderer.to_stdout().unwrap();
```