use yew::prelude::*;


#[derive(PartialEq, Properties)]
pub struct Props {
    pub children:Children
}


#[function_component(Container)]
fn container(props: &Props)  -> Html {
    html!{
       <div class={"container"}>
       {props.children.clone()}
       </div>
    }
}

#[function_component(Drawer)]
fn drawer() -> Html {
    html!{
        <aside class="mdc-drawer">
        <div class="mdc-drawer__header">
          <h3 class="mdc-drawer__title">{"Daily-App"}</h3>
        </div>
        <div class="mdc-drawer__content">
          <nav class="mdc-list">
            <a class="mdc-list-item mdc-list-item--activated" href="#" aria-current="page">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"inbox"}</i>
              <span class="mdc-list-item__text">{"Inbox"}</span>
            </a>
            <a class="mdc-list-item" href="#">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"send"}</i>
              <span class="mdc-list-item__text">{"Outgoing"}</span>
            </a>
            <a class="mdc-list-item" href="#">
              <span class="mdc-list-item__ripple"></span>
              <i class="material-icons mdc-list-item__graphic" aria-hidden="true">{"drafts"}</i>
              <span class="mdc-list-item__text">{"Drafts"}</span>
            </a>
          </nav>
        </div>
      </aside>
    }
}

#[function_component(App)]
fn app()->Html {
    html!{
      <Container>
        <Drawer />
       </Container>
    }
}

fn main() {
    yew::start_app::<App>(); 
}