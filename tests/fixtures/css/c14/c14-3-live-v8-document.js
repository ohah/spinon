var c14C143Ids = [
  "min-content",
  "border-box-min-content",
  "max-content",
  "fit-content-small-basis",
  "percent-width",
  "percent-height",
  "fit-content-max-clamped",
  "percent-cyclic-width",
  "flex-min-content",
  "flex-percent-width",
  "flex-percent-height",
  "grid-min-content",
  "grid-percent-width",
  "grid-percent-height"
];
var c14C143Root = document.createElement("div");
c14C143Root.setAttribute("id", "root");
for (var c14C143Index = 0; c14C143Index < c14C143Ids.length; c14C143Index++) {
  var c14C143Id = c14C143Ids[c14C143Index];
  var c14C143Basis = document.createElement("div");
  c14C143Basis.setAttribute("id", "basis-" + c14C143Id);
  var c14C143Image = document.createElement("img");
  c14C143Image.setAttribute("id", c14C143Id);
  c14C143Image.setAttribute("alt", "");
  c14C143Image.setAttribute("src", "https://spinon.invalid/c14-1-png-basic.png");
  c14C143Basis.appendChild(c14C143Image);
  c14C143Root.appendChild(c14C143Basis);
}
document.appendChild(c14C143Root);
