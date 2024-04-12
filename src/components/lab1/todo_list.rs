use leptos::{ev::MouseEvent, *};
use stylance::{import_crate_style, import_style};

import_crate_style!(
    #[allow(dead_code)]
    my_style,
    "src/components/lab1/todo_list.module.css"
);
import_style!(todo_style, "todo_list.module.css");

#[derive(Clone)]
struct Todo {
    id: usize,
    description: String,
    is_complete: RwSignal<bool>,
}

/// Usage of using if/else for control flow is demonstrated here
#[component]
pub fn TodoList() -> impl IntoView {
    let initial_todos = vec![
        Todo {
            id: 0,
            description: "Finish lecture".to_string(),
            is_complete: create_rw_signal(true),
        },
        Todo {
            id: 1,
            description: "Do homework".to_string(),
            is_complete: create_rw_signal(false),
        },
        Todo {
            id: 2,
            description: "Sleep".to_string(),
            is_complete: create_rw_signal(true),
        },
    ];

    let (todo_list, set_todo_list) = create_signal(initial_todos);

    let remove_todo = move |id| {
        set_todo_list.update(|todo_list| {
            todo_list.retain(|todo_item| {
                // manually destroy signal to prevent memory leaks
                if todo_item.id == id {
                    todo_item.is_complete.dispose();
                }
                todo_item.id != id
            });
        });
    };

    // let toggle_complete = set_todo_list.update(|todo|)

    view! {
        <section>
            <h2>"Todo List"</h2>
            <For each=todo_list key=|todo_item| todo_item.id let:todo_item>
                <TodoItem todo_item=todo_item.clone() on_click=move |_| remove_todo(todo_item.id)/>
            </For>
        </section>
    }
}

#[component]
fn TodoItem(todo_item: Todo, #[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    let message = move || {
        if todo_item.is_complete.get() {
            Some(" (Done!)")
        } else {
            None
        }
    };
    view! {
        <div class=my_style::todo>
            <label>
                <input
                    type="checkbox"
                    checked=todo_item.is_complete
                    on:change=move |_| {
                        todo_item.is_complete.update(|is_complete| *is_complete = !*is_complete);
                    }
                />

                {todo_item.description}
                {message}
            </label>
            <button on:click=on_click>Remove</button>
        </div>
    }
}
