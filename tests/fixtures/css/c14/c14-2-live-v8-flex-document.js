var c14Main = document.createElement("main");
c14Main.setAttribute("id", "cases");
var c14Cases = __SPINON_C14_2_CASES__;
for (var c14Index = 0; c14Index < c14Cases.length; c14Index++) {
  var c14Case = c14Cases[c14Index];
  var c14Parent = document.createElement("div");
  c14Parent.setAttribute("id", c14Case.id);
  c14Parent.setAttribute("class", c14Case.parentClass);
  if (c14Case.parentStyle) c14Parent.setAttribute("style", c14Case.parentStyle);
  var c14Image = document.createElement("img");
  c14Image.setAttribute("id", c14Case.id + "-image");
  c14Image.setAttribute("alt", "");
  c14Image.setAttribute("src", "https://spinon.invalid/c14-2-png-basic.png");
  if (c14Case.style) c14Image.setAttribute("style", c14Case.style);
  c14Parent.appendChild(c14Image);
  c14Main.appendChild(c14Parent);
}
document.appendChild(c14Main);
