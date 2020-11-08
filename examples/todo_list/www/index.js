import { get_todos, mark_as_done } from "todo-rpc";

function createTodoNode(todo) {
    const newNode = document.createElement("p");
    const textNode = document.createTextNode(todo.content);
    const checkBoxNode = document.createElement("input");
    checkBoxNode.setAttribute("type", "checkbox");
    checkBoxNode.checked = todo.completed;
    checkBoxNode.onchange = function onCheck() {
        markTodoAsDone(todo.id);
    }

    newNode.appendChild(checkBoxNode);
    newNode.appendChild(textNode);
    return newNode;
}

function markTodoAsDone(id) {
    mark_as_done(id);
}

window.onload = async function() {
    // This is how easy it is to make an RPC call!
    const todos = await get_todos();

    todos.forEach(todo => {
        window.document.getElementById("todos").appendChild(createTodoNode(todo));
    })
}();
