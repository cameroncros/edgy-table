#![doc = include_str!("../README.md")]

pub mod cell;
mod line;
pub mod row;
pub mod segment;
pub mod table;

#[cfg(test)]
mod tests {
    use crate::cell::Cell;
    use crate::{cell, row, seg, table};
    use owo_colors::AnsiColors;

    #[test]
    fn test_render_simple() {
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
        table.render()
    }
}
