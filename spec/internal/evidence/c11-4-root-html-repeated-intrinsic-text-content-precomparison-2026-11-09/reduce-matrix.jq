{
  reference: {
    browser: "Chrome 157 via Agent Browser",
    userAgent: .ua,
    dpr: .dpr,
    fixtureTitle: .fixtureTitle,
    fixtureSha256: $fixture_sha
  },
  uniqueTailOrders,
  contentCases,
  textDescriptors,
  scenes: [
    .scenes[] | {
      contentCase, tail, cap, align, gridHeight, gridRows,
      gridFrame: [.gridFrame.x, .gridFrame.y, .gridFrame.width, .gridFrame.height],
      itemFrames: [.frames[] | [.x, .y, .width, .height]],
      payloadFrames: [.frames[] | select(.payload != null) | [.payload.x, .payload.y, .payload.width, .payload.height]]
    }
  ]
}
