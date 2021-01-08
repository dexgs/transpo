function setTheme(theme) {
  window.localStorage.setItem("theme", theme);
  loadTheme(theme);
}

function loadTheme(theme) {
  document.getElementById("theme").href = theme;
}

function loadStoredTheme() {
  const theme = window.localStorage.getItem("theme");
  if (theme != undefined) {
    loadTheme(theme);
    updateDropdown(theme);
  }
}

function updateDropdown(theme) {
  // https://stackoverflow.com/a/7373115
  const themeSelect = document.getElementById("theme-select");
  if (themeSelect != null) {
    const option = themeSelect.querySelector("#theme-select [value=\"" + theme + "\"]");
    if (option != null) {
      option.selected = true;
    }
  }
}

loadStoredTheme()
