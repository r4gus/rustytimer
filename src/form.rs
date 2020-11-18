use yew::prelude::*;
use super::helper::*;

pub struct Form {
    link: ComponentLink<Self>,
    on: u64,
    off: u64,
    cycles: u64,
    callback: Callback<(u64, u64, u64)>,
    text: &'static str,
}

pub enum Msg {
    UpdateOnH(String),
    UpdateOnM(String),
    UpdateOnS(String),
    UpdateOffH(String),
    UpdateOffM(String),
    UpdateOffS(String),
    UpdateCycles(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub callback: Callback<(u64, u64, u64)>,
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            on: 20,
            off: 10,
            cycles: 8,
            callback: props.callback,
            text: "",
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateOnH(hou) => {
                let res = hou.parse::<u64>();

                match res {
                    Ok(h) => {
                        let mut temp = self.on % 3600; // strip hours
                        temp += h * 3600;
                        self.on = temp;
                    },
                    Err(_) => {},
                }
            }
            Msg::UpdateOnM(min) => {
                let res = min.parse::<u64>();

                match res {
                    Ok(m) => {
                        let mut temp = self.on - minutes(self.on) * 60; // strip minutes
                        temp += m * 60;
                        self.on = temp;
                    },
                    Err(_) => {},
                }
            }
            Msg::UpdateOnS(sec) => {
                let res = sec.parse::<u64>();

                match res {
                    Ok(s) => {
                        let mut temp = self.on - seconds(self.on); // strip seconds
                        temp += s;
                        self.on = temp;
                    },
                    Err(_) => {},
                }
            }
            Msg::UpdateOffH(hou) => {
                let res = hou.parse::<u64>();

                match res {
                    Ok(h) => {
                        let mut temp = self.off % 3600; // strip hours
                        temp += h * 3600;
                        self.off = temp;
                    },
                    Err(_) => {},
                }
            }
            Msg::UpdateOffM(min) => {
                let res = min.parse::<u64>();

                match res {
                    Ok(m) => {
                        let mut temp = self.off - minutes(self.off) * 60; // strip minutes
                        temp += m * 60;
                        self.off = temp;
                    },
                    Err(_) => {},
                }
            }
            Msg::UpdateOffS(sec) => {
                let res = sec.parse::<u64>();

                match res {
                    Ok(s) => {
                        let mut temp = self.off - seconds(self.off); // strip seconds
                        temp += s;
                        self.off = temp;
                    },
                    Err(_) => {},
                }
            }
            Msg::UpdateCycles(cyc) => {
                let res = cyc.parse::<u64>();

                match res {
                    Ok(c) => {
                        self.cycles = c;
                    },
                    Err(_) => {},
                }
            }
        }

        self.callback.emit((self.on, self.off, self.cycles));
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.callback = props.callback;
        true
    }

    fn view(&self) -> Html {
        html! {
            <form>
                <div class="form-row">
                    <div class="col-sm-4">
                        <h3 class="center"><strong>{"On Time"}</strong></h3>
                        <label for="onHour">{ format!("Hours: {}", hours(self.on)) }</label>
                        <input type="range" min="0" max="23", value={ hours(self.on) } class="custom-range" id="onHour"
                            oninput={ self.link.callback(|e: InputData| Msg::UpdateOnH(e.value)) }
                        />
                        <label for="onMinute">{ format!("Minutes: {}", minutes(self.on)) }</label>
                        <input type="range" min="0" max="59", value={ minutes(self.on) } class="custom-range" id="onMinute"
                            oninput={ self.link.callback(|e: InputData| Msg::UpdateOnM(e.value)) }
                        />
                        <label for="onSecond">{ format!("Seconds: {}", seconds(self.on)) }</label>
                        <input type="range" min="0" max="59", value={ seconds(self.on) } class="custom-range" id="onSecond"
                            oninput={ self.link.callback(|e: InputData| Msg::UpdateOnS(e.value)) }
                        />
                    </div>
                    <div class="col-sm-4">
                        <h3 class="center"><strong>{"Off Time"}</strong></h3>
                        <label for="offHour">{ format!("Hours: {}", hours(self.off)) }</label>
                        <input type="range" min="0" max="23", value={ hours(self.off) } class="custom-range" id="offHour"
                            oninput={ self.link.callback(|e: InputData| Msg::UpdateOffH(e.value)) }
                        />
                        <label for="offMinute">{ format!("Minutes: {}", minutes(self.off)) }</label>
                        <input type="range" min="0" max="59", value={ minutes(self.off) } class="custom-range" id="offMinute"
                            oninput={ self.link.callback(|e: InputData| Msg::UpdateOffM(e.value)) }
                        />
                        <label for="offSecond">{ format!("Seconds: {}", seconds(self.off)) }</label>
                        <input type="range" min="0" max="59", value={ seconds(self.off) } class="custom-range" id="offSecond"
                            oninput={ self.link.callback(|e: InputData| Msg::UpdateOffS(e.value)) }
                        />
                    </div>
                    <div class="col-sm-4">
                        <h3 class="center"><strong>{"Cycles"}</strong></h3>
                        <label for="cycles">{ format!("{}", self.cycles) }</label>
                        <input type="range" min="1" max="100", value={ self.cycles } class="custom-range" id="cycles"
                            oninput={ self.link.callback(|e: InputData| Msg::UpdateCycles(e.value)) }
                        />
                    </div>
                </div>
            </form>
        }
    }
}
