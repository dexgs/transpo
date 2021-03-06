import { JSChaCha20 } from "./js-chacha20/src/jschacha20.js";
const { Reader, Writer } = window.conflux;


export async function upload(name, files) {
  var ws = new WebSocket("wss://" + document.location.host + document.location.pathname + "ws/" + name);
 
  ws.onopen = function(event) {
    zipEncryptAndSend(files, ws, name);
  };
}


const chunkSize = 60000;

async function zipEncryptAndSend(files, ws, name) {
  var key = genKey();
  var nonce = new Uint8Array(12);
  var crypto = new JSChaCha20(key, nonce);
  var uploaded = 0;
  var fileNames = new Array(files.length);
  if (files.length > 1) {
    var { readable, writable } = new Writer();
    var writer = writable.getWriter();
    for (var i = 0; i < files.length; i++) {
      const file = files[i];
      fileNames[i] = file.name;
      writer.write({
        name: "/" + file.name,
        lastModified: new Date(0),
        stream: () => new Response(file.stream()).body
      });
    }
    var reader = readable.getReader();
  } else {
    fileNames[0] = files[0].name;
    var reader = files[0].stream().getReader();
  }
  new ReadableStream({
    async start(controller) {
      while (true) {
        const { done, value } = await reader.read();
        if (done) {
          setUiEnabled(true);
          removeAllFiles();
          addUploadPreview(fileNames, name, keyString(key));
          controller.close();
          setProgress(100, true);
          //ws.close();
          return;
        }
        if (uploadSize > 2000000) {
          uploaded += value.length;
          const progress = 100 * uploaded / uploadSize;
          // Firefox is fine without this, but Chromium will not yield and allow
          // the progress bar to update without this line.
          await new Promise(r => setTimeout(r, 0));
          setProgress(progress, false);
        }
        const ciphertext = crypto.encrypt(value);
        // upload must be chunked since Chromium browsers like to send the file
        // in 2MB+ increments which the server can't process.
        const numChunks = ~~(ciphertext.length / chunkSize);
        for (var i = 0; i < numChunks; i++) {
          const startIndex = i * chunkSize;
          const endIndex = startIndex + chunkSize;
          const chunk = ciphertext.slice(startIndex, endIndex);
          ws.send(chunk);
        }
        const endChunk = ciphertext.slice(numChunks * chunkSize);
        ws.send(endChunk);
      }
    }
  });
  try {
    writer.close();
  } catch (e) {}
}


function genKey() {
  var key = window.crypto.getRandomValues(new Uint8Array(32));
  return key;
}


function keyString(key) {
  var output = "";
  for (var i = 0; i < key.length; i++) {
    output += byteToHex(key[i]);
  }
  return output;
}


const hexChars = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];


function byteToHex(b) {
  var firstDigit = ~~(b / 16);
  var secondDigit = b - 16 * firstDigit;
  return hexChars[firstDigit] + hexChars[secondDigit];
}
