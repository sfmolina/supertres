//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  26oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use yew_router::prelude::*;
use crate::{components::nav_bar::NavbarComponent,
    router::{switch, Route}
};



#[function_component(App)]
pub fn app() -> Html {


    html! {
        <HashRouter>
            <NavbarComponent />
            <main>
                <Switch<Route> render={switch} />
            </main>
        </HashRouter>
    }
}
