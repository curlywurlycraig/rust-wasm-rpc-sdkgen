import { get_todos, add_todo } from "todo-rpc";

const createTodoNode = (todo) => {
    const newNode = document.createElement("p");
    const textNode = document.createTextNode(todo.content);
    const checkBoxNode = document.createElement("input");
    checkBoxNode.setAttribute("type", "checkbox");
    checkBoxNode.checked = todo.completed;

    newNode.appendChild(checkBoxNode);
    newNode.appendChild(textNode);
    return newNode;
}

window.onload = async () => {
    // This is how easy it is to make an RPC call!
    const todos = await get_todos();

    todos.forEach(todo => {
        window.document.getElementById("todos").appendChild(createTodoNode(todo));
    })
};