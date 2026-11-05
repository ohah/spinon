# C11.4 Root HTML intrinsic implicit row precomparison

The fixed Chrome 156 input is [the precomparison fixture](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-precomparison.html). Its SHA-256 is `9698ba9a203b8ba01a89bc7ea3f73e686e2f69d8f945df74c787d99d485d151a`.

Agent Browser reported `HeadlessChrome/156.0.0.0`. The computed row list was `10px 22px 34px` for one explicit 10px row followed by two `grid-auto-rows:min-content` rows. The direct children had fixed block sizes of 10px, 22px, and 34px, so their independent min-content contributions set the three used row sizes. The expected row origins are therefore y=0, 10, and 32px. The 40px-tall Grid has visible overflow; the final item ends at y=66px, 26px beyond the Grid.

The independent expected values are in [expected.json](expected.json); raw browser observations and screenshot hash are in [chrome156-observation.json](chrome156-observation.json). [Chrome screenshot](chrome156.png) is browser evidence only. It is not Spinon runtime or WPT evidence.

The Agent Browser session was closed with `agent-browser close --all`; no `agent-browser` process remained. The temporary loopback HTTP server on port 18876 was stopped after capture.
