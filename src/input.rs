use gtk4::{gdk, GestureStylus};
use p2d::bounding_volume::AABB;
use std::collections::VecDeque;

use crate::strokes::strokestyle::InputData;

pub const INPUT_OVERSHOOT: f64 = 30.0;

/// Map pen input to the position on a sheet
pub fn map_inputdata(
    zoom: f64,
    data_entries: &mut VecDeque<InputData>,
    mapped_offset: na::Vector2<f64>,
) {
    data_entries.iter_mut().for_each(|inputdata| {
        *inputdata = InputData::new(
            inputdata.pos().scale(1.0 / zoom) + mapped_offset,
            inputdata.pressure(),
        )
    });
}

/// Filter inputdata to sheet bounds
pub fn filter_mapped_inputdata(filter_bounds: AABB, data_entries: &mut VecDeque<InputData>) {
    data_entries.retain(|data| filter_bounds.contains_local_point(&na::Point2::from(data.pos())));
}

/// Retreive inputdata from a (emulated) pointer
pub fn retreive_pointer_inputdata(x: f64, y: f64) -> VecDeque<InputData> {
    let mut data_entries: VecDeque<InputData> = VecDeque::with_capacity(1);
    //std::thread::sleep(std::time::Duration::from_millis(100));

    data_entries.push_back(InputData::new(
        na::vector![x, y],
        InputData::PRESSURE_DEFAULT,
    ));
    data_entries
}

/// Retreives available input axes, defaults if not available. X and Y is already available from closure, and should not retreived from .axis() (because of gtk-rs weirdness)
pub fn retreive_stylus_inputdata(
    gesture_stylus: &GestureStylus,
    with_backlog: bool,
    x: f64,
    y: f64,
) -> VecDeque<InputData> {
    let mut data_entries: VecDeque<InputData> = VecDeque::new();
    //std::thread::sleep(std::time::Duration::from_millis(100));

    if with_backlog {
        if let Some(backlog) = gesture_stylus.backlog() {
            for logentry in backlog {
                let axes = logentry.axes();
                let x = axes[1];
                let y = axes[2];
                let pressure = axes[5];
                //log::debug!("{:?}", axes);
                data_entries.push_back(InputData::new(na::vector![x, y], pressure));
            }
        }
    }

    // Get newest data
    let pressure = if let Some(pressure) = gesture_stylus.axis(gdk::AxisUse::Pressure) {
        pressure
    } else {
        InputData::PRESSURE_DEFAULT
    };

    data_entries.push_back(InputData::new(na::vector![x, y], pressure));

    data_entries
}
