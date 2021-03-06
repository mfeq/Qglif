use sdl2::mouse::MouseButton;

use crate::editor::Editor;
use crate::user_interface::Interface;

// Pan
use super::{EditorEvent, MouseEventType, Tool, prelude::*};

#[derive(Clone)]
pub struct Zoom {}

impl Tool for Zoom {
    fn handle_event(&mut self, _v: &mut Editor, i: &mut Interface, event: EditorEvent) {
        match event {
            EditorEvent::MouseEvent { event_type, meta } => {
                match event_type {
                    MouseEventType::Released => { self.mouse_released(i, meta)}
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Zoom {
    pub fn new() -> Self {
        Self {}
    }

    fn mouse_released(&self, i: &mut Interface, meta: MouseInfo)
    {
        let current_scale = i.viewport.factor;
        let mut scale = i.viewport.factor;
        let mut offset = i.viewport.offset;

        match meta.button {
            MouseButton::Left => {
                scale = zoom_in_factor(scale, i);
            }
            MouseButton::Right => {
                scale = zoom_out_factor(scale, i);
            }
            _ => {}
        }

        let center = (i.viewport.winsize.0 as f32 / 2., i.viewport.winsize.1 as f32 / 2.);
        let diff = (meta.absolute_position.0 - center.0, meta.absolute_position.1 - center.1);
        offset.0 -= diff.0;
        offset.1 -= diff.1;
        offset.0 /= current_scale / scale;
        offset.1 /= current_scale / scale;

        i.update_viewport(Some(offset), Some(scale));
        i.center_cursor();
    }
}


pub fn zoom_in_factor(_factor: f32, i: &mut Interface) -> f32 {
    i.viewport.factor + SCALE_FACTOR
}

pub fn zoom_out_factor(_factor: f32, i: &mut Interface) -> f32 {
    let mut scale = i.viewport.factor;
    if scale >= 0.10 {
        scale += -SCALE_FACTOR;
    }
    scale
}
