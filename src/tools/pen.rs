
use super::prelude::*;
use crate::renderer::UIPointType;
use crate::renderer::points::draw_point;

use MFEKmath::{Bezier, evaluate::Primitive};
use editor::util::get_contour_start_or_end;
#[derive(Clone)]
pub struct Pen {}

impl Tool for Pen {
    fn handle_event(&mut self, v: &mut Editor, event: EditorEvent) {
        match event {
            EditorEvent::MouseEvent { event_type, meta } => {
                match event_type {
                    super::MouseEventType::Pressed => { self.mouse_pressed(v, meta) }
                    super::MouseEventType::Released => { self.mouse_released(v, meta)}
                    super::MouseEventType::Moved => { self.mouse_moved(v, meta) }
                    _ => {}
                }
            }
            EditorEvent::Draw { skia_canvas } => { 
                self.draw_nearest_point(v, skia_canvas);
                self.draw_merge_preview(v, skia_canvas);
             }
            _ => {}
        }
    }
}

impl Pen {
    pub fn new() -> Self {
        Self {}
    }

    fn mouse_moved(&self, v: &mut Editor, meta: MouseInfo) {
        if !meta.is_down { return };

        if let Some(idx) = v.contour_idx {
            let mousepos = meta.position;
            v.with_active_layer_mut(|layer| {
                let outline = get_outline_mut!(layer);
                let last_point = outline[idx].last().unwrap().clone();

                let pos = (calc_x(mousepos.0 as f32), calc_y(mousepos.1 as f32));
                let offset = (last_point.x - pos.0, last_point.y - pos.1);
                let handle_b = (last_point.x + offset.0, last_point.y + offset.1);

                outline[idx].last_mut().unwrap().a = Handle::At(calc_x(mousepos.0 as f32), calc_y(mousepos.1 as f32));
                outline[idx].last_mut().unwrap().b = Handle::At(handle_b.0, handle_b.1);
            });
        }
    }

    fn mouse_pressed(&self, v: &mut Editor, meta: MouseInfo) {
        v.begin_layer_modification("Add point.");


        // We check if we have a point selected and are clicking on the beginning of another contour.
        // If that is the case we merge them and then return.
        if let (Some(c_idx), Some(p_idx)) = (v.contour_idx, v.point_idx) {
            // we've clicked a handle?
            if let Some(info) = clicked_point_or_handle(v, meta.position, None) {
                // we have the end of one contour active and clicked the start of another?
                let end_is_active = get_contour_start_or_end(v, c_idx, p_idx) == Some(SelectPointInfo::End);
                let start_is_clicked = get_contour_start_or_end(v, info.0, info.1) == Some(SelectPointInfo::Start);

                // make sure these contours are open
                let selected_open = v.with_active_layer(|layer| get_contour_type!(layer, c_idx)) == PointType::Move;
                let target_open = v.with_active_layer(|layer| get_contour_type!(layer, info.0)) == PointType::Move;
                if end_is_active && start_is_clicked && selected_open && target_open {
                    v.with_active_layer_mut(|layer| {
                        get_contour_mut!(layer, c_idx).push(Point::from_x_y_type(
                        (calc_x(meta.position.0 as f32), calc_y(meta.position.1 as f32)),
                        PointType::Curve
                        ));
                    });
                    v.merge_contours(info.0, c_idx);
                    return;
                }
            }
    
        }

        // Next we check if our mouse is over an existing curve. If so we add a point to the curve and return.
        if let Some(info) = nearest_point_on_curve(v, meta.position) {
            v.with_active_layer_mut(|layer| {
                let mut second_idx_zero = false;
                let contour = &mut layer.outline.as_mut().unwrap()[info.contour_idx];
                let mut point = contour.remove(info.seg_idx);
                let mut next_point = if info.seg_idx == contour.len() {
                    second_idx_zero = true;
                    contour.remove(0)
                } else { 
                    contour.remove(info.seg_idx) 
                };

                let bez = Bezier::from(&point, &next_point);
                let subdivisions = bez.subdivide(info.t);

                if let Some(subdivisions) = subdivisions {
                    let (sub_a, sub_b) = (subdivisions.0.to_control_points(), subdivisions.1.to_control_points());
                    point.a = sub_a[1].to_handle();
                    next_point.b = sub_b[2].to_handle();

                    if second_idx_zero { 
                        contour.insert(0, next_point);
                    } else {
                        contour.insert(info.seg_idx, next_point);
                    }

                    let (x, y) = (sub_a[3].x, sub_a[3].y);
                    contour.insert(info.seg_idx, Point{
                        x: x as f32,
                        y: y as f32,
                        a: sub_b[1].to_handle(),
                        b: sub_a[2].to_handle(),
                        name: None,
                        ptype: PointType::Curve,
                        data: None,

                    });

                    contour.insert(info.seg_idx, point);
                }
            });
            return
        }

        // If we've got the end of a contour selected with continue drawing that contour and return.
        if let Some(contour_idx) = v.contour_idx {
            let mouse_pos = meta.position;
            let contour_len = v.with_active_layer(|layer| {get_outline!(layer)[contour_idx].len()});

            if v.point_idx.unwrap() == contour_len - 1 {
                v.point_idx = v.with_active_layer_mut(|layer| {
                    let outline = get_outline_mut!(layer);
                    outline[contour_idx].push(Point::from_x_y_type(
                    (calc_x(mouse_pos.0 as f32), calc_y(mouse_pos.1 as f32)),
                    PointType::Curve,
                    ));
    
                    Some(outline[contour_idx].len() - 1)
                });
                return
            }
        }


        // Lastly if we get here we create a new contour.
        let mouse_pos = meta.position;
        v.contour_idx = v.with_active_layer_mut(|layer| {
            let outline = get_outline_mut!(layer);
            let mut new_contour: Contour<PointData> = Vec::new();
            new_contour.push(Point::from_x_y_type(
                (calc_x(mouse_pos.0 as f32), calc_y(mouse_pos.1 as f32)),
                if meta.modifiers.shift {
                    PointType::Move
                } else {
                    PointType::Curve
                },
            ));
            outline.push(new_contour);

            Some(outline.len() - 1)
        });
        v.point_idx = Some(0);
    }

    fn mouse_released(&self, v: &mut Editor, _meta: MouseInfo) {
        // No matter what a mouse press generates a layer modification so we have to finalize that here.
        if let Some(idx) = v.contour_idx {
            v.with_active_layer_mut(|layer| {
                get_outline_mut!(layer)[idx].last_mut().map(|point| {
                    if point.a != Handle::Colocated && point.ptype != PointType::Move {
                        point.ptype = PointType::Curve;
                    }
                });
            });
        }

        v.end_layer_modification();
    }

    fn draw_nearest_point(&self, v: &mut Editor, canvas: &mut Canvas) {
        if v.mouse_info.is_down { return };
        let info = nearest_point_on_curve(v, v.mouse_info.position);

        if let Some(info) = info {
            draw_point(
                v,
                (calc_x(info.point.0), calc_y(info.point.1)),
                info.point,
                None,
                UIPointType::Point((Handle::At(info.a.0, info.a.1), Handle::At(info.b.0, info.b.1))),
                true,
                canvas
            )
        }
    }

    fn draw_merge_preview(&self, v: &Editor, canvas: &mut Canvas) {
        // we've got a point selected?
        if v.contour_idx.is_some() && v.point_idx.is_some() {
            // we've clicked a handle?
            if let Some(info) = clicked_point_or_handle(v, v.mouse_info.position, None) {
                let c_idx = v.contour_idx.unwrap();
                let p_idx = v.contour_idx.unwrap();

                // we have the end of one contour active and clicked the start of another?
                let end_is_active = get_contour_start_or_end(v, c_idx, p_idx) == Some(SelectPointInfo::End);
                let start_is_clicked = get_contour_start_or_end(v, info.0, info.1) == Some(SelectPointInfo::Start);

                // make sure these contours are open
                let selected_open = v.with_active_layer(|layer| get_contour_type!(layer, c_idx)) == PointType::Move;
                let target_open = v.with_active_layer(|layer| get_contour_type!(layer, info.0)) == PointType::Move;
                if end_is_active && start_is_clicked && selected_open && target_open {
                    let point = v.with_active_layer(|layer| get_contour!(layer, info.0)[info.1].clone());
                    draw_point(
                        v,
                        (calc_x(point.x), calc_y(point.y)),
                        (point.x, point.y),
                        None,
                        UIPointType::Point((point.a, point.b)),
                        true,
                        canvas
                    );
                }
            }
    
        }
    }
}