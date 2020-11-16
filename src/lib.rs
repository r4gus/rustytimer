#![recursion_limit="1024"] // limit the recursion depth of the html! macro
mod helper;
mod clock;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{Task, IntervalService, ConsoleService};

use helper::{hours, minutes, seconds};
use clock::Clock;
use wasm_bindgen::__rt::core::time::Duration;


/// This represents the upper layer of an interval timer.
///
/// A interval timer (also known as ta-ba-ta timer) loops between `on` and `off` state until all
/// cycles of a training are completed.
struct Timer {
    link: ComponentLink<Self>,
    duration_on: u64,   // duration of each cycle in seconds
    duration_off: u64,  // duration of each pause in seconds
    cycles: u64,        // total number of rounds
    start: u64,         // seconds until the timer starts
    counter_s: u64,
    counter_c: u64,
    callback_tick: Callback<()>, // callback to be invoked on a `tick`
    message: &'static str,
    state: State,       // the current state of the timer
    saved_state: State,
    job: Option<Box<dyn Task>>, // Currently active task
}

/// Messages the `Timer` can handle.
///
/// # Messages
///
/// * `StartTimer` - Starts the timer.
/// * `StopTimer` - Stops the timer (state is preserved).
/// * `ResetTimer` - Resets everything to the currently selected settings.
enum Msg {
    StartTimer,
    StopTimer,
    ResetTimer,
    SetTimer,
    Tick,
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    Start,
    On,
    Off,
    Paused,
    Idle,
}

impl Component for Timer {
    type Message = Msg;
    type Properties = ();

    /// Create a new tiemr component.
    ///
    /// # Arguments
    ///
    /// * `_props` - Properties from the paren (currently none).
    /// * `link` - A link to register callbacks or send messages to the component.
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            callback_tick: link.callback(|_| Msg::Tick),
            link,
            duration_on: 20,
            duration_off: 10,
            cycles: 8,
            start: 5,
            counter_s: 20,
            counter_c: 0,
            message: "Get ready!",
            state: State::Idle,
            saved_state: State::Idle,
            job: None,
        }
    }

    /// Handle incomming messages.
    ///
    /// The `update()` lifecycle method is called for each message.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to handle.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartTimer => {
                let handle = IntervalService::spawn(Duration::from_secs(1), self.callback_tick.clone());
                self.job = Some(Box::new(handle));

                match self.state {
                    State::Idle => { // Start timer
                        self.counter_s = self.duration_on;
                        self.counter_c = 0;
                        self.message = "Timer started";
                        self.state = State::On;

                    },
                    _ => { // Resume timer
                        self.message = "Timer resumed";
                        self.state = self.saved_state;
                    }
                }

            },
            Msg::StopTimer => { // Pause the timer (state is preserved until it is started again or reset)
                self.message = "Timer stoped";
                self.saved_state = self.state;  // Save current state
                self.state = State::Paused;             // Switch timer into pause state
                self.job = None;                        // Remove the current interval service that calls tick
            },
            Msg::ResetTimer => { // Reset the timer state
                self.counter_s = self.duration_on;
                self.counter_c = 0;
                self.state = State::Idle;
                self.message = "Reset";
                self.job = None;
            },
            Msg::SetTimer => {},
            Msg::Tick => { // Called every second to update the timer state
                self.counter_s -= 1;

                if self.counter_s == 0 {
                    match self.state {
                        State::On => self.counter_c += 1,
                        _ => {},
                    }

                    if self.counter_c < self.cycles {
                        match self.state {
                            State::On => {
                                self.state = State::Off;
                                self.counter_s = self.duration_off;
                            },
                            State::Off => {
                                self.state = State::On;
                                self.counter_s = self.duration_on;
                            },
                            _ => {}, // Should be impossible
                        }
                    } else {
                        self.state = State::Idle;
                        self.message = "Done, nice work!";
                        self.job = None;
                    }
                }
            },
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    /// Create a (html) layout for the component.
    fn view(&self) -> Html {
        html! {
            <div class="cover-container d-flex w-100 h-100 p-3 mx-auto flex-column">
              <header class="masthead mb-auto">
                <div class="inner">
                  <h3 class="masthead-brand">{ "RustyTimer" }</h3>
                  <nav class="nav nav-masthead justify-content-center">
                    <a class="nav-link active" href="#">{ "Contact" }</a>
                  </nav>
                </div>
              </header>

              <main role="main" class="inner cover">
                <p class="lead">{ self.message }</p>
                <div class="clock-container">
                    <Clock progress={ self.counter_c as f64 / self.cycles as f64 } text=format!("{:02}:{:02}:{:02}", hours(self.counter_s), minutes(self.counter_s), seconds(self.counter_s))
                        darken={self.state == State::Off}/>
                </div>

                {
                    match self.state {
                        State::Idle => html! { <button type="button" class="btn btn-outline-primary" onclick=self.link.callback(|_| Msg::StartTimer)>{ "Start" }</button> },
                        State::Paused => html! { <button type="button" class="btn btn-outline-primary" onclick=self.link.callback(|_| Msg::StartTimer)>{ "Resume" }</button> },
                        _ => html! { <button type="button" class="btn btn-outline-secondary" onclick=self.link.callback(|_| Msg::StopTimer)>{ "Stop" }</button> },
                    }
                }

                <button type="button" class="btn btn-outline-warning" onclick=self.link.callback(|_| Msg::ResetTimer)>{ "Reset" }</button>
              </main>

              <footer class="mastfoot mt-auto">
                <div class="inner">
                  <p>{ "Developed by David Sugar" }</p>
                </div>
              </footer>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Timer>::new().mount_to_body();
}