use crate::editor::Editor;
use super::prelude::*;

// Pan is a good example of a simple tool. It holds only a little bit of state, an optional position.
// If the optional position is set the tool will calculate the delta between that position and the current
// mouse position and set the camera accordingly.
#[derive(Clone)]
pub struct Pan {
    last_position: Option<(f32, f32)>
}


// We implement Tool for our tool. Here you can route events to functions or implement logic directly in the
// match statement.
impl Tool for Pan {
    fn handle_event(&mut self, v: &mut Editor, event: EditorEvent) {
        match event {
            EditorEvent::MouseEvent { event_type, meta } => {
                match event_type {
                    MouseEventType::Moved => { self.mouse_moved(v, meta) }
                    MouseEventType::Pressed => { self.mouse_pressed(v, meta) }
                    MouseEventType::Released => { self.mouse_released(v, meta) }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

// Here you can implement behaviors for events. 
impl Pan {
    pub fn new() -> Self {
        Self {
            last_position: None
        }
    }

    fn mouse_moved(&mut self, v: &mut Editor, meta: MouseInfo) {
        if !meta.is_down { return }
        if let Some(pivot_point) = self.last_position {
            // calculate delta and offset camera
            let mut offset = v.viewport.offset;

            offset.0 += (meta.absolute_position.0 - pivot_point.0).floor() as f32;
            offset.1 += (meta.absolute_position.1 - pivot_point.1).floor() as f32;
           
            v.viewport.offset = offset;

            //update last mouse position
            self.last_position = Some(meta.absolute_position);
        }
    }

    // When the mouse is pressed we store the point.
    fn mouse_pressed(&mut self, _v: &mut Editor, meta: MouseInfo) {
        self.last_position = Some(meta.absolute_position);
    }

    // When it's released we set it to none.
    fn mouse_released(&mut self, _v: &mut Editor, _meta: MouseInfo) {
        self.last_position = None;
    }
}