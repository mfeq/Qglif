
use glifparser::{PointType, glif::{Layer, MFEKContour, MFEKPointData}};

use crate::contour_operations;
use super::Editor;
use super::util::is_point_selected;

impl Editor {
    /// Copy the current selection and put it in our clipboard. 
    pub fn copy_selection(&mut self) {        
        let layer = &self.glyph.as_ref().unwrap().layers[self.layer_idx.unwrap()];
        let mut new_outline: Vec<MFEKContour<MFEKPointData>> = Vec::new();
        for (contour_idx, contour) in layer.outline.iter().enumerate() {
            let mut results = Vec::new();
            let mut cur_contour = Vec::new();

            let mut begin = 0;

            let mut deleted = false;
            for (point_idx, point) in contour.inner.iter().enumerate() {
                let to_delete = !is_point_selected(self, contour_idx, point_idx);

                if to_delete {
                    let mut mfekcur: MFEKContour<MFEKPointData> = cur_contour.into();
                    mfekcur.operation = contour_operations::sub(&contour, begin, point_idx);
                    results.push(mfekcur);

                    cur_contour = Vec::new();
                    deleted = true;
                    begin = point_idx + 1;
                } else  {
                    cur_contour.push(point.clone());
                }
            }
            let mut mfekcur: MFEKContour<MFEKPointData> = cur_contour.into();
            mfekcur.operation = contour_operations::sub(&contour, begin, contour.inner.len() - 1);
            results.push(mfekcur);

            if results.len() > 1 && contour.inner.first().unwrap().ptype != PointType::Move {
                let mut move_to_front = results.pop().unwrap().clone();
                move_to_front.inner.append(&mut results[0].inner);
                move_to_front.operation = contour_operations::append(&move_to_front, &results[0]);
                results[0] = move_to_front;
            }

            for mut result in results {
                if result.inner.len() != 0 {
                    if deleted {
                        result.inner.first_mut().unwrap().ptype = PointType::Move;
                    }
                    new_outline.push(result); 
                }
            }
        }

        self.clipboard = Some(Layer{
            name: "".to_string(),
            visible: true,
            color: None,
            outline: new_outline,
            operation: None,
            images: layer.images.clone(),
        })
    }

    pub fn paste_selection(&mut self, _position: (f32, f32)) {
        self.begin_layer_modification("Paste clipboard.");
        if let Some(clipboard) = &self.clipboard {
            self.contour_idx = None;
            self.point_idx = None;
            self.selected.clear();

            let layer = &mut self.glyph.as_mut().unwrap().layers[self.layer_idx.unwrap()];
            for contour in clipboard.outline.iter() {
                let cur_idx = layer.outline.len();
                for (point_selection, _) in contour.inner.iter().enumerate() {
                    self.selected.insert((cur_idx, point_selection));
                }
                layer.outline.push(contour.clone());
            }
        }
        self.end_layer_modification();
    }

    pub fn delete_selection(&mut self) {
        self.begin_layer_modification("Delete selection.");
        
        let layer = &self.glyph.as_ref().unwrap().layers[self.layer_idx.unwrap()];
        let mut new_outline: Vec<MFEKContour<MFEKPointData>> = Vec::new();
        for (contour_idx, contour) in layer.outline.iter().enumerate() {
            let mut results = Vec::new();
            let mut cur_contour = Vec::new();

            let mut begin = 0;

            let mut deleted = false;
            for (point_idx, point) in contour.inner.iter().enumerate() {
                let to_delete = is_point_selected(self, contour_idx, point_idx);

                if to_delete {
                    let mut mfekcur: MFEKContour<MFEKPointData> = cur_contour.into();
                    mfekcur.operation = contour_operations::sub(&contour, begin, point_idx);
                    results.push(mfekcur);

                    cur_contour = Vec::new();
                    deleted = true;
                    begin = point_idx + 1;
                } else  {
                    cur_contour.push(point.clone());
                }
            }
            let mut mfekcur: MFEKContour<MFEKPointData> = cur_contour.into();
            mfekcur.operation = contour_operations::sub(&contour, begin, contour.inner.len() - 1);
            results.push(mfekcur);

            if results.len() > 1 && contour.inner.first().unwrap().ptype != PointType::Move {
                let mut move_to_front = results.pop().unwrap().clone();
                move_to_front.inner.append(&mut results[0].inner);
                move_to_front.operation = contour_operations::append(&move_to_front, &results[0]);
                results[0] = move_to_front;
            }

            for mut result in results {
                if result.inner.len() != 0 {
                    if deleted {
                        result.inner.first_mut().unwrap().ptype = PointType::Move;
                    }
                    new_outline.push(result); 
                }
            }
        }

        self.glyph.as_mut().unwrap().layers[self.layer_idx.unwrap()].outline = new_outline;

        self.end_layer_modification();

        self.contour_idx = None;
        self.point_idx = None;
        self.selected.clear();
    }
}
