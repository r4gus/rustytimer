use yew::prelude::*;

pub struct Clock {
    link: ComponentLink<Self>,
    viewbox: (u64, u64, u64, u64),
    height: u64,
    width: u64,
    stroke_width: u64,
    radius: u64,
    position: (u64, u64),
    progress: f64,
    circumference: f64,
    text: String,
    color: &'static str,
    darken: bool,   // tells if the text color should be dark to highlight a difference between states
}


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub progress: f64,
    pub text: String,
    pub darken: bool,
}

impl Component for Clock {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let width = 500;
        let height = 500;
        let stroke_width = 21;
        let radius = (width / 2) - (stroke_width * 2);

        Self {
            link,
            viewbox: (0, 0, width, height),
            height: height,
            width: width,
            stroke_width: stroke_width,
            radius: radius,
            position: (width / 2, height / 2),
            progress: props.progress,
            circumference: radius as f64 * 2.0 * std::f64::consts::PI,
            text: props.text,
            color: "#39c9bb",
            darken: props.darken,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.progress = props.progress;
        self.text = props.text;
        self.darken = props.darken;
        true
    }

    fn view(&self) -> Html {
        let style = format!("stroke-dasharray: {} {}; stroke-dashoffset: {};
            transition: stroke-dashoffset 0.35s; transform: rotate(-90deg); transform-origin: 50% 50%;",
            self.circumference, self.circumference, self.circumference - self.progress * self.circumference);
        html! {
            <svg
                class="progress-ring"
                viewBox={format!("{} {} {} {}", self.viewbox.0, self.viewbox.1, self.viewbox.2, self.viewbox.3)}
            >
                <circle
                    class="progress-ring__background"
                    stroke-width={ self.stroke_width }
                    stroke="#fff"
                    fill="none"
                    r={ self.radius }
                    cx={ self.position.0 }
                    cy={ self.position.1 }
                />
                <circle
                    class="progress-ring__circle"
                    stroke-width={ self.stroke_width }
                    stroke={ self.color }
                    fill="none"
                    r={ self.radius }
                    cx={ self.position.0 }
                    cy={ self.position.1 }
                    style={ style }
                />
                <text
                    x={ self.position.0 }
                    y={ self.position.1 }
                    text-anchor="middle"
                    font-size="6em"
                    fill={ if self.darken { "#808080" } else { "#ffffff" } }
                    dominant-baseline="middle"
                >
                    { &self.text }
                </text>
            </svg>
        }
    }
}