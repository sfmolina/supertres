//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v2                                                  //
//  Modified:  31dec24                                            //
//---------------------------------------------------------------//



use yew::prelude::*;
use crate::components::{
    super_tres::SuperTresComponent,
    nav_bar::NavbarComponent
};



#[function_component(App)]
pub fn app() -> Html {


    html! {
        <>
            <NavbarComponent />
            <SuperTresComponent />
        </>
    }
}
