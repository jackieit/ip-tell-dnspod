use yew::prelude::*;
use gloo_console::log;

#[function_component(Record)]
pub fn record() -> Html {
    let domain_name = use_state(|| String::new());
    let record_type = use_state(|| String::new());
    let record_value = use_state(|| String::new());

    let on_domain_name_change = {
        let domain_name = domain_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            domain_name.set(input.value());
        })
    };

    let on_record_type_change = {
        let record_type = record_type.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            record_type.set(input.value());
        })
    };

    let on_record_value_change = {
        let record_value = record_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            record_value.set(input.value());
        })
    };
    
    let on_submit = {
        let domain_name = domain_name.clone();
        let record_type = record_type.clone();
        let record_value = record_value.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // Handle form submission here
            
            // log!(
            //     "Domain: {}, Record Type: {}, Record Value: {}",
            //     *domain_name.clone(),
            //     *record_type.clone(),
            //     *record_value.clone()
            // );
            // log domain_name, record_type, record_value
            log!(
                "Domain: {}, Record Type: {}, Record Value: {}",
                &*domain_name,
                &*record_type,
                &*record_value
            );
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <div>
                <label for="domain_name">{ "Domain Name" }</label>
                <input
                    type="text"
                    id="domain_name"
                    value={(*domain_name).clone()}
                    oninput={on_domain_name_change}
                />
            </div>
            <div>
                <label for="record_type">{ "Record Type" }</label>
                <input
                    type="text"
                    id="record_type"
                    value={(*record_type).clone()}
                    oninput={on_record_type_change}
                />
            </div>
            <div>
                <label for="record_value">{ "Record Value" }</label>
                <input
                    type="text"
                    id="record_value"
                    value={(*record_value).clone()}
                    oninput={on_record_value_change}
                />
            </div>
            <button type="submit">{ "Submit" }</button>
        </form>
    }
}
