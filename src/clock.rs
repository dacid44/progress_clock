use std::f32::consts::PI;
use std::iter::once;
use chrono::{NaiveTime, Timelike};
use egui::{Color32, Painter, Pos2, Stroke, vec2};
use egui::epaint::PathShape;

const RESOLUTION: i32 = 50;

pub(crate) fn draw_clock_timestamp(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    timestamp: NaiveTime
) {
    let ts = (timestamp.num_seconds_from_midnight() as f32) +
        ((timestamp.nanosecond() as f32) / 1_000_000_000.0);
    let hour = ts / 3600.0 % 12.0;
    let (pm, _) = timestamp.hour12();
    let minute = ts / 60.0 % 60.0;
    let second = ts % 60.0;
    draw_clock(painter, center, radius, hour, pm, minute, second);
}

pub(crate) fn draw_clock(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    hour: f32,
    pm: bool,
    minute: f32,
    second: f32
) {
    draw_clock_hand(painter, center, radius, HandPosition::Hour(hour, pm));
    draw_clock_hand(painter, center, radius, HandPosition::Minute(minute));
    draw_clock_hand(painter, center, radius, HandPosition::Second(second));
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
    draw_arc(painter, center, hand_position.radius(radius), start, arc, hand_position.color());
}

pub(crate) fn draw_arc(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    start: f32,
    arc: f32,
    color: Color32
) {
    draw_convex_arc(painter, center, radius, start, arc, color, ArcSegment::First);
    draw_convex_arc(painter, center, radius, start, arc, color, ArcSegment::Second);
}

enum ArcSegment {
    Full,
    First,
    Second,
}

fn draw_convex_arc(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    start: f32,
    arc: f32,
    color: Color32,
    segment: ArcSegment
) {
    let f_step = |step: i32| arc_fn(center, radius, start, arc)(
        (step as f32) / (RESOLUTION as f32)
    );
    let range = match segment {
        ArcSegment::Full => (0..=RESOLUTION),
        ArcSegment::First => (0..=RESOLUTION / 2),
        ArcSegment::Second => (RESOLUTION / 2..=RESOLUTION)
    };
    painter.add(PathShape::convex_polygon(
        once(center)
            .chain(range.map(f_step))
            .collect(),
        color,
        (1.0, color),
    ));
}

fn arc_fn(center: Pos2, radius: f32, start: f32, arc: f32) -> impl Fn(f32) -> Pos2 {
    move |t: f32| center + vec2(
        (t * arc + start).cos() * radius,
        -(t * arc + start).sin() * radius,
    )
}
