use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TaskInputProps {
    pub on_add: Callback<String>,
}

#[function_component(TaskInput)]
pub fn task_input(props: &TaskInputProps) -> Html {
    let input_ref = use_node_ref();
    let value = use_state(|| String::new());

    let oninput = {
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };

    let onclick = {
        let value = value.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |_| {
            let text = (*value).trim().to_string();
            if !text.is_empty() {
                on_add.emit(text);
                value.set(String::new());
            }
        })
    };

    let onkeypress = {
        let value = value.clone();
        let on_add = props.on_add.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let text = (*value).trim().to_string();
                if !text.is_empty() {
                    on_add.emit(text);
                    value.set(String::new());
                }
            }
        })
    };

    html! {
        <div class="task-input-container">
            <input
                ref={input_ref}
                type="text"
                value={(*value).clone()}
                {oninput}
                {onkeypress}
                placeholder="Введите новую задачу..."
                class="task-input"
            />
            <button {onclick} class="add-button">
                {"➕ Добавить"}
            </button>
        </div>
    }
}
