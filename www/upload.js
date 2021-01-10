import { JSChaCha20 } from "./js-chacha20/src/jschacha20.js";
const { Reader, Writer } = window.conflux;

export function upload(name, files) {

  var ws = new WebSocket("wss://" + document.location.host + document.location.pathname + "ws/" + name);

  ws.addEventListener('open', function (event) {
    zipEncryptAndSend(files, ws, name);
  });
}

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
    start(controller) {
      pump();
      function pump() {
        reader.read().then(({done, value}) => {
          if (done) {
            setUiEnabled(true);
            removeAllFiles();
            addUploadPreview(fileNames, name, keyString(key));
            controller.close();
            setProgress(100, true);
            return;
          }
          if (uploadSize > 2000000) {
            uploaded += value.length;
            setProgress(100 * uploaded / uploadSize, false);
          }
          ws.send(crypto.encrypt(value));
          pump();
        });
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
