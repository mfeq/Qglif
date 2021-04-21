use sdl2::mouse::MouseButton;

use crate::editor::Editor;

// Pan
use super::{EditorEvent, MouseEventType, Tool, prelude::*};

#[derive(Clone)]
pub struct Zoom {}

impl Tool for Zoom {
    fn handle_event(&mut self, v: &mut Editor, event: EditorEvent) {
        match event {
            EditorEvent::MouseEvent { event_type, meta } => {
                match event_type {
                    MouseEventType::Released => { self.mouse_released(v, meta)}
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

    fn mouse_released(&self, v: &mut Editor, meta: MouseInfo)
    {
        let mut scale = v.viewport.factor;
        match meta.button {
            MouseButton::Left => {
                scale = zoom_in_factor(scale, v);
            }
            MouseButton::Right => {
                scale = zoom_out_factor(scale, v);
            }
            _ => {}
        }
        let mut offset = v.viewport.offset;
        let winsize = v.viewport.winsize;
        let position = meta.absolute_position;
        let center = (
            (winsize.0 as f32 / 2.) + offset.0,
            (winsize.1 as f32 / 2.) + offset.1,
        );
        offset.0 = -(position.0 as f32 - center.0);
        offset.1 = -(position.1 as f32 - center.1);
        update_viewport(v, Some(offset), Some(scale));
    }
}


pub fn zoom_in_factor(_factor: f32, v: &mut Editor) -> f32 {
    v.viewport.factor + SCALE_FACTOR
}

pub fn zoom_out_factor(_factor: f32, v: &mut Editor) -> f32 {
    let mut scale = v.viewport.factor;
    if scale >= 0.10 {
        scale += -SCALE_FACTOR;
    }
    scale
}