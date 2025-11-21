use yew::prelude::*;
// use crate::task_input::TaskInput;
use super::task::TaskDto;
use super::task_input::TaskInput;
use super::task_list::TaskList;

#[function_component(App)]
pub fn app() -> Html {
    let tasks = use_state(|| Vec::<TaskDto>::new());
    let next_id = use_state(|| 0usize);
    
    let on_add = {
        let tasks = tasks.clone();
        let next_id = next_id.clone();
        Callback::from(move |text: String| {
            let mut new_tasks = (*tasks).clone();
            new_tasks.push(TaskDto {
                id: *next_id,
                text,
            });
            tasks.set(new_tasks);
            next_id.set(*next_id + 1);
        })
    };

    let on_delete = {
        let tasks = tasks.clone();
        Callback::from(move |id: usize| {
            let new_tasks: Vec<TaskDto> = (*tasks)
                .iter()
                .filter(|task| task.id != id)
                .cloned()
                .collect();
            tasks.set(new_tasks);
        })
    };
    
    html! {
        <>
            <div class="container">
                <div class="header">
                    <h1>{"Список задач на Yew"}</h1>
                </div>

                <TaskInput {on_add} />
        
                <div class="tasks-container">
                    <div class="tasks-header">
                        {format!("Задачи: ({})", tasks.len())}
                    </div>
                    <TaskList tasks={(*tasks).clone()} {on_delete} />
                </div>
            </div>
        </>
    }
}
