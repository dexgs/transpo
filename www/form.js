let form = document.getElementById("upload-form");

form.onsubmit = function() {
  var formData = new FormData(form);

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
  xhr.open('POST', form.getAttribute('action'), true);
  xhr.send(formData);

  xhr.onreadystatechange = function() {
    if (xhr.readyState == 4 && xhr.status == 200) {
    }
  }

  return false;
}
