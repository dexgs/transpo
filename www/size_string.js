let units = ["B", "KB", "MB", "GB", "PB"];

function sizeString(numBytes) {
    if (numBytes ==  0) {
        return "0B";
    }

    var power = Math.floor(Math.log10(numBytes) / 3);

    if (power < 5) {
        return String((numBytes / Math.pow(10, power * 3)).toFixed(1)) + " " + units[power];
    } else {
        return "Big";
    }
}
