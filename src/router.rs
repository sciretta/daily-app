use crate::modules::shared::components::{Container,Drawer};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/task")]
    Task,
    #[at("/stats")]
    Stats,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
          <Container>
          <Drawer />
          <div >
          {"home"}</div>
         </Container>
        },Route::Task => html! { <Container>
            <Drawer />
            {"tasks"}
           </Container> },
        Route::Stats =>html! { <Container>
            <Drawer />
            {"stats"}
           </Container> },
        _ => html! { <h1>{ "404" }</h1> },
    }
  }