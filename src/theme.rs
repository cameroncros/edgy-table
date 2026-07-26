use crate::border_cell;
use crate::cell::Cell;

#[derive(Default)]
pub struct Theme {
    pub top_left: Option<Cell>,
    pub top_border: Option<Cell>,
    pub top_intersection: Option<Cell>,
    pub top_right: Option<Cell>,

    pub left_intersection: Option<Cell>,
    pub left_wall: Option<Cell>,
    pub vertical_wall: Option<Cell>,
    pub horizontal_wall: Option<Cell>,
    pub intersection: Option<Cell>,
    pub right_intersection: Option<Cell>,
    pub right_wall: Option<Cell>,

    pub bottom_left: Option<Cell>,
    pub bottom_border: Option<Cell>,
    pub bottom_intersection: Option<Cell>,
    pub bottom_right: Option<Cell>,
}

impl Theme {
    pub fn none() -> Self {
        Self {
            vertical_wall: Some(border_cell!["  "]),
            ..Default::default()
        }
    }

    pub fn basic() -> Self {
        Self {
            top_left: Some(border_cell!["┌"]),
            top_border: Some(border_cell!["─"]),
            top_intersection: Some(border_cell!["┬"]),
            top_right: Some(border_cell!["┐"]),
            left_intersection: Some(border_cell!["├"]),
            left_wall: Some(border_cell!["│"]),
            vertical_wall: Some(border_cell!["│"]),
            horizontal_wall: Some(border_cell!["─"]),
            intersection: Some(border_cell!["┼"]),
            right_intersection: Some(border_cell!["┤"]),
            right_wall: Some(border_cell!["│"]),
            bottom_left: Some(border_cell!["└"]),
            bottom_border: Some(border_cell!["─"]),
            bottom_intersection: Some(border_cell!["┴"]),
            bottom_right: Some(border_cell!["┘"]),
        }
    }

    pub fn wtf() -> Self {
        Self {
            top_left: Some(border_cell!["++\n++"]),
            top_border: Some(border_cell!["##\n##"]),
            top_intersection: Some(border_cell!["++\n++"]),
            top_right: Some(border_cell!["++\n++"]),
            left_intersection: Some(border_cell!["++\n++"]),
            left_wall: Some(border_cell!["##\n##"]),
            vertical_wall: Some(border_cell!["##\n##"]),
            horizontal_wall: Some(border_cell!["##\n##"]),
            intersection: Some(border_cell!["++\n++"]),
            right_intersection: Some(border_cell!["++\n++"]),
            right_wall: Some(border_cell!["##\n##"]),
            bottom_left: Some(border_cell!["++\n++"]),
            bottom_border: Some(border_cell!["##\n##"]),
            bottom_intersection: Some(border_cell!["++\n++"]),
            bottom_right: Some(border_cell!["++\n++"]),
        }
    }
}