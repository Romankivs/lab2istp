const uri = '/tableware';
let tablewares = [];
function getTableware() {
    fetch(uri + "/list")
        .then(response => response.json())
.then(data => _displayTableware(data))
.catch(error => console.error('Unable to get tableware.', error));
}
function addTableware() {
    const addManIdTextbox = document.getElementById('add_manufacturer_id');
    const addNameTextbox = document.getElementById('add_name');
    const addTypeTextbox = document.getElementById('add_type');
    const addMaterialTextbox = document.getElementById('add_main_material');
    const addColourTextbox = document.getElementById('add_main_colour');
    const tableware = {
        manufacturer_id : parseInt(addManIdTextbox.value.trim()),
        name: addNameTextbox.value.trim(),
        type_ : addTypeTextbox.value.trim(),
        main_material: addMaterialTextbox.value.trim(),
        main_colour : addColourTextbox.value.trim()
    };
    fetch(uri, {
        method: 'POST',
    headers: {

    'Accept': 'application/json',
    'Content-Type': 'application/json'
    },
    body: JSON.stringify(tableware)
})
.then(response => response.json())
.then(() => {
        getTableware();
        addManIdTextbox.value = '';
        addNameTextbox.value = '';
        addTypeTextbox.value = '';
        addMaterialTextbox.value = '';
        addColourTextbox.value = '';
})
.catch(error => console.error('Unable to add tableware.', error));
}
function deleteCategory(id) {
    fetch(`${uri}/${id}`, {
        method: 'DELETE'
})
.then(() => getTableware())
.catch(error => console.error('Unable to delete tableware.', error));
}
function displayEditForm(id) {
    const tableware = tablewares.find(tableware => tableware.tableware_id === id);
    document.getElementById('edit_id').value = tableware.tableware_id;
    document.getElementById('edit_manufacturer_id').value = tableware.manufacturer_id;
    document.getElementById('edit_name').value = tableware.name;
    document.getElementById('edit_type').value = tableware.type_;
    document.getElementById('edit_main_material').value = tableware.main_material;
    document.getElementById('edit_main_colour').value = tableware.main_colour;
    document.getElementById('editTableware').style.display = 'block';
}
function updateTableware() {
    const tablewareId = parseInt(document.getElementById('edit_id').value.trim());
    const tableware = {
            manufacturer_id: parseInt(document.getElementById('edit_manufacturer_id').value.trim()),
            name: document.getElementById('edit_name').value.trim(),
            type_: document.getElementById('edit_type').value.trim(),
            main_material: document.getElementById('edit_main_material').value.trim(),
            main_colour: document.getElementById('edit_main_colour').value.trim()
};
    fetch(uri + "/" + tablewareId, {
        method: 'PUT',
    headers: {
    'Accept': 'application/json',
    'Content-Type': 'application/json'
    },
    body: JSON.stringify(tableware)
})
.then(() => getTableware())
.catch(error => console.error('Unable to update tableware.', error));
    closeInput();
    return false;
}
function closeInput() {
    document.getElementById('editTableware').style.display = 'none';
}
function _displayTableware(data) {
    const tBody = document.getElementById('tableware');

    tBody.innerHTML = '';
    const button = document.createElement('button');
    data.forEach(tableware => {
        let editButton = button.cloneNode(false);
        editButton.innerText = 'Edit';
        editButton.setAttribute('onclick', `displayEditForm(${tableware.tableware_id})`);
        let deleteButton = button.cloneNode(false);
        deleteButton.innerText = 'Delete';
        deleteButton.setAttribute('onclick', `deleteCategory(${tableware.tableware_id})`);
        let tr = tBody.insertRow();
        let td1 = tr.insertCell(0);
        let textNode = document.createTextNode(tableware.tableware_id);
        td1.appendChild(textNode);
        let td2 = tr.insertCell(1);
        let manIdNode = document.createTextNode(tableware.manufacturer_id);
        td2.appendChild(manIdNode);
        let td3 = tr.insertCell(2);
        let nameNode = document.createTextNode(tableware.name);
        td3.appendChild(nameNode);
        let td4 = tr.insertCell(3);
        let typeNode = document.createTextNode(tableware.type_);
        td4.appendChild(typeNode);
        let td5 = tr.insertCell(4);
        let materialNode = document.createTextNode(tableware.main_material);
        td5.appendChild(materialNode);
        let td6 = tr.insertCell(5);
        let colourNode = document.createTextNode(tableware.main_colour);
        td6.appendChild(colourNode);
        let td7 = tr.insertCell(6);
        td7.appendChild(editButton);
        let td8 = tr.insertCell(7);
        td8.appendChild(deleteButton);
});
    tablewares = data;
}