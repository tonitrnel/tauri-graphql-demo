import { createEffect, createMemo, createSignal, For, onCleanup, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';

const graphql = async <T = unknown>(
    query: string,
    variables: Record<string, unknown>
): Promise<{ data: T }> => {
    return invoke('graphql', {
        body: {
            query,
            variables
        }
    });
};

type PageInfo = {
    hasPreviousPage: boolean
    hasNextPage: boolean
    startCursor?: string
    endCursor?: string
}
type Connection<N> = {
    edges: {
        node: N,
        cursor: string
    }[]
    // totalCount: number
    pageInfo: PageInfo
}

type Todo = {
    id: string,
    description: string,
    done: boolean,
    created_at: string
}

type Filter = 'all' | 'active' | 'completed';

const services = {
    listTodos: async (params: {
        first?: number
        after?: string
        last?: number
        before?: string
    } = {}) => {
        const list = await graphql<{ listTodos: Connection<Todo> }>(`
          query ListTodos($first: Int, $after: Cursor, $last: Int, $before: Cursor){ 
            listTodos(first: $first, after: $after, last: $last, before: $before) {
              edges {
                node {
                  id
                  description
                  done
                  createdAt
                }
                cursor
              }
              # totalCount
              pageInfo {
                hasPreviousPage
                hasNextPage
                startCursor
                endCursor
              }
            }
          }
      `, params);
        return list.data.listTodos;
    },
    addTodo: async (description: string) => {
        const res = await graphql<string>(`
            mutation AddTodo($description: String!) {
                addTodo(description: $description)
            }
        `, { description });
        return res.data;
    },
    completeTodo: async (id: string, done: boolean) => {
        const res = await graphql<boolean>(`
            mutation CompleteTodo($id: ID!, $done: Boolean!) {
                completeTodo(id: $id, done: $done)
            }
        `, { id, done });
        return res.data;
    },
    toggleAll: async (done: boolean) => {
        const res = await graphql<boolean>(`
            mutation ToggleAll($done: Boolean!) {
                toggleAll(done: $done)
            }
        `, { done });
        return res.data;
    },
    clearCompleted: async () => {
        const res = await graphql<boolean>(`
            mutation ClearCompleted {
                clearCompleted
            }
        `, {});
        return res.data;
    },
    editTodo: async (id: string, description: string) => {
        const res = await graphql<string>(`
            mutation EditTodo($id: ID!, $description: String!) {
                editTodo(id: $id, description: $description)
            }
        `, { id, description });
        return res.data;
    },
    removeTodo: async (id: string) => {
        const res = await graphql<boolean>(`
            mutation RemoveTodo($id: ID!) {
                removeTodo(id: $id)
            }
        `, { id });
        return res.data;
    }
};

declare module 'solid-js' {
    namespace JSX {
        // noinspection JSUnusedGlobalSymbols
        interface Directives {
            setFocus: boolean;
        }
    }
}

// @ts-ignore
// noinspection JSUnusedLocalSymbols
const setFocus = (el: HTMLElement) => setTimeout(() => el.focus());

function App() {
    const [submitting, setSubmitting] = createSignal(false);
    const [todos, setTodos] = createSignal<Todo[]>([]);
    const [editing, setEditing] = createSignal<string | undefined>(undefined);
    const [showMode, setShowMode] = createSignal<Filter>('all');

    const listTodos = async () => {
        const list = await services.listTodos({ first: 999 });
        setTodos(list.edges.map(it => it.node));
    };
    const addTodo = async ({ target, code }: KeyboardEvent) => {
        const description = (target as HTMLInputElement).value.trim();
        if (!['Enter', 'NumpadEnter'].includes(code) || !description) {
            return;
        }
        setSubmitting(true);
        try {
            await services.addTodo(description);
            (target as HTMLInputElement).value = '';
            await listTodos();
        } catch (e) {
            console.error(e);
        }
        setSubmitting(false);
    };
    const removeTodo = async (id: string) => {
        await services.removeTodo(id);
        await listTodos();
    };
    const save = async (todoId: string, { target: { value } }: { target: HTMLInputElement }) => {
        const description = value.trim();
        if (editing() === todoId && description) {
            await services.editTodo(todoId, description);
            await listTodos();
            setEditing(undefined);
        }
    };
    const toggle = async ([id, done]: [id: string, done: boolean]) => {
        await services.completeTodo(id, done);
        await listTodos();
    };
    const toggleAll = async (done: boolean) => {
        await services.toggleAll(done);
        await listTodos();
    };
    const clearCompleted = async () => {
        await services.clearCompleted();
        await listTodos();
    };
    const doneEditing = (todoId: string, e: KeyboardEvent) => {
        if (['Enter', 'NumpadEnter'].includes(e.code)) {
            save(todoId, e as KeyboardEvent & { target: HTMLInputElement }).catch(console.error);
        } else if (e.key === 'Escape') {
            setEditing(undefined);
        }
    };
    const filteredTodos = createMemo(() => {
        switch (showMode()) {
            case 'all':
                return todos();
            case 'active':
                return todos().filter(it => !it.done);
            case 'completed':
                return todos().filter(it => it.done);
        }
    });
    const remainingCount = createMemo(() => todos().filter(it => !it.done).length);

    createEffect(async () => {
        await listTodos();
    });

    const locationHandler = () => setShowMode(location.hash.slice(2) as Filter || 'all');
    window.addEventListener('hashchange', locationHandler);
    onCleanup(() => window.removeEventListener('hashchange', locationHandler));

    // noinspection HtmlUnknownAnchorTarget
    return (
        <section class="todoapp">
            <header class="header">
                <h1>todos</h1>
                <input type="text" class="new-todo" placeholder="What needs to be done?" onKeyDown={addTodo}
                       disabled={submitting()} />
            </header>
            <Show when={todos().length > 0}>
                <section class="main">
                    <input id="toggle-all" class="toggle-all" checked={!remainingCount()} type="checkbox"
                           onInput={({ target: { checked } }) => toggleAll(checked)} />
                    <label for="toggle-all" />
                    <ul class="todo-list">
                        <For each={filteredTodos()}>
                            {(todo) => (
                                <li class="todo" classList={{ editing: editing() === todo.id, completed: todo.done }}>
                                    <div class="view">
                                        <input type="checkbox" class="toggle" checked={todo.done}
                                               onInput={[toggle, [todo.id, !todo.done]]} />
                                        <label onDblClick={[setEditing, todo.id]}>{todo.description}</label>
                                        <button class="destroy" onClick={[removeTodo, todo.id]} />
                                    </div>
                                    <Show when={editing() === todo.id}>
                                        <input class="edit" value={todo.description} onFocusOut={[save, todo.id]}
                                               onKeyUp={[doneEditing, todo.id]} use:setFocus />
                                    </Show>
                                </li>
                            )}
                        </For>
                    </ul>
                </section>

                <footer class="footer">
                    <span class="todo-count">
                        <strong>{remainingCount()}</strong>{' '}
                        {remainingCount() === 1 ? ' item ' : ' items '} left
                    </span>
                    <ul class="filters">
                        <li>
                            <a href="#/" classList={{ selected: showMode() === 'all' }}>All</a>
                        </li>
                        <li>
                            <a href="#/active" classList={{ selected: showMode() === 'active' }}>Active</a>
                        </li>
                        <li>
                            <a href="#/completed" classList={{ selected: showMode() === 'completed' }}>Completed</a>
                        </li>
                    </ul>
                    <Show when={remainingCount() !== todos().length}>
                        <button class="clear-completed" onClick={clearCompleted}>
                            Clear completed
                        </button>
                    </Show>
                </footer>
            </Show>
        </section>
    );
}

export default App;
