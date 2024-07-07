use leptos::*;

#[component]
pub fn Child(counter: ReadSignal<i16>, set_counter: WriteSignal<i16>) -> impl IntoView {
    let increment_counter = move |_| set_counter.update(|c| *c += 15);
    let decrement_counter = move |_| set_counter.update(|c| *c -= 7);
    view! {
        <div>
            <div style="border: 1px solid black; margin: 4px">
                <h2>"Child Write Signal"</h2>
                <p>"Counter: "{counter}</p>
                <div>
                    <button on:click=increment_counter>"Increment"</button>
                    <button on:click=decrement_counter>"Decrement"</button>
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
                <h2>"Parent Child Write Signal"</h2>
                <p>"Counter: "{counter}</p>
                <div>
                    <button on:click=increment_counter>"Increment"</button>
                    <button on:click=decrement_counter>"Decrement"</button>
                </div>
            </div>
            <Child counter=counter set_counter=set_counter/>
        </div>
    }
    
}