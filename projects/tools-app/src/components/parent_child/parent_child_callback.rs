use leptos::{ev::MouseEvent, *};

#[component]
pub fn Child(#[prop(into)]on_increment: Callback<MouseEvent>, #[prop(into)]on_decrement: Callback<MouseEvent>, counter: ReadSignal<i16>) -> impl IntoView {
    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px">
                <h2>"Child Callback"</h2>
                <p>"Counter: "{counter}</p>
                <div>
                    <button on:click=on_increment>"Increment"</button>
                    <button on:click=on_decrement>"Decrement"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Parent() -> impl IntoView {


        let (counter, set_counter) = create_signal::<i16>(0);
    
        let increment_counter = move |_| set_counter.update(|c| *c += 15);
        let decrement_counter = move |_| set_counter.update(|c| *c -= 7);
        view! {
            <div>
                <div style="border: 1px solid black; margin: 4px">
                    <h2>"Parent Callback"</h2>
                    <p>"Counter: "{counter}</p>
                    <div>
                        <button on:click=increment_counter>"Increment"</button>
                        <button on:click=decrement_counter>"Decrement"</button>
                    </div>
                </div>
                <Child counter=counter on_increment=increment_counter on_decrement=decrement_counter/>
            </div>
        }
        
    
    
}