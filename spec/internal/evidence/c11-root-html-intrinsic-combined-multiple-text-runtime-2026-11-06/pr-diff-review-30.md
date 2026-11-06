# C11.4 intrinsic columns + multiple text rows PR diff 검토 · 30개

구현, fixture 정정, simulator evidence를 포함한 실제 PR diff를 owner 경계·오류 원자성·근거의 출처와 범위 관점에서 다시 검토했다.

| # | 검토 관점 | 결과 |
|---:|---|---|
| 1 | singleton `grid-auto-rows:min-content` 밖의 값도 새 후보가 되는가 | helper가 정확한 singleton computed track만 선택하고 다른 implicit-row 값 테스트가 거부한다. **통과.** |
| 2 | 기존 fixed explicit-track multiple-text 경로가 달라지는가 | intrinsic explicit track이 없으면 이전 조건과 같은 후보 결과를 돌려준다. 기존 multiple-text regressions가 통과한다. **통과.** |
| 3 | intrinsic columns 없이 intrinsic-row combined branch가 열린다 | intrinsic explicit track이 있는 branch는 intrinsic explicit columns도 요구한다. **통과.** |
| 4 | explicit rows가 한 개의 positive px라는 전제를 놓친다 | rows slice가 유한한 양수 LengthPx 하나인지 검사한다. zero와 다중 row 테스트가 있다. **통과.** |
| 5 | intrinsic explicit row가 새 text route에 들어온다 | helper와 runtime negative가 intrinsic explicit row를 거부한다. **통과.** |
| 6 | fit-content track이 두 축 중 하나에 섞인다 | combined helper는 양 축의 fit-content를 거부하며 runtime failure는 `completed=None`이다. **통과.** |
| 7 | columns/rows auto-repeat 또는 auto columns가 implicit columns를 만든다 | helper unit matrix가 auto-repeat과 implicit columns를 거부한다. Runtime parser는 auto-repeat declarations를 완성 cascade 전에 거부하며 publication이 없다. **통과.** |
| 8 | named areas와 auto placement가 섞인다 | helper와 runtime negative가 named areas를 거부한다. **통과.** |
| 9 | dense/column flow에서 source order와 row identity가 달라진다 | helper는 sparse row flow만 허용하고 기존 C11 flow negatives를 유지한다. **통과.** |
| 10 | hidden, nested 또는 둘 이상의 Grid owner 중 하나만 검증된다 | 공통 validator의 visible owner와 direct-child 제한은 유지된다. hidden/multiple-owner/중첩 음성 regressions가 있다. **통과.** |
| 11 | div 외 element나 anonymous item이 direct Grid item으로 취급된다 | 공통 validator가 direct HTML div만 허용하고 기존 non-div/anonymous negatives가 유지된다. **통과.** |
| 12 | 공백 문법이나 source text 제한이 회귀한다 | 기존 ASCII `A` token/single-space parser를 재사용하며 새 invalid-second-source case가 거부된다. **통과.** |
| 13 | text item's style 지원 범위가 폭넓어진다 | font, line-height, wrapping, dimension, placement 검증을 바꾸지 않는다. nowrap negative는 layout을 atomic reject한다. **통과.** |
| 14 | 첫 text item만 측정하거나 두 item's contribution이 섞인다 | 두 owner별 MinContent contribution과 서로 다른 heights를 layout trace로 검사한다. **통과.** |
| 15 | column track을 풀기 전에 text row 높이를 결정한다 | four column modes에서 used column과 row sequences를 함께 대조한다. **통과.** |
| 16 | Chrome font advance를 Android/iOS native 수치로 간주한다 | Chrome 값은 precomparison으로 별도 기록한다. Rust runtime은 각 실행 경로의 line count/frame을 검사하고 simulator screenshot은 visual smoke로만 분류한다. **통과.** |
| 17 | DOM text order를 뒤집으면 paint order나 row identity가 뒤섞인다 | forward/reverse 모두 source identity, row sequence, box order와 draw-command order를 검증한다. **통과.** |
| 18 | 정정된 fixture가 이전 auto-width control을 남긴다 | 빈 control은 기존 item 계약에 맞는 고정 1px이며 text min-content보다 좁다. Fixture hash와 다시 만든 Chrome record가 일치한다. **통과.** |
| 19 | precomparison screenshot이 Spinon product output처럼 보인다 | Chrome images live under `chrome/` and the precomparison README labels them oracle-only. Product images are separately captured from simulators. **통과.** |
| 20 | test output uses stale fixture expectations or hash | test includes committed `expected.json`; its fixture hash is `ef0cff8d4e53a162cc7a5dfdb0c8c5ec0d7b7f3942ba9985edc4f1f7101e33e6`, equal to simulator-installed HTML. **통과.** |
| 21 | runtime text is stale or source revisions differ from layout | Runtime assertions compare generation, document, render-tree and style revisions for each text owner to the completed key. **통과.** |
| 22 | 형제 사이의 background/text draw order가 달라진다 | runtime test가 각 box command 뒤에 text command가 오고, text 뒤에 다음 형제 box가 오는 순서를 검사한다. **통과.** |
| 23 | 10,000-track success drops frames because trace is over budget | combined intrinsic-column fixture keeps the final frame at 10,000 tracks while omitting optional trace only. **통과.** |
| 24 | 10,001st row publishes a partial layout | combined fixture rejects before frame layout and identifies `grid-auto-rows`; test observes no successful output. **통과.** |
| 25 | unsupported second item/source/style or combined shape leaks partial output | negative cases assert failed layout and `completed=None`; auto-repeat is rejected earlier by the current computed-style profile. **통과.** |
| 26 | simulator build uses an opt-in C11 test route or wrong Android target | Android used only API 37 ARM64 `emulator-5554`; both C11 fixture flags were off and default Root HTML `root-app.js` ran. **통과.** |
| 27 | iOS evidence belongs to a different runtime/device | logs and screenshot identify the selected iPhone 17 Pro / iOS 26.2 simulator UDID. **통과.** |
| 28 | installed app packaged a stale fixture or bootstrap restoration lost user assets | installed HTML/JS hashes match source; original bootstrap files compare byte-for-byte after the runner's cleanup trap. **통과.** |
| 29 | screenshots prove more than they measure | screenshot sizes and visible overflow are recorded, with no PixelCopy, Metal readback, compositor parity, or physical-device claim. **통과.** |
| 30 | isolated green tests hide known broad-suite failures or complete C11 | focused tests, workspace Clippy and format pass; base-reproduced full-suite failures are listed in `baseline-suite-recheck.md`. C11.4/C11 stay incomplete at 78/214. **통과.** |

실제 diff에는 두 simulator screenshots, platform logs, bootstrap recovery copies, precomparison correction, implementation review와 checksum manifest가 포함된다. merge 후 별도 순차 docs PR에서 internal contract 0059, STATUS 및 public/Tailnet roadmap을 동기화한다.
