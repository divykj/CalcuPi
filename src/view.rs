use crate::{
    app::{Model, Msg},
    canvas::CANVAS_SIZE,
    helpers::calculate_pi,
};
use num_format::{Locale, ToFormattedString};
use seed::{
    a, attrs, button, canvas, caption, div, h1, h3, h5, header, i, id, main, nodes,
    prelude::{el_ref, ev, px, At, El, Ev, IndexMap, Node, ToClasses, UpdateEl},
    span, C,
};

macro_rules! icon {
    ($icon:literal, $($rest:expr),*) => {
        i![C![$icon], $($rest),*]
    };
    ($icon:literal) => {
        icon![$icon,]
    };
}

macro_rules! icon_button {
    ($icon:literal, $text:literal, $title:literal, $on_click:expr, $($rest:expr),*) => {{
        button![icon![$icon], span![$text], attrs![At::Title => $title], ev(Ev::Click, $on_click), $($rest),*]
    }};
    ($icon:literal, $text:literal, $title:literal, $on_click:expr) => {
        icon_button!($icon, $text, $title, $on_click,)
    };
    ($icon:literal, $title:literal, $on_click:expr, $($rest:expr),*) => {
        button![icon![$icon], C!["icon-only"], attrs![At::Title => $title], ev(Ev::Click, $on_click), $($rest),*]
    };
    ($icon:literal, $title:literal, $on_click:expr) => {
        icon_button!($icon, $title, $on_click,)
    };
}

macro_rules! simulation_speed_button {
    ($speed:literal, $title:literal, $simulation_speed:ident) => {
        icon_button![
            "fas fa-times",
            $speed,
            $title,
            |_| Msg::SetSimulationSpeed($speed),
            C!["small", ($simulation_speed == $speed).then(|| "primary")]
        ]
    };
}

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        main![
            view_header(),
            view_controls(
                model.simulation_timer_handle.is_some(),
                model.simulation_speed
            ),
            view_results(model.total_points, model.points_in_circle),
            div![C!["spacer"]],
            view_footer(model.prefers_dark_mode)
        ],
        div![
            id!["visualization"],
            canvas![
                el_ref(&model.canvas),
                attrs![
                    At::Width => px(CANVAS_SIZE),
                    At::Height => px(CANVAS_SIZE),
                ],
            ],
        ]
    ]
}

fn view_header() -> Node<Msg> {
    header![h1!["CalcuPi"], h3!["Monte Carlo Method"]]
}

fn view_controls(is_playing: bool, simulation_speed: usize) -> Node<Msg> {
    div![
        id!["controls"],
        h5!["Run Simulation"],
        div![
            C!["horizontal-group"],
            if is_playing {
                icon_button![
                    "fas fa-pause",
                    "Pause",
                    "Pause Simulation",
                    |_| Msg::StopSimulation,
                    C!["primary"]
                ]
            } else {
                icon_button![
                    "fas fa-play",
                    "Play",
                    "Play Simulation",
                    |_| Msg::StartSimulation,
                    C!["primary"]
                ]
            },
            icon_button![
                "fas fa-redo-alt",
                "Reset",
                "Reset Simulation",
                |_| Msg::Reset,
                C!["secondary"]
            ],
        ],
        h5!["Simulation Speed"],
        div![
            C!["horizontal-group"],
            simulation_speed_button!(1, "1x speed", simulation_speed),
            simulation_speed_button!(10, "10x speed", simulation_speed),
            simulation_speed_button!(100, "100x speed", simulation_speed),
        ],
        div![
            C!["horizontal-group"],
            simulation_speed_button!(1000, "1000x speed", simulation_speed),
            simulation_speed_button!(10000, "10000x speed", simulation_speed),
        ],
    ]
}

fn view_results(total_points: usize, points_in_circle: usize) -> Node<Msg> {
    let pi = calculate_pi(total_points, points_in_circle);

    div![
        id!["results"],
        h5!["Results"],
        div![
            C!["horizontal-group"],
            div![
                C!["infobox"],
                caption!["Total Points"],
                div![C!["spacer"]],
                div![span!["S = "], total_points.to_formatted_string(&Locale::en)]
            ],
            div![
                C!["infobox"],
                caption!["Inside Circle"],
                div![C!["spacer"]],
                div![
                    span!["C = "],
                    points_in_circle.to_formatted_string(&Locale::en)
                ]
            ],
        ],
        div![
            C!["horizontal-group"],
            div![
                C!["infobox"],
                caption!["Calculated Pi"],
                match pi {
                    Some(pi) => div![
                        span!["4*C/S = "],
                        format!("{:.5}", pi,),
                        span![format!("({:+.5})", std::f64::consts::PI - pi)]
                    ],
                    None => div!["Not calculated"],
                }
            ],
        ],
    ]
}

fn view_footer(prefers_dark_mode: bool) -> Node<Msg> {
    div![
        id!["footer-actions"],
        if prefers_dark_mode {
            icon_button!["fas fa-sun", "Light Mode", |_| Msg::ToggleDarkMode,]
        } else {
            icon_button!["fas fa-moon", "Dark Mode", |_| Msg::ToggleDarkMode,]
        },
        a![
            C!["button", "icon-only"],
            attrs![At::Title => "View Source Code", At::Href => "https://github.com/divykj/CalcuPi/"],
            icon!["fas fa-code"]
        ],
        a![
            C!["button", "icon-only"],
            attrs![At::Title => "Divya Jain - Github", At::Href => "https://github.com/divykj"],
            div![id!["profile-photo"]],
        ],
    ]
}
