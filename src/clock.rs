use std::f32::consts::PI;
use std::iter::once;
use std::ops::RangeInclusive;
use chrono::{NaiveTime, Timelike};
use egui::{Align2, Color32, FontFamily, FontId, Painter, Pos2, Stroke, vec2};
use egui::epaint::PathShape;

const RESOLUTION: f32 = 2.5;

pub(crate) fn draw_clock_timestamp(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    timestamp: NaiveTime,
    accent_color: Color32,
) {
    let ts = (timestamp.num_seconds_from_midnight() as f32) +
        ((timestamp.nanosecond() as f32) / 1_000_000_000.0);
    let hour = ts / 3600.0 % 12.0;
    let (pm, _) = timestamp.hour12();
    let minute = ts / 60.0 % 60.0;
    let second = ts % 60.0;
    draw_clock(painter, center, radius, hour, pm, minute, second, accent_color);
}

pub(crate) fn draw_clock(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    hour: f32,
    pm: bool,
    minute: f32,
    second: f32,
    accent_color: Color32,
) {
    draw_clock_hand(painter, center, radius, HandPosition::Hour(hour, pm));
    draw_clock_hand(painter, center, radius, HandPosition::Minute(minute));
    draw_clock_hand(painter, center, radius, HandPosition::Second(second));
    for i in 1..=12 {
        draw_clock_text(painter, center, radius, i, accent_color);
    }
}

pub(crate) enum HandPosition {
    Hour(f32, bool),
    Minute(f32),
    Second(f32),
}

impl HandPosition {
    fn arc(&self) -> (f32, f32) {
        let length = match self {
            Self::Hour(hour, _) => (PI / 6.0) * hour,
            Self::Minute(minute) => (PI / 30.0) * minute,
            Self::Second(second) => (PI / 30.0) * second,
        };
        ((PI / 2.0) - length, length)
    }

    fn color(&self) -> Color32 {
        match self {
            // Self::Hour(_, pm) => if *pm { Color32::DARK_BLUE } else { Color32::BLUE },
            // Self::Minute(_) => Color32::from_rgb(72, 209, 204),
            // Self::Second(_) => Color32::GREEN,
            Self::Hour(_, pm) => if *pm { Color32::DARK_RED } else { Color32::RED },
            Self::Minute(_) => Color32::from_rgb(255, 140, 0),
            Self::Second(_) => Color32::GOLD,
        }
    }

    fn radius(&self, scale: f32) -> f32 {
        scale * match self {
            Self::Hour(_, _) => 1.0,
            Self::Minute(_) => 0.75,
            Self::Second(_) => 0.5,
        }
    }
}

pub(crate) fn draw_clock_hand(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    hand_position: HandPosition
) {
    let (start, arc) = hand_position.arc();
    let radius = hand_position.radius(radius);
    let color = hand_position.color();
    draw_arc(painter, center, radius, start, arc, color);
}

fn draw_arc(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    start: f32,
    arc: f32,
    color: Color32,
) {
    let segments = calc_segments(radius, arc);
    let f_step = |step: i32| arc_fn(center, radius, start, arc)(
        (step as f32) / (segments as f32)
    );
    let arc_path = |range: RangeInclusive<i32>| PathShape::convex_polygon(
        once(center)
            .chain(range.map(f_step))
            .collect(),
        color,
        Stroke::none(),
    );
    if arc > (3.0 * PI / 4.0) {
        painter.add(arc_path(0..=segments / 2));
        painter.add(arc_path(segments / 2..=segments));
        painter.line_segment([center, f_step(segments / 2)], (0.75, color));
    } else {
        painter.add(arc_path(0..=segments));
    }
}

fn calc_segments(radius: f32, arc: f32) -> i32 {
    (radius * arc / RESOLUTION / 2.0).ceil() as i32 * 2
}

fn arc_fn(center: Pos2, radius: f32, start: f32, arc: f32) -> impl Fn(f32) -> Pos2 {
    move |t: f32| center + vec2(
        (t * arc + start).cos() * radius,
        -(t * arc + start).sin() * radius,
    )
}

fn draw_clock_text(painter: &Painter, center: Pos2, radius: f32, number: u32, color: Color32) {
    let theta = (number as f32) * (PI / 6.0) - (PI / 2.0);
    let dash = |c1: f32, c2: f32| painter.line_segment(
        [radius_to_pos(center, radius * c1, theta), radius_to_pos(center, radius * c2, theta)],
        (1.0, color),
    );
    dash(0.475, 0.525);
    dash(0.725, 0.775);
    dash(0.975, 1.025);
    painter.text(
        radius_to_pos(center, radius * 1.130, theta),
        Align2::CENTER_CENTER,
        number.to_string(),
        FontId::new(radius / 8.0, FontFamily::default()),
        color,
    );
}

fn radius_to_pos(center: Pos2, radius: f32, theta: f32) -> Pos2 {
    center + vec2(radius * theta.cos(), radius * theta.sin())
}
