{
  reference: {
    browser: "Chrome 157 via Agent Browser",
    userAgent: .ua,
    dpr: .dpr,
    fixtureTitle: .fixtureTitle,
    fixtureSha256: "bac3d41dcb39b64016e6789677c692334604ca22880b5b01ee6df5ac114e9f0b"
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
