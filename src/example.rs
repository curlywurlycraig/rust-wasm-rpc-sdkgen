#[mirror]
struct Todo {
    ...
}

#[remote]
fn get_todos() -> Vec<Todo> {

}

#[remote]
fn mark_as_done(todo: &Todo) {
    // SQL
}

#[url("/todos")]
fn main() {
    html!{
        <div>
            { get_todos().iter().map(|todo| {
                html!{
                    <button
                        on_click={|| {
                            mark_as_done(&todo)
                        }}
                    />
                }
            })}
        </div>
    }
}