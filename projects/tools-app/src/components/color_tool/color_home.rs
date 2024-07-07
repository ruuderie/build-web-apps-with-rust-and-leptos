use crate::repositories::color_repository::{all_colors, remove_color, AppendColor, RemoveColor};
//use crate::components::shared::tool_header::ToolHeader;
use crate::components::color_tool::color_form::ColorForm;
use crate::components::color_tool::color_list::ColorList;
use leptos::*;
#[component]
pub fn ColorHome() -> impl IntoView {
    let append_color = create_server_action::<AppendColor>();
    let remove_color = create_server_action::<RemoveColor>();
    let color_resource = create_resource(
        move || (append_color.version().get(), remove_color.version().get()),
        |_| all_colors(),
    );

    view! {
       /*  <ToolHeader header_text="Color Tool".to_string()/> */
        <ColorList color_resource=color_resource remove_color=remove_color/>
        <ColorForm append_color=append_color/>
    }
}
