use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo;

#[derive(PartialEq, Properties)]
pub struct PropsContainer {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &PropsContainer)  -> Html {
    html!{
       <>
       {props.children.clone()}
       </>
    }
}

fn get_drawer_item_class(route_name:&str) -> String {
  let uri = match gloo::utils::window().location().pathname()  {
    Ok(uri) => uri,
    Err(error) => format!("Error getting browser URI: {:?}", error),
  };

  if !uri.contains(route_name) {
    String::from("mdc-list-item p-6")
    
  }else { 
    String::from("mdc-list-item mdc-list-item--activated p-6")
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
            <a class={get_drawer_item_class("task")} href="/task/new">
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

#[derive(PartialEq, Properties)]
pub struct PropsCheckbox {
  #[prop_or(false)]
    pub checked: bool,
    pub on_check: Callback<bool>,
    #[prop_or(false)]
    pub disabled: bool
}

#[function_component(Checkbox)]
  pub  fn checkbox(props:&PropsCheckbox) ->Html {

    let onchange = {
      let props_onchange = props.on_check.clone();
      
      Callback::from(move |event: Event| {
          let value = event
              .target()
              .unwrap()
              .unchecked_into::<HtmlInputElement>()
              .checked();
          props_onchange.emit(value);
      })
  };
    
    html!{
        <div class="mdc-touch-target-wrapper">
            <div class="mdc-checkbox mdc-checkbox--touch">
            <input type="checkbox" 
            disabled={props.disabled}
            class="mdc-checkbox__native-control"
            checked={props.checked}
            {onchange}
            />
            <div class="mdc-checkbox__background">
            <svg class="mdc-checkbox__checkmark"
                viewBox="0 0 24 24">
                <path class="mdc-checkbox__checkmark-path"
                    fill="none"
                    d="M1.73,12.91 8.1,19.28 22.79,4.59"/>
            </svg>
            <div class="mdc-checkbox__mixedmark"></div>
        </div>
        <div class="mdc-checkbox__ripple"></div>
    </div>
    </div>}
  }


  #[derive(PartialEq, Properties)]
pub struct PropsSwitch {
   #[prop_or(false)]
    pub default_checked: bool,
    pub on_check: Callback<bool>,
}

fn get_checked_class(checked:bool) -> String {
  if  checked {
    String::from("mdc-switch mdc-switch--selected")
    
  }else { 
    String::from("mdc-switch mdc-switch--unselected")
  }

}

#[function_component(Switch)]
  pub  fn switch(props:&PropsSwitch) ->Html {
    let checked = use_state(|| props.default_checked);

    let onchange = {
      let props_onchange = props.on_check.clone();
      let checked = checked.clone();
      
      Callback::from(move |event: MouseEvent| {
          checked.set(!*checked);
          props_onchange.emit(!*checked);
      })
  };
    
    html!{
      <button onclick={onchange} id="basic-switch" class={get_checked_class(*checked)} type="button" role="switch" aria-checked="false">
      <div class="mdc-switch__track"></div>
      <div class="mdc-switch__handle-track">
          <div class="mdc-switch__handle">
          <div class="mdc-switch__shadow">
              <div class="mdc-elevation-overlay"></div>
          </div>
          <div class="mdc-switch__ripple"></div>
          <div class="mdc-switch__icons">
              <svg class="mdc-switch__icon mdc-switch__icon--on" viewBox="0 0 24 24">
              <path d="M19.69,5.23L8.96,15.96l-4.23-4.23L2.96,13.5l6,6L21.46,7L19.69,5.23z" />
              </svg>
              <svg class="mdc-switch__icon mdc-switch__icon--off" viewBox="0 0 24 24">
              <path d="M20 13H4v-2h16v2z" />
              </svg>
          </div>
          </div>
      </div>
      </button>
    }
  }