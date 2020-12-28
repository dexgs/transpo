function copyDownloadLink(indicator) {
  var text = document.createElement("textarea");
  //text.style = "display: none;";
  text.value = indicator.querySelector(".dl-link").href;
  document.body.appendChild(text);
  text.select();
  document.execCommand("copy");
  document.body.removeChild(text);
}
