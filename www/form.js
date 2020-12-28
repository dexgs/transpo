let form = document.getElementById("upload-form");

form.onsubmit = function() {
  var formData = new FormData();

  var files = document.getElementById("file-input").files;

  if (files.length == 0) {
    return false;
  }

  var days = document.getElementById("days").value;
  var hours = document.getElementById("hours").value;
  var minutes = document.getElementById("minutes").value;

  var enableDownloadLimit = document.getElementById("enable-download-limit").checked;
  if (enableDownloadLimit) {
    var downloadLimit = document.getElementById("download-limit").value;
  } else {
    var downloadLimit = 0;
  }

  var enablePassword = document.getElementById("enable-password").checked;
  if (enablePassword) {
    var password = document.getElementById("password").value;
  } else {
    var password = "";
  }

  formData.append("days", days);
  formData.append("hours", hours);
  formData.append("minutes", minutes);
  formData.append("download-limit", downloadLimit);
  formData.append("password", password);
  for (var i = 0; i < files.length; i++) {
    formData.append("file-" + files[i].name, files[i]);
  }

  var xhr = new XMLHttpRequest();

  var progressIndicator = document.getElementById("upload-status").content.cloneNode(true);
  document.getElementById("upload-indicators").appendChild(progressIndicator);
  var children = document.getElementById("upload-indicators").children;
  progressIndicator = children[children.length - 1];

  xhr.onreadystatechange = function() {
    if (xhr.readyState == 4 && xhr.status == 200) {
      progressIndicator.querySelector(".progress-bar").remove();
      var dlLink = progressIndicator.querySelector(".dl-link");
      dlLink.href = document.URL + xhr.responseText;
      dlLink.textContent = xhr.responseText;
      progressIndicator.querySelector(".x-button").style = "";
      progressIndicator.querySelector(".copy-button").style = "";
    }
  }
  
  if (uploadSize >= 10000000) {
    progressIndicator.querySelector(".progress-bar").style = "";
    xhr.upload.addEventListener("progress", function(e) {
      if (e.lengthComputable) {
        progressIndicator.querySelector(".progress-bar").value = (e.loaded / e.total) * 100;
      }
    }, false);
  }

  xhr.open('POST', document.URL, true);
  xhr.send(formData);

  removeAllFiles();

  return false;
}
