import { JSChaCha20 } from "./js-chacha20/src/jschacha20.js";
const FILE_TYPES = {
  "jpg": "image/jpeg",
  "jpeg": "image/jpeg",
  "png": "image/png",
  "zip": "application/zip",
  "gif": "image/gif",
  "mp3": "audio/mpeg",
  "pdf": "application/pdf",
  "rar": "application/vnd.rar",
  "txt": "text/plain",
  "avi": "video/x-msvideo",
};
export async function downloadAndDecrypt(password) {
  const index = document.URL.indexOf("#") + 1;
  const keyString = document.URL.substring(index, document.URL.length);
  const key = getKey(keyString);
  const nonce = new Uint8Array(12);
  const crypto = new JSChaCha20(key, nonce); 
  await fetch(document.URL.substring(0, index), {
    method: 'POST',
    cache: 'no-cache',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: "password=" + encodeURIComponent(password)
  }).then(response => {
    if (response.status == 200) {
      decryptStream(response.body, crypto).then(rs => new Response(rs))
        .then(response => response.blob()).then(blob => {
          var fileName = response.headers.get("content-disposition");
          fileName = fileName.substring(fileName.indexOf("filename") + 10, fileName.length - 1);
          // https://stackoverflow.com/a/42274086
          const file = window.URL.createObjectURL(blob);
          const a = document.createElement("a");
          a.href = file;
          if (fileName.length == 0) {
            a.download = "transpo_" + dateString() + ".zip";
            a.type = "application/zip";
          } else {
            a.download = fileName;
            var extIndex = fileName.lastIndexOf(".");
            if (extIndex != -1) {
              var ext = fileName.substring(extIndex, fileName.length);
              if (ext in FILE_TYPES) {
                a.type = FILE_TYPES[ext];
              }
            }
          }
          document.body.appendChild(a);
          a.click();
          a.remove();
        });
    } else {
      // handle error
    }
  });
}
// https://github.com/mdn/dom-examples/blob/master/streams/simple-pump/index.html
async function decryptStream(response, crypto) {
  const reader = response.getReader();
  return new ReadableStream({
    async start(controller) {
      while (true) {
        const { done, value } = await reader.read();
        if (done) { break; }
        controller.enqueue(crypto.decrypt(value));
      }
      controller.close();
      reader.releaseLock();
    }
  });
}
function getKey(keyString) {
  const key = new Uint8Array(32);
  for (var i = 0; i < keyString.length / 2; i++) {
    const index = i * 2;
    key[i] = hexToByte(keyString.substring(index, index + 2));
  }
  return key;
}
function hexToByte(s) {
  const bytes = new TextEncoder().encode(s);
  return 16 * hexDigit(bytes[0]) + hexDigit(bytes[1]);
}
function hexDigit(n) {
  if (n >= 97) {
    return n - 87;
  } else {
    return n - 48;
  }
}
function dateString() {
  let string = "";
  const date = new Date();
  string += date.getFullYear();
  string += date.getMonth();
  string += date.getDate();
  string += date.getHours();
  string += date.getMinutes();
  string += date.getSeconds();
  return string;
}
