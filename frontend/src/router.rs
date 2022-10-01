use crate::modules::shared::components::{Container,Drawer};
use crate::modules::home::index::Home;
use crate::modules::task::index::Task;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/home")]
    Home,
    #[at("/task/:id")]
    Task { id:String},
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
          <Home />
         </Container>
        },Route::Task {id}=> html! { 
          <Container>
            <Drawer />
            <Task id={id.clone()} />
           </Container> },
        Route::Stats =>html! {
           <Container>
             <Drawer />
            {"stats"}
           </Container> },
        _ => html! { <h1>{ "404" }</h1> },
    }
  }