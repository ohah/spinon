export function parseC11GridFrames(log) {
  return [...log.matchAll(/SPINON_C111_GRID_NODE_FRAME_SWIFT[^\n]*?node=(\d+),x=(-?\d+(?:\.\d+)?),y=(-?\d+(?:\.\d+)?),width=(\d+(?:\.\d+)?),height=(\d+(?:\.\d+)?)/g)]
    .map((match) => ({
      nodeId: Number(match[1]),
      rect: {
        x: Number(match[2]),
        y: Number(match[3]),
        width: Number(match[4]),
        height: Number(match[5]),
      },
    }));
}

export function hasC11GridDrawn(log, boxCount) {
  return log.split(/\r?\n/).some((line) =>
    line.includes('SPINON_C0410_DRAW')
    && line.includes(`status=0 presented boxes=${boxCount}`));
}
