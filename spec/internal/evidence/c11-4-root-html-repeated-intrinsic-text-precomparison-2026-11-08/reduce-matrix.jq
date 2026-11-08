{
  reference: {
    browser: "Chrome 157 via Agent Browser",
    userAgent: .ua,
    dpr: .dpr,
    fixtureTitle: .fixtureTitle,
    fixtureSha256: "923a9d831a6509c1eb577428296535369146034336ac5af6658af0506157b8a9"
  },
  uniqueTailOrders,
  textDescriptors,
  scenes: [
    .scenes[] | {
      tail, cap, align, gridHeight, gridRows,
      gridFrame: [.gridFrame.x, .gridFrame.y, .gridFrame.width, .gridFrame.height],
      itemFrames: [.frames[] | [.x, .y, .width, .height]],
      payloadFrames: [.frames[] | select(.payload != null) | [.payload.x, .payload.y, .payload.width, .payload.height]]
    }
  ]
}
