use crate::models::color::Color;
use crate::repositories::color_repository::AppendColor;
use error::Result;
use leptos::*;
use leptos_router::ActionForm;
use server_fn::ServerFn;

#[component]
pub fn ColorForm(append_color: Action<AppendColor, Result<Color, ServerFnError>>) -> impl IntoView {
    view! {
        <ActionForm action={append_color}>
            <label for="color-name">"Color Name:"
                <input type="text" id="color-name" name="name"/>
            </label>
            <label for="color-hex-code">"Color Hex Code:"
                <input type="text" id="color-hex-code" name="hex_code"/>
            </label>
            <button type="submit">"Add Color"</button>
        </ActionForm>
    }
}
