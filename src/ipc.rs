use log::error;
use mfek_ipc::{self, Available, IPCInfo};
use glifparser::{Guideline, GuidelinePoint, IntegerOrFloat};

use crate::editor::Editor;

use std::{process, str};

pub fn fetch_metrics(v: &mut Editor) {
    let (status, qmdbin) = mfek_ipc::module_available("metadata".into());
    if status != Available::Yes { return }
    let filename = v.with_glyph(|glyph| {glyph.filename.clone()});
    let ipc_info = IPCInfo::from_glif_path("MFEKglif".to_string(), &filename.unwrap());

    match &ipc_info.font.as_ref() {
        Some(ref font) => {
            let command = process::Command::new(qmdbin)
                .arg(font)
                .args(&["arbitrary", "-k", "ascender", "-k", "descender"])
                .output()
                .expect("No output, font corrupt?");

            let lines_vec = str::from_utf8(&command.stdout).unwrap();
            let mut lines_iter = lines_vec.lines();
            let nlines = lines_iter.count();
            lines_iter = lines_vec.lines();

            if nlines != 2 {
                error!("Cannot set ascender/descender, font corrupt?");
            } else {
                let names = &["ascender", "descender"];
                for (i, line) in lines_iter.enumerate() {
                    v.with_glyph_mut(|glyph|glyph.guidelines.push(Guideline {
                        at: GuidelinePoint {x: 0., y: line.parse().expect("Font is corrupt, metrics not numeric!")},
                        angle: IntegerOrFloat::Float(0.),
                        name: Some(names[i].to_string()),
                        color: None,
                        identifier: None
                    }));
                }
            }
        }
        None => {
            error!("Cannot set metrics, .glif file not part of a UFO!");
        }
    }

    v.ipc_info = Some(ipc_info);
}    

