//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  26oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::super_tres::SuperTresComponent;



#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Root,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => html! { <SuperTresComponent /> }
    }
}
