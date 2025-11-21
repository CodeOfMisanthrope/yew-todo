use yew::prelude::*;
use super::task::TaskDto;
use super::task_item::TaskItem;

#[derive(Properties, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<TaskDto>,
    pub on_delete: Callback<usize>,
}

#[function_component(TaskList)]
pub fn task_list(props: &TaskListProps) -> Html {
    if props.tasks.is_empty() {
        html! {
            <div class="empty-state">
                <p>{"Список пуст. Быстро создай первую задачу!"}</p>
            </div>
        }
    } else {
        html! {
            <div class="task-list">
                {props.tasks.iter().map(|task| {
                    html! {
                        <TaskItem
                            key={task.id}
                            task={task.clone()}
                            on_delete={props.on_delete.clone()}
                        />
                    }
                }).collect::<Html>()}
            </div>
        }
    }
}
