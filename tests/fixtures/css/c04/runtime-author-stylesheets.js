const app = document.createElement("div");
app.setAttribute("id", "app");
document.appendChild(app);

const firstSheet = document.createElement("style");
firstSheet.setAttribute("id", "sheet-one");
firstSheet.setAttribute("type", "text/css");
firstSheet.setAttribute("media", "screen");
firstSheet.appendChild(document.createTextNode(`
  #app {
    display: flex;
    box-sizing: border-box;
    width: 100vw;
    height: 100vh;
    flex-direction: row;
    align-items: flex-start;
    justify-content: flex-start;
    gap: var(--space);
    --space: 11px;
    --surface: #123456;
    background-color: var(--surface);
  }
  div.tile {
    display: block;
    box-sizing: border-box;
    width: 24px;
    height: 14px;
    --tile: #3366ff;
    background-color: var(--tile);
  }
  .tile { width: 31px; }
  #second { width: 39px !important; }
`));
app.appendChild(firstSheet);

const first = document.createElement("div");
first.setAttribute("id", "first");
first.setAttribute("class", "tile");
first.setAttribute("style", "width: 47px !important");
app.appendChild(first);

const secondSheet = document.createElement("style");
secondSheet.setAttribute("id", "sheet-two");
secondSheet.setAttribute("type", "TEXT/CSS;charset=utf-8");
secondSheet.appendChild(document.createTextNode(`
  .tile { width: 41px; }
  #second { width: 43px !important; }
`));
app.appendChild(secondSheet);

const second = document.createElement("div");
second.setAttribute("id", "second");
second.setAttribute("class", "tile");
second.setAttribute("style", "width: 47px");
app.appendChild(second);
