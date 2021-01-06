import * as Download from "./download.js";

document.getElementById("unlock-form").onsubmit = function() {
  Download.downloadAndDecrypt(document.getElementById("password").value);
  return false;
}
