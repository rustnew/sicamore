use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use web_sys::KeyboardEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Todo {
    task: Signal<String>,
    completed: Signal<bool>,
    id: u32,
}
#[component(inline_props)]
fn TodoItem<F>(todo: Todo, remove_todo: F) -> View
where
    F: Fn(u32) + Copy + 'static,
{
    on_cleanup(move || {
        todo.task.dispose();
        todo.completed.dispose();
    });

    // We are using inline styles here which is generally not considered best practice.
    // In real app, you would probably use an external CSS file.
    let style = move || {
        if todo.completed.get() {
            "text-decoration: line-through;"
        } else {
            ""
        }
    };
    let toggle_completed = move |_| todo.completed.set(!todo.completed.get());
    let remove_todo = move |_| remove_todo(todo.id);

    let is_editing = create_signal(false);
    let start_editing = move |_| is_editing.set(true);

    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" && !todo.task.with(String::is_empty) {
            is_editing.set(false);
        }
    };

    view! {
        li {
            span(style=style, on:click=toggle_completed) {
                (if is_editing.get() {
                    view! { input(bind:value=todo.task, on:keydown=on_keydown) }
                } else {
                    view! { (todo.task) }
                })
            }
            button(on:click=start_editing, disabled=is_editing.get()) { "Edit Task" }
            button(on:click=remove_todo) { "Remove" }
        }
    }
    
}

#[component(inline_props)]
fn TodoList<F>(#[prop(setter(into))] todos: MaybeDyn<Vec<Todo>>, remove_todo: F) -> View
where
    F: Fn(u32) + Copy + 'static,
{
    view! {
        ul {
            Keyed(
                list=todos,
                view=move |todo| view! { TodoItem(todo=todo, remove_todo=remove_todo) },
                key=|todo| todo.id,
            )
        }
    }
}


#[component(inline_props)]
fn TodoInput<F>(add_todo: F) -> View
where
    F: Fn(String) + 'static,
{
    let input = create_signal(String::new());

    let on_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" && !input.with(String::is_empty) {
            add_todo(input.get_clone());
            input.set(String::new());
        }
    };

    view! {
        div {
            "New Todo: "
            input(bind:value=input, on:keydown=on_keydown)
        }
    }
}

#[component]
fn App() -> View {
    // Initialize application state from localStorage.
    let local_storage = window()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");

    let todos: Signal<Vec<Todo>> = if let Ok(Some(app_state)) = local_storage.get_item("todos") {
        serde_json::from_str(&app_state).unwrap_or_default()
    } else {
        Default::default()
    };

    // Set up an effect that runs whenever app_state.todos changes to save the todos to
    // localStorage.
    create_effect(move || {
        todos.with(|todos| {
            // Also track all nested signals.
            for todo in todos {
                todo.task.track();
                todo.completed.track();
            }
            local_storage
                .set_item("todos", &serde_json::to_string(todos).unwrap())
                .unwrap();
        });
    });

    let next_id = create_signal(0);
    // `replace(...)` is the same as `set(...)` but returns the previous value.
    let get_next_id = move || next_id.replace(next_id.get() + 1);

    let add_todo = move |task| {
        todos.update(|todos| {
            todos.push(Todo {
                task: create_signal(task),
                completed: create_signal(false),
                id: get_next_id(),
            })
        })
    };

    let remove_todo = move |id| todos.update(|todos| todos.retain(|todo| todo.id != id));

    view! {
        TodoInput(add_todo=add_todo)
        TodoList(todos=todos, remove_todo=remove_todo)
    }
}

fn main() {
    sycamore::render(App);
}
