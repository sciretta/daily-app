use yew::prelude::*;
use gloo;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &Props)  -> Html {
    html!{
       <body>
       {props.children.clone()}
       </body>
    }
}

fn get_drawer_item_class(route_name:&str) -> String {
  let uri = match gloo::utils::window().location().pathname()  {
    Ok(uri) => uri,
    Err(error) => format!("Error getting browser URI: {:?}", error),
  };

  if !uri.contains(route_name) {
    String::from("mdc-list-item")
    
  }else {
    String::from("mdc-list-item mdc-list-item--activated")
  }

}

#[function_component(Drawer)]
pub fn drawer() -> Html {
    html!{
        <aside class="mdc-drawer">
        <div class="mdc-drawer__header">
          <h3 class="mdc-drawer__title">{"Daily-App"}</h3>
        </div>
        <div class="mdc-drawer__content">
          <nav class="mdc-list">
            <a class={get_drawer_item_class("home")} href="/home" aria-current="page">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"home"}</i>
              <span class="mdc-list-item__text">{"Home"}</span>
            </a>
            <a class={get_drawer_item_class("task")} href="/task">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"task"}</i>
              <span class="mdc-list-item__text">{"Tasks"}</span>
            </a>
            <a class={get_drawer_item_class("stats")} href="/stats">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"query_stats"}</i>
              <span class="mdc-list-item__text">{"Stats"}</span>
            </a>
          </nav>
        </div>
      </aside>
    }
}