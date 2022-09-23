use super::connector;

use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use wasm_timer::Delay;
use yew::{html, Component, Html};
use yew::Context;

pub struct Model {
    tasks: Vec<Task>,
}

pub struct Task {
    pub status: i32,
    pub fav: bool,
    pub info: Option<connector::Payload>,
}

pub enum TaskMsg {
    Fetchit(connector::Payload),
    ToggleFav,
}

impl Component for Task {
    type Message = TaskMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let inst = Task {
            status: 0,
            fav: false,
            info: None::<connector::Payload>,
        };

        let link = ctx.link().clone();

        spawn_local(async move {
            loop {
                let info = connector::fetchit().await.unwrap();
                if let Some(info) = info {
                    //let task_msg = TaskMsg::Fetchit(info);
                    log::info!("quote.body: {:?}", info.quote.body.clone());
                    link.send_message(TaskMsg::Fetchit(info));
                }
                Delay::new(Duration::from_secs(5)).await.unwrap();
            }
        });
        inst
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Fetchit(info) => {
                self.info = Some(info);
                self.fav = false;
                true
            }
            Self::Message::ToggleFav => {
                self.fav = !self.fav;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="card mt-3">
                <div class="card-body">
                    <figure>
                        <blockquote class="blockquote">
                            if self.fav {
                                <span class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-danger">
                                    { "1+" }
                                    <span class="visually-hidden">{ "Number of likes" }</span>
                                </span>
                            }
                            <p>
                                { self.info.as_ref().map(|val| val.quote.body.clone() ).unwrap_or("".to_string())}
                            </p>
                            <a class="btn btn-success position-absolute bottom-0 end-0 me-2 mb-2" href="#" onclick={ctx.link().callback(|_| TaskMsg::ToggleFav)}>
                                { "Like" }
                            </a>
                        </blockquote>
                        <figcaption class="blockquote-footer">
                            <cite title="Source Title">
                                { self.info.as_ref().map(|val| val.clone().quote.author).unwrap_or("fetching --->".to_string())}
                            </cite>
                        </figcaption>
                    </figure>
                </div>
            </div>
        }
    }
}

pub enum Msg {
    DoIt,
}

impl Component for Model {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Model { tasks: Vec::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DoIt => {
                let task = Task {
                    fav: false,
                    info: None::<connector::Payload>,
                    status: 0,
                };
                self.tasks.push(task);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <a class="btn btn-warning" href="#" onclick={ctx.link().callback(|_| Msg::DoIt)}>
                    { "fetch quotes of the day" }
                </a>

                { for self.tasks.iter().map(|_tsk| html! {
                    <Task />
                })}
            </>
        }
    }
}
