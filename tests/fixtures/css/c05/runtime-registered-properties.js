{
const existingFixture = globalThis.__spinonC052FixtureState;
if (existingFixture) {
  const currentCss = existingFixture.firstSheetText.data;
  const nextCss = currentCss.includes("initial-value: 23px;")
    ? currentCss.replace("initial-value: 23px;", "initial-value: 37px;")
    : currentCss.replace("initial-value: 37px;", "initial-value: 23px;");
  if (nextCss === currentCss) throw new Error("C05.2 초기값 토글 기준을 찾지 못했습니다");
  existingFixture.firstSheetText.data = nextCss;
  if (existingFixture.secondSheet.nextSibling !== existingFixture.firstSheet) {
    existingFixture.app.insertBefore(existingFixture.secondSheet, existingFixture.firstSheet);
  }
} else {
const app = document.createElement("div");
app.setAttribute("id", "app");
document.appendChild(app);

const firstSheet = document.createElement("style");
firstSheet.setAttribute("id", "sheet-one");
firstSheet.appendChild(document.createTextNode(`
  @property --tile-width {
    syntax: "<length>";
    inherits: false;
    initial-value: 23px;
  }
  @property --tone {
    syntax: "<length>";
    inherits: true;
    initial-value: 7px;
  }
  @property --tile-color {
    syntax: "<color>";
    inherits: false;
    initial-value: rgb(51, 102, 255);
  }
  @property --unknown-descriptor {
    syntax: "<length>";
    inherits: false;
    initial-value: 13px;
    vendor-extension: ignored;
  }
  #app {
    display: flex;
    box-sizing: border-box;
    width: 301px;
    height: 100px;
    align-items: flex-start;
    gap: 5px;
    background-color: #123456;
  }
  .tile {
    display: block;
    box-sizing: border-box;
    height: 14px;
    flex-shrink: 0;
    background-color: var(--tile-color);
  }
  #declared { --tile-width: 41px; width: var(--tile-width); }
  #default { width: var(--tile-width); }
  #invalid { --tile-width: red; width: var(--tile-width); }
  #inherit-parent {
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 60px;
    height: 30px;
    flex-shrink: 0;
    --tile-width: 71px;
    --tone: 19px;
  }
  #not-inherited { width: var(--tile-width); }
  #inherited { width: var(--tone); }
  #late { width: var(--late-width); }
  @property --late-width {
    syntax: "<length>";
    inherits: false;
    initial-value: 31px;
  }
  #unknown { width: var(--unknown-descriptor); }
  #inline-priority { --tile-width: 39px; width: var(--tile-width); }
  #color-override { --tile-color: #ff6600; width: 11px; }
  @property --duplicate {
    syntax: "<length>";
    inherits: false;
    initial-value: 17px;
  }
`));
const firstSheetText = firstSheet.firstChild;
app.appendChild(firstSheet);

function appendTile(id, inlineStyle) {
  const tile = document.createElement("div");
  tile.setAttribute("id", id);
  tile.setAttribute("class", "tile");
  if (inlineStyle) tile.setAttribute("style", inlineStyle);
  app.appendChild(tile);
  return tile;
}

appendTile("declared");
appendTile("default");
appendTile("invalid");

const inheritParent = document.createElement("div");
inheritParent.setAttribute("id", "inherit-parent");
app.appendChild(inheritParent);
for (const id of ["not-inherited", "inherited"]) {
  const child = document.createElement("div");
  child.setAttribute("id", id);
  child.setAttribute("class", "tile");
  inheritParent.appendChild(child);
}

appendTile("late");
appendTile("unknown");
appendTile("inline-priority", "--tile-width: 47px !important");
appendTile("color-override");
appendTile("duplicate");

const secondSheet = document.createElement("style");
secondSheet.setAttribute("id", "sheet-two");
secondSheet.appendChild(document.createTextNode(`
  @property --duplicate {
    syntax: "<number>";
    inherits: false;
    initial-value: 3;
  }
  #duplicate { --duplicate: 29px; width: var(--duplicate); }
`));
app.appendChild(secondSheet);
globalThis.__spinonC052FixtureState = { app, firstSheet, firstSheetText, secondSheet };
}
}
