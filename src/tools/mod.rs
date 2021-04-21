pub mod prelude;
use self::prelude::*;
use self::{measure::Measure, pan::Pan, pen::Pen, select::Select, zoom::Zoom};
use dyn_clone::DynClone;
use imgui::Ui;

pub use self::zoom::{zoom_in_factor, zoom_out_factor};

use crate::editor::Editor;

use sdl2::video::Window;
use sdl2::Sdl;

pub mod console;

pub mod pan;
pub mod pen;
pub mod select;
//pub mod vws;
pub mod zoom;
pub mod measure;

pub trait Tool: DynClone{
    fn handle_event(&mut self, v: &mut Editor, event: EditorEvent);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolEnum {
    Pan,
    Pen,
    Select,
    Zoom,
    Measure,
    VWS,
}

pub fn tool_enum_to_tool(tool: ToolEnum) -> Box<dyn Tool> {
    match tool {
        ToolEnum::Pan => {Box::new(Pan::new())}
        ToolEnum::Pen => {Box::new(Pen::new())}
        ToolEnum::Select => {Box::new(Select::new())}
        ToolEnum::Zoom => {Box::new(Zoom::new())}
        ToolEnum::Measure => {Box::new(Measure::new())}
        ToolEnum::VWS => {Box::new(Pan::new())} //FIXME: enable vws
    }
}

pub enum MouseEventType {
    Pressed,
    DoubleClick,
    Released,
    Moved
}

pub enum EditorEvent<'a> {
    MouseEvent {
        event_type: MouseEventType,
        meta: MouseInfo
    },

    Draw {
        skia_canvas:  &'a mut Canvas
    },

    Ui {
        ui: &'a mut Ui<'a>
    }
}

// Generic events
pub fn _center_cursor(v: &mut Editor, sdl_context: &Sdl, sdl_window: &Window) {
    let mut center = sdl_window.size();
    center.0 /= 2;
    center.1 /= 2;
    v.mouse_info.absolute_position = (center.0 as f32, center.1 as f32);

    sdl_context
        .mouse()
        .warp_mouse_in_window(&sdl_window, center.0 as i32, center.1 as i32);
}

// this gets called by tools so it accepts &mut State
pub fn update_viewport(
    v: &mut Editor,
    offset: Option<(f32, f32)>,
    scale: Option<f32>,
) {
    let offset = match offset {
        None => v.viewport.offset,
        Some(offset) => offset,
    };
    let scale = match scale {
        None => v.viewport.factor,
        Some(scale) => scale,
    };

    v.viewport.factor = scale;
    v.viewport.offset = offset;
}