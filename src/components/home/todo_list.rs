use leptos::ev::MouseEvent;
use leptos::prelude::*;
use stylance::import_style;

// You can add scoped styles from an external css file using external crates (libraries)
// This app is using the stylance crate https://crates.io/crates/stylance to import styles
import_style!(todo_style, "todo_list.module.css");

#[derive(Clone)]
struct Todo {
    id: usize,
    description: String,
    is_complete: RwSignal<bool>,
}

/// Todo list component that handles displaying the add todo and to-do items
/// Passing closures as props is also introduced in this section
#[component]
pub fn TodoList() -> impl IntoView {
    let initial_todos = vec![
        Todo {
            id: 0,
            description: "Finish lecture".to_string(),
            is_complete: RwSignal::new(true),
        },
        Todo {
            id: 1,
            description: "Do homework".to_string(),
            is_complete: RwSignal::new(false),
        },
        Todo {
            id: 2,
            description: "Sleep".to_string(),
            is_complete: RwSignal::new(true),
        },
    ];

    let (todo_list, set_todo_list) = signal(initial_todos);
    let mut next_id = todo_list.get_untracked().len();

    // Example of passing a callback as a prop. Both remove_todo and add_todo show the different
    // ways you pass in closures (as callbacks) into components.
    let remove_todo = move |id: usize| {
        set_todo_list.update(|todo_list| {
            todo_list.retain(|todo_item| {
                // Manually destroy signal to prevent memory leaks
                if todo_item.id == id {
                    todo_item.is_complete.dispose();
                }
                todo_item.id != id
            });
        });
    };

    let add_todo = move |description: String| {
        set_todo_list.update(move |todo_list| {
            todo_list.push(Todo {
                id: next_id,
                description,
                is_complete: RwSignal::new(false),
            });
        });
        next_id += 1;
    };

    view! {
        <section>
            <h2>"Todo List"</h2>
            <For each=move || todo_list.get() key=|todo_item| todo_item.id let:todo_item>
                <TodoItem
                    todo_item=todo_item.clone()
                    on_click=move |_| { remove_todo(todo_item.id) }
                />
            </For>
            <AddTodo add_todo=add_todo />

        </section>
    }
}

/// Renders a single todo item with a checkbox, which is linked to the todo_items is_complete
/// field. If an item is complete then the message "Done!" will show. It also contains a remove
/// button that removes the item from the todo list.
#[component]
fn TodoItem(todo_item: Todo, on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    let message = move || {
        if todo_item.is_complete.get() {
            Some(" (Done!)")
        } else {
            None
        }
    };
    view! {
        <div class=todo_style::todo>
            <label>
                <input
                    type="checkbox"
                    checked=todo_item.is_complete
                    // Since Todo is passed in as a prop, and is_complete is a RwSignal, you can
                    // directly mutate the variable here with a closure
                    on:change=move |_| {
                        todo_item.is_complete.update(|is_complete| *is_complete = !*is_complete);
                    }
                />
                {todo_item.description}
                {message}
            </label>
            // the on_click function passed in as a callback is called here
            <button on:click=on_click>Remove</button>
        </div>
    }
}

/// Add todo form that handles adding a new todo to the todo list by calling the add_todo function.
#[component]
fn AddTodo<F>(mut add_todo: F) -> impl IntoView
where
    F: FnMut(String) + 'static,
{
    let description = NodeRef::new();
    view! {
        <h3>"Add Todo"</h3>
        <form class=todo_style::todo_form>
            <label for="todoDescription">"Description"</label>
            <input id="todoDescription" type="text" node_ref=description />
            <input
                type="submit"
                on:click=move |ev| {
                    ev.prevent_default();
                    add_todo(description.get().expect("ref should have loaded").value());
                }
            />

        </form>
    }
}
