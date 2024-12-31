//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v2                                                  //
//  Modified:  31dec24                                            //
//---------------------------------------------------------------//



use yew::prelude::*;



#[function_component(NavbarComponent)]
pub fn navbar() -> Html {

    html! {
        <div class="nav-style">
            <nav class="navbar is-fixed-top" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item username" href="https://sfmolina.github.io/">
                        {"@sfmolina"}
                    </a>
                </div>
            </nav>
        </div>
    }

}
