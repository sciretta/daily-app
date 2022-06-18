use yew::prelude::*;


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

#[function_component(Drawer)]
pub fn drawer() -> Html {
    html!{
        <aside class="mdc-drawer">
        <div class="mdc-drawer__header">
          <h3 class="mdc-drawer__title">{"Daily-App"}</h3>
        </div>
        <div class="mdc-drawer__content">
          <nav class="mdc-list">
            <a class="mdc-list-item mdc-list-item--activated" href="/" aria-current="page">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"home"}</i>
              <span class="mdc-list-item__text">{"Home"}</span>
            </a>
            <a class="mdc-list-item" href="/task">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"task"}</i>
              <span class="mdc-list-item__text">{"Tasks"}</span>
            </a>
            <a class="mdc-list-item" href="/stats">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"query_stats"}</i>
              <span class="mdc-list-item__text">{"Stats"}</span>
            </a>
          </nav>
        </div>
      </aside>
    }
}