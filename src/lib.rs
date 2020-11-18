#![recursion_limit="1024"] // limit the recursion depth of the html! macro
mod helper;
mod clock;
mod form;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{Task, IntervalService, ConsoleService};

use helper::{hours, minutes, seconds};
use clock::Clock;
use form::Form;
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
    callback_form: Callback<(u64, u64, u64)>,
    message: &'static str,
    state: State,       // the current state of the timer
    saved_state: State, // Used to save the state if the timer is paused.
    job: Option<Box<dyn Task>>, // Currently active task
}

/// Messages the `Timer` can handle.
///
/// # Messages
///
/// * `StartTimer` - Starts the timer.
/// * `StopTimer` - Stops the timer (state is preserved).
/// * `ResetTimer` - Resets everything to the currently selected settings.
/// * `SetTimer` - Set a new On and Off duration as well as a new number of cycles to complete.
/// * `Tick` - Frequently called (each second) by an `IntervalService` if the timer is active (`On`, `Off`).
enum Msg {
    StartTimer,
    StopTimer,
    ResetTimer,
    SetTimer(u64, u64, u64),
    Tick,
}

/// The different states of the `Timer`.
///
/// The `Timer` starts in `Idle` state. If the user clicks the start button the `Timer` switches
/// to the `Start` state and a countdown appears after which it transitions to the `On` state.
/// The `Timer` toggles between `On` and `Off` until either all cycles are completed or the user
/// presses the pause button. In `Pause` state the timer can either be resumed or reset.
///
/// # States
///
/// * `Start` - Start/ Resume the timer.
/// * `On` - The state in which the user is demanded to work out.
/// * `Off` - The state in which the user is granted some rest.
/// * `Paused` - The timer is paused.
/// * `Idle` - Do nothing.
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
    type Properties = (); // Root node so we have no properties

    /// Create a new `Timer` component.
    ///
    /// # Arguments
    ///
    /// * `_props` - Properties from the parent component (currently none - it's the root).
    /// * `link` - A link to register callbacks or send messages to the component.
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            callback_tick: link.callback(|_| Msg::Tick), // register new `Tick` callback.
            callback_form: link.callback(|tup: (u64, u64, u64)| Msg::SetTimer(tup.0, tup.1, tup.2)),
            link,
            duration_on: 20,
            duration_off: 10,
            cycles: 8,
            start: 5,
            counter_s: 20,
            counter_c: 0,
            message: "",
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
            // Called when the timer is started or resumed.
            Msg::StartTimer => {
                // Create an new `IntervalService` instance that calls `Tick` every second.
                let handle = IntervalService::spawn(Duration::from_secs(1), self.callback_tick.clone());
                self.job = Some(Box::new(handle));

                match self.state {
                    State::Idle => { // Start timer
                        self.counter_s = self.duration_on;
                        self.counter_c = 0;
                        //self.message = "Timer started";
                        self.state = State::Start;
                    },
                    _ => { // Resume timer
                        //self.message = "Timer resumed";
                        self.state = self.saved_state;
                    }
                }

            },
            Msg::StopTimer => { // Pause the timer (state is preserved until it is started again or reset)
                //self.message = "Timer stoped";
                self.saved_state = self.state;  // Save current state
                self.state = State::Paused;             // Switch timer into pause state
                self.job = None;                        // Remove the current interval service that calls tick
            },
            Msg::ResetTimer => { // Reset the timer state
                self.counter_s = self.duration_on;
                self.counter_c = 0;
                self.start = 5;
                self.state = State::Idle;
                //self.message = "Reset";
                self.job = None;
            },
            Msg::SetTimer(on, off, rounds) => {
                self.duration_on = on;
                self.duration_off = off;
                self.cycles = rounds;
                self.link.callback(|_| Msg::ResetTimer).emit(());
            },
            Msg::Tick => { // Called every second to update the timer state
                match self.state {
                    State::Start => { // The timer has just bee started and we're counting down.

                        if self.start == 0 { // Countdown finished, switch to `On` state.
                            self.start = 5;
                            self.state = State::On;
                        } else {
                            self.start -= 1;
                        }

                        if self.start == 0 { // Play countdown sound.
                            play_countdown("long-beep", "long-beep-player");
                        } else if self.start <= 4 { // Play countdown sound.
                            play_countdown("beep", "beep-player");
                        }
                    },
                    _ => {


                        if self.counter_s == 0 { // Counted down
                            match self.state {
                                State::On => self.counter_c += 1, // `On` - `Off` cycle completed.
                                _ => {},
                            }

                            if self.counter_c < self.cycles { // Not all cycles are completed.
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
                            } else { // All cycles completed, Nice Job !
                                self.state = State::Idle;
                                //self.message = "Done, nice work!";
                                self.job = None;
                            }
                        } else {
                            self.counter_s -= 1; // Decrement counter on every tick.
                        }

                        if self.counter_s == 0 { // Play countdown sound.
                            play_countdown("long-beep", "long-beep-player");
                        } else if self.counter_s <= 4 { // Play countdown sound.
                            play_countdown("beep", "beep-player");
                        }
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
            <>
                <div class="cover-container d-flex w-100 h-100 p-3 mx-auto flex-column">
                  <header class="masthead mb-auto">
                    <div class="inner">
                      <h3 class="masthead-brand">{ "RustyTimer" }</h3>
                      <nav class="nav nav-masthead justify-content-center">
                        <a class="nav-link" href="#" data-toggle="modal" data-target="#settingsModal">{ "Settings" }</a>
                        <a class="nav-link" href="#" data-toggle="modal" data-target="#infoModal">{ "Info" }</a>
                      </nav>
                    </div>
                  </header>

                  <main role="main" class="inner cover">
                    <p class="lead">{ self.message }</p>
                    <div class="clock-container">
                        <Clock progress={ self.counter_c as f64 / self.cycles as f64 }
                               text={ if self.state == State::Start {
                                        format!("{}", self.start)
                                      } else {
                                        format!("{:02}:{:02}:{:02}", hours(self.counter_s), minutes(self.counter_s), seconds(self.counter_s))
                                      }}
                               darken={self.state == State::Off}
                               color="#39c9bb"
                        />
                    </div>

                    {
                        match self.state {
                            State::Idle => html! { <button type="button" class="btn btn-outline-info btn-lg" onclick=self.link.callback(|_| Msg::StartTimer)>{ "Start" }</button> },
                            State::Paused => html! { <><button type="button" class="btn btn-outline-info btn-lg mr-3" onclick=self.link.callback(|_| Msg::StartTimer)>{ "Resume" }</button>
                                                     <button type="button" class="btn btn-outline-warning btn-lg" onclick=self.link.callback(|_| Msg::ResetTimer)>{ "Reset" }</button></>},
                            State::Start => html! { },
                            _ => html! { <button type="button" class="btn btn-outline-secondary btn-lg" onclick=self.link.callback(|_| Msg::StopTimer)>{ "Stop" }</button> },
                        }
                    }


                  </main>

                  <footer class="mastfoot mt-auto">
                    <div class="inner">
                        <a href="https://ko-fi.com/sug4r" target="_blank">
                            <img src="images/BuyMeACoffee.png" alt="Buy Me a Coffee!" style="width: 180px;" />
                        </a>
                        <div style="padding-top: 6px;">{"Copyright (c) 2020 David Sugar"}</div>
                    </div>
                  </footer>

                  <audio id="beep">
                    <source id="beep-player" src="sounds/beep.mp3" type="audio/mp3"/>
                  </audio>
                  <audio id="long-beep">
                    <source id="long-beep-player" src="sounds/long-beep.mp3" type="audio/mp3"/>
                  </audio>
                </div>

                <div class="modal fade" id="settingsModal" tabindex="-1" role="dialog" aria-labelledby="settingsModalLabel" aria-hidden="true">
                    <div class="modal-dialog" role="document">
                        <div class="modal-content">
                            <div class="modal-header bg-info">
                                <h4 class="modal-title" id="settingsModalLabel">{ "Settings" }</h4>
                                <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                                    <i class="fa fa-times" aria-hidden="true" style="color: #fff;"></i>
                                </button>
                            </div>
                            <div class="modal-body text-dark" id="settingsModalBody">
                                <Form callback={ self.callback_form.clone() } />
                            </div>
                        </div>
                    </div>
                </div>

                <div class="modal fade" id="infoModal" tabindex="-1" role="dialog" aria-labelledby="infoModalLabel" aria-hidden="true">
                    <div class="modal-dialog" role="document">
                        <div class="modal-content">
                            <div class="modal-header bg-info">
                                <h4 class="modal-title">{ "Info" }</h4>
                                <button type="button" class="close" data-dismiss="modal" aria-label="Close">
                                    <i class="fa fa-times" aria-hidden="true" style="color: #fff;"></i>
                                </button>
                            </div>
                            <div class="modal-body text-dark" id="infoModalBody">
                            <p>
                                {"I believe in free software that benefits people. I don't store any personal data \
                                    nor do I wanna show you advertising. If you want to support me feel free and "}
                                    <a href="https://ko-fi.com/sug4r" target="_blank">
                                        <img src="images/BuyMeACoffee.png" alt="Buy Me a Coffee!" style="width: 120px;" />
                                    </a>
                                    {"."}<br/><br/>{"This site is licensed under "} <a href="https://github.com/r4gus/rustytimer/blob/main/LICENSE" style="color: black;" target="_blank"><strong>{"MIT "} </strong></a>
                                    {"and uses "}<a href="https://getbootstrap.com/" style="color: black;" target="_blank"><strong>{"Bootstrap "}</strong></a>
                                    {"and "}<a href="https://fontawesome.com/v4.7.0/" style="color: black;" target="_blank"><strong> {"FontAwesome "}</strong></a> {"for it's layout. \
                                    It is written in "} <a href="https://www.rust-lang.org/" style="color: black;" target="_blank"><strong>{"Rust "}</strong></a> {"using the "}
                                    <a href="https://yew.rs/docs/en/" style="color: black;" target="_blank"><strong>{"Yew "}</strong></a> {"framework."}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn play_countdown(aid: &str, sid: &str);
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Timer>::new().mount_to_body();
}