import * as Upload from "./upload.js";

let form = document.getElementById("upload-form");

form.onsubmit = function() {

  var formData = new FormData();

  var files = document.getElementById("file-input").files;

  if (files.length == 0) {
    return false;
  }

  if (uploadSize > 2000000) {
    setDisabledClass(true);
  } else {
    setDisabledClass(false);
  }

  setUiEnabled(false);

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

  var body = "";
  body += "days=" + days + "&";
  body += "hours=" + hours + "&";
  body += "minutes=" + minutes + "&";
  body += "download_limit=" + downloadLimit + "&";
  body += "password=" + encodeURIComponent(password) + "&";
  body += "file_name="
  if (files.length == 1) {
    body += encodeURIComponent(files[0].name);
  }

  var xhr = new XMLHttpRequest();

  xhr.onreadystatechange = function() {
    if (xhr.readyState == 4 && xhr.status == 200) {
      Upload.upload(xhr.responseText, files);
    }
  }
  
  xhr.open("POST", document.URL, true);
  xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
  xhr.send(body);

  return false;
}
