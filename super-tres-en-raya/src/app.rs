//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  26oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use crate::components::{
    super_tres::SuperTresComponent,
    nav_bar::NavbarComponent
};



#[function_component(App)]
pub fn app() -> Html {


    html! {
        <main>
            <NavbarComponent />
            <SuperTresComponent />
        </main>
    }
}
