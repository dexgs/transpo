// https://www.smashingmagazine.com/2018/01/drag-drop-file-uploader-vanilla-js/
let fileArea = document.getElementById("file-area");
let filePreviews = document.getElementById("file-previews");
let fileMap = new Map();
let previewTemplate = document.getElementById("file-preview");
let fileInput = document.getElementById("file-input");

let removeAllFilesButton = document.getElementById("remove-all-files");
let uploadButton = document.getElementById("upload");
let selectFilesButton = document.getElementById("select-files");

let uploadTemplate = document.getElementById("upload-status");
let uploadIndicators = document.getElementById("upload-indicators");

let disablableElements = [removeAllFilesButton, uploadButton, selectFilesButton];

let progressBar = document.getElementById("progress-bar");

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

function setUiEnabled(state) {
  for (var i = 0; i < disablableElements.length; i++) {
    disablableElements[i].disabled = !state;
  }

  var style = ""
  if (!state) {
    style = "display: none"
  }
  document.getElementById("file-previews").querySelectorAll(".x-button").forEach(function(button) {
    button.style = style;
  });
}

function setDisabledClass(state) {
  if (state) {
    for (var i = 0; i < disablableElements.length; i++) {
      disablableElements[i].classList.add("disabled-button");
    }
  } else {
    for (var i = 0; i < disablableElements.length; i++) {
      disablableElements[i].classList.remove("disabled-button");
    }
  }
}

const MAX_LENGTH = 18;

function addUploadPreview(fileNames, uploadURL, keyString) {
  var preview = uploadTemplate.content.cloneNode(true);

  var maxLength = MAX_LENGTH;

  if (fileNames.length > 1) {
    var quantity = "+" + (fileNames.length - 1) + " more";
    maxLength -= quantity.length;
    preview.querySelector(".upload-quantity").textContent = quantity;
  }

  var fileName = fileNames[0];
  var index = fileName.lastIndexOf(".");
  if (index != -1) {
    var fileExt = fileName.substring(index, fileName.length).substring(0, 5);
    var fileStem = fileName.substring(0, index);
    if (fileStem.length > maxLength) {
      fileStem = fileStem.substring(0, maxLength) + "… ";
    }
    preview.querySelector(".upload-name").textContent = fileStem;
    preview.querySelector(".upload-extension").textContent = fileExt;
  } else {
    preview.querySelector(".upload-name").textContent = fileName;
  }

  if (fileNames.length > 1) {
    preview.querySelector(".upload-quantity").textContent = "+" + (fileNames.length - 1) + " more";
  }

  var linkTitle = fileName;
  for (var i = 1; i < fileNames.length; i++) {
    linkTitle += "\n" + fileNames[i];
  }
  
  var link = preview.querySelector("a");
  link.title = linkTitle;
  link.href = document.URL + uploadURL + "#" + keyString;

  uploadIndicators.appendChild(preview);
}

function setProgress(progress, isDone) {
  progressBar.style.width = progress + "%";
  if (isDone) {
    progressBar.style.display = "none";
  } else {
    progressBar.style.display = "block";
  }
}

function copyDownloadLink(preview) {
  var link = preview.querySelector("a");
  navigator.clipboard.writeText(link.href);
}
