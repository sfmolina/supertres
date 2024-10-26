//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  26oc24                                             //
//---------------------------------------------------------------//



mod app;
mod components;



use app::App;



fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
