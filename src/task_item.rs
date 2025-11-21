use yew::prelude::*;
use super::task::TaskDto;

#[derive(Properties, PartialEq)]
pub struct TaskItemProps {
    pub task: TaskDto,
    pub on_delete: Callback<usize>,
}

#[function_component(TaskItem)]
pub fn task_item(props: &TaskItemProps) -> Html {
    let task_id = props.task.id;
    let on_delete = props.on_delete.clone();

    let onclick = Callback::from(move |_| {
        on_delete.emit(task_id);
    });

    html! {
        <div class="task-item">
            <span class="task-text">{&props.task.text}</span>
            <button {onclick} class="delete-button">
                {"Удалить"}
            </button>
        </div>
    }
}
