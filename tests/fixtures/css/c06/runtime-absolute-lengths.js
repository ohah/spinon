const root = document.createElement("div");
root.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;width:100%;height:100%;flex-direction:column;row-gap:2px;background-color:#101827"
);
document.appendChild(root);

const negativeRow = document.createElement("div");
negativeRow.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;flex:0 0 8px;flex-direction:row;width:280px;height:8px;margin-left:-4.5pt;padding-left:8px;column-gap:0;background-color:#26354f"
);
root.appendChild(negativeRow);

const negativeProbe = document.createElement("div");
negativeProbe.setAttribute(
  "style",
  "display:block;box-sizing:border-box;flex:0 0 auto;width:1in;height:6px;background-color:#ff9933"
);
negativeRow.appendChild(negativeProbe);

const variableRow = document.createElement("div");
variableRow.setAttribute(
  "style",
  "display:flex;box-sizing:border-box;flex:0 0 8px;flex-direction:row;width:280px;height:8px;margin-left:6px;padding-left:3px;column-gap:3px;background-color:#26354f"
);
root.appendChild(variableRow);

const variableProbe = document.createElement("div");
variableProbe.setAttribute(
  "style",
  "display:block;box-sizing:border-box;flex:0 0 auto;--physical-length:2.54cm;width:var(--physical-length);height:6px;background-color:#cc55aa"
);
variableRow.appendChild(variableProbe);

const measurements = [
  ["px", "96px", "6px", "48px", "6px", "3px", "3px"],
  ["in", "1in", "0.0625in", "0.5in", "0.0625in", "0.03125in", "0.03125in"],
  ["cm", "2.54cm", "0.15875cm", "1.27cm", "0.15875cm", "0.079375cm", "0.079375cm"],
  ["mm", "25.4mm", "1.5875mm", "12.7mm", "1.5875mm", "0.79375mm", "0.79375mm"],
  ["q", "101.6Q", "6.35Q", "50.8Q", "6.35Q", "3.175Q", "3.175Q"],
  ["pt", "72pt", "4.5pt", "36pt", "4.5pt", "2.25pt", "2.25pt"],
  ["pc", "6pc", "0.375pc", "3pc", "0.375pc", "0.1875pc", "0.1875pc"],
];

for (const [unit, width, height, basis, margin, padding, gap] of measurements) {
  const row = document.createElement("div");
  row.setAttribute(
    "style",
    `display:flex;box-sizing:border-box;flex:0 0 8px;flex-direction:row;width:280px;height:8px;margin-left:${margin};padding-left:${padding};column-gap:${gap};background-color:#26354f`
  );
  root.appendChild(row);

  const widthProbe = document.createElement("div");
  widthProbe.setAttribute(
    "style",
    `display:block;box-sizing:border-box;flex:0 0 auto;width:${width};height:${height};background-color:#3366ff`
  );
  row.appendChild(widthProbe);

  const basisProbe = document.createElement("div");
  basisProbe.setAttribute(
    "style",
    `display:block;box-sizing:border-box;flex:0 0 ${basis};height:${height};background-color:#22aa66`
  );
  row.appendChild(basisProbe);
}
