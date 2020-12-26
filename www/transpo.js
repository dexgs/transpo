// https://www.smashingmagazine.com/2018/01/drag-drop-file-uploader-vanilla-js/
let fileArea = document.getElementById("file-area");
let filePreviews = document.getElementById("file-previews");
let fileMap = new Map();
let previewTemplate = document.getElementById("file-preview");
let fileInput = document.getElementById("file-input");

let uploadSize = 0;

fileArea.addEventListener("drop", dropFiles, false);

function dropFiles(e) {
    let dt = e.dataTransfer;
    addFiles(dt.files);
}

['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
  fileArea.addEventListener(eventName, preventDefaults, false)
})

function preventDefaults (e) {
  e.preventDefault()
  e.stopPropagation()
}

function addFiles(files) {
    for (var i = 0; i < files.length; i++) {
        var file = files.item(i);
        if (!fileMap.has(file.name)) {
            uploadSize += file.size;
            addFilePreview(file);
            fileMap.set(file.name, file);
        }
    }
    updateFiles();
}

function addFilePreview(file) {
    var preview = previewTemplate.content.cloneNode(true);
    var nameLabel = preview.querySelector(".file-name");
    var sizeLabel = preview.querySelector(".file-size");
    nameLabel.textContent = file.name;
    sizeLabel.textContent = sizeString(file.size);
    sizeLabel.title = String(file.size) + " bytes";
    filePreviews.appendChild(preview);
}

function removePreviewAndFile(preview) {
    var fileName = preview.querySelector(".file-name").textContent;
    uploadSize -= fileMap.get(fileName).size;
    fileMap.delete(fileName);
    preview.remove();
    updateFiles();
}

function removeAllFiles() {
    var length = filePreviews.children.length;
    for (var i = 0; i < length; i++) {
        removePreviewAndFile(filePreviews.children[0]);
    }
}

function updateFiles() {
    const dt = new DataTransfer();
    for (const file of fileMap.values()) {
        dt.items.add(file);
    }
    fileInput.files = dt.files;
    var allFiles = document.getElementById("all-files");
    if (fileMap.size == 0) {
        allFiles.style = "display: none;";
    } else {
        allFiles.style = "display: block;";
    }
    document.getElementById("total-size").textContent = sizeString(uploadSize);
}

let units = ["B", "KB", "MB", "GB", "PB"];

function sizeString(numBytes) {
    var power = Math.floor(Math.log10(numBytes) / 3);
    if (power < 5) {
        return String((numBytes / Math.pow(10, power * 3)).toFixed(1)) + " " + units[power];
    } else {
        return "Big";
    }
}
