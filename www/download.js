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
  const fileId = document.URL.substring(0, index);
  await fetch(fileId, {
    method: 'POST',
    cache: 'no-cache',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: "password=" + encodeURIComponent(password)
  }).then(response => {
    if (response.status == 200) {
      if (window.indexedDB) {
        decryptAndDownload(response, crypto, fileId);
      } else {
        decryptStream(response.body, crypto).then(rs => new Response(rs))
          .then(response => response.blob()).then(blob => {
            var fileName = response.headers.get("content-disposition");
            fileName = fileName.substring(fileName.indexOf("filename") + 10, fileName.length - 1);
            var fileType = "application/octet-stream";
            if (fileName.length == 0) {
              fileName = "transpo_" + dateString() + ".zip";
              fileType = "application/zip";
            } else {
              var extIndex = fileName.lastIndexOf(".");
              if (extIndex != -1) {
                var ext = fileName.substring(extIndex, fileName.length);
                if (ext in FILE_TYPES) {
                  fileType = FILE_TYPES[ext];
                }
              }
            }
            downloadBlob(blob, fileName, fileType);
          });
      }
    }
  });
}


function fileNameParts(response) {
  let fileName = response.headers.get("content-disposition");
  fileName = fileName.substring(fileName.indexOf("filename") + 10, fileName.length - 1);
  if (fileName.length == 0) {
    fileName = "transpo_" + dateString() + ".zip";
    return [ fileName, "application/zip"];
  } else {
    const extIndex = fileName.lastIndexOf(".");
    if (extIndex != -1) {
      const ext = fileName.substring(extIndex, fileName.length);
      if (ext in FILE_TYPES) {
        return [fileName, FILE_TYPES[ext]];
      } else {
        return [fileName, "application/octet-stream"];
      }
    }
  }
}


// https://github.com/mdn/dom-examples/blob/master/streams/simple-pump/index.html
async function decryptStream(response, crypto) {
  const reader = response.getReader();
  return new ReadableStream({
    async start(controller) {
      let downloadedBytes = 0;
      while (true) {
        await new Promise(r => setTimeout(r, 0));
        const { value, done } = await reader.read();
        if (done) { break; }
        downloadedBytes += value.size;
        controller.enqueue(crypto.decrypt(value));
      }
      controller.close();
      reader.releaseLock();
    }
  });
}


async function decryptAndDownload(response, crypto, fileId) {
  // https://medium.com/free-code-camp/a-quick-but-complete-guide-to-indexeddb-25f030425501
  var request = window.indexedDB.open("transpo", 1);
  request.onupgradeneeded = function(event) {
    let db = event.target.result;
    db.createObjectStore("files");
  };
  request.onsuccess = async function(event) {
    let downloadedBytes = 0;
    let db = event.target.result;
    let buffer = new Blob();
    let numChunks = 0;
    let progressIndicator = document.getElementById("download-progress");
    const reader = response.body.getReader();
    while (true) {
      await new Promise(r => setTimeout(r, 0));
      const { done, value } = await reader.read();
      if (done) { break; }
      downloadedBytes += value.length;
      progressIndicator.innerHTML = sizeString(downloadedBytes) + " downloaded";
      const plaintext = crypto.decrypt(value);
      buffer = new Blob([buffer, plaintext], { type: "application/octet-stream" });
      if (buffer.size > 50000000) {
        let currentChunk = numChunks;
        let currentBuffer = buffer;
        const transaction = db.transaction("files", "readwrite");
        const store = transaction.objectStore("files");
        await store.put(currentBuffer, fileId + currentChunk);
        buffer = new Blob();
        numChunks += 1;
      }
    }
    progressIndicator.innerHTML += " (finished)"
    const parts = fileNameParts(response);
    const fileName = parts[0];
    const fileType = parts[1];
    let finalBlob = new Blob();
    const transaction = db.transaction("files", "readwrite");
    const store = transaction.objectStore("files");
    for (var i = 0; i < numChunks; i++) {
      let index = i;
      const key = fileId + index;
      store.get(key).onsuccess = async function(e) {
        const blob = e.target.result;
        finalBlob = new Blob([finalBlob, blob], { type: "application/octet-stream" });
        store.delete(key);
        if (index == numChunks - 1) {
          finalBlob = new Blob([finalBlob, buffer]);
          downloadBlob(finalBlob, fileName, fileType);
          await store.clear();
        }
      };
    }
    if (numChunks == 0) {
      downloadBlob(buffer, fileName, fileType);
    }
  };
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


function downloadBlob(blob, fileName, fileType) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = fileName;
  a.type = fileType;
  document.body.appendChild(a);
  a.click();
  a.remove();
}
