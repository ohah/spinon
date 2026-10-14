# C13.1 Android 앱 hit identity grid 계획 실패 관점 검토 · 2026-10-14

구현 전에 계획과 고정된 Chrome precomparison을 검토했다. 범위는 compact 13-case fixture의 Android 앱 경로와 정수 CSS point 30,000개뿐이며 paint, DOM event, fractional coordinates 또는 C13.1 전체 지원을 완료했다고 확대하지 않는다.

| # | 계획 실패 관점 | 사전 판정과 근거 |
|---:|---|---|
| 1 | Android app route가 Chrome precomparison과 다른 fixture source를 쓴다 | 두 fixture 파일 SHA와 기존 observation SHA를 고정하고 FFI가 같은 41-node DOM tree를 사용해야 한다. **기준 고정.** |
| 2 | 기존 고정 Chrome observation이 다른 revision/browser에서 생성됐다 | Chrome 155.0.8059.40 revision, agent-browser 0.31.1, UA를 validator가 고정한다. **기준 고정.** |
| 3 | Android surface viewport가 300×100 CSS layout을 덮어쓴다 | full-grid route는 layout viewport를 확인한 뒤 query하며 `innerWidth`와 device pixels를 섞지 않는다. existing surface route에서 viewport override가 과거 실패였으므로 runtime log 기준에 포함했다. **통과 기준 포함.** |
| 4 | Chrome 페이지가 스크롤된 좌표를 zero-origin으로 착각한다 | oracle validator는 scroll `(0,0)`을 요구하고 AVD route도 CSS point를 viewport-local로 전달한다. **통과 기준 포함.** |
| 5 | 정수 CSS 좌표를 pixel center와 같은 의미로 잘못 라벨링한다 | 기존 `elementFromPoint(x,y)` 자료와 같은 정수 grid를 사용하며 center grid라고 부르지 않는다. **통과.** |
| 6 | 마지막 행/열의 fractional point Chrome null을 제품의 clip 결과로 오인한다 | center probe에서 우측 열·아래 행 총 399 null을 분리 보존하고 CSSOM View 알고리즘과 다른 관찰을 limitation으로 둔다. 구현 oracle은 integer query만 쓴다. **경계 분리.** |
| 7 | RLE 행 하나가 빠지거나 두 행이 중복된다 | validator가 100 rows와 30,000 point count를 요구하고 순서대로 expand한다. **통과.** |
| 8 | RLE run 사이에 pixel hole 또는 overlap이 있다 | 매 run 시작이 직전 end이고 모든 row가 정확 `[0,300)`를 덮어야 한다. gap/overlap mutation이 거부된다. **통과.** |
| 9 | run이 300 밖으로 넘어가거나 0-width/역전된다 | integer end, 증가, width 상한을 검증한다. **통과.** |
| 10 | node index 255 sentinel이 실제 node처럼 인정된다 | full oracle에 unexpected hit가 없고 모든 index가 0..40이어야 한다. **통과.** |
| 11 | node ID order가 document source order와 달라 NodeId 비교를 뒤집는다 | root→case/box/[inner]/target의 41 ID 순서를 pinned source의 case order에서 재구성한다. **통과.** |
| 12 | 같은 node ID가 두 index에 등록돼 잘못된 equality를 만든다 | 중복 없는 registry와 정확한 41 ID 목록을 요구한다. **통과.** |
| 13 | root fallback, clip box, visible target을 서로 같은 배경색이라 구별하지 못한다 | oracle은 paint RGBA가 아닌 `data-c131-surface-node`와 DOM ID를 비교한다. **통과.** |
| 14 | full 30,000 grid가 기존 52 fixed query를 재현하지 않는다 | 기존 52 sample ID/좌표는 전체 grid subset과 exact `hitId`로 대조한다. **52/52 통과.** |
| 15 | 기존 host 1.4M result를 Android app runtime result로 대신한다 | plan은 app FFI path에서 별도 30,000 query와 AVD runtime marker를 요구한다. **경계 명시.** |
| 16 | 30,000 hit results를 현재 JNI 32,768-byte buffer에서 JSON으로 잘라 성공 처리한다 | summary-only FFI response와 explicit serialization error를 요구한다. 전체 결과를 넣지 않는다. **통과 기준 포함.** |
| 17 | mismatch 좌표가 없어 원인을 재현할 수 없다 | total mismatch와 처음 16개의 `(x,y,expected,actual)`을 bounded error summary로 보존한다. **통과 기준 포함.** |
| 18 | 30,000 query마다 Logcat을 써 runtime timeout/로그 누수를 만든다 | query loop는 direct bounded Rust call, native log는 summary 한 건과 mismatch preview만 허용한다. **통과 기준 포함.** |
| 19 | query가 다른 renderer handle, surface generation 또는 stale layout에 적용된다 | 기존 host route handle과 frame/surface generation을 한 번 고정하고 동일 run marker로 report/PixelCopy를 연결한다. **통과 기준 포함.** |
| 20 | app matrix pass를 event dispatch, fractional hit, iOS matrix 또는 C13.1 완료로 과장한다 | scope 및 final status에서 Android AVD integer hit identity만 판정하고 나머지를 명시적으로 제외한다. **통과.** |

계획은 [C13.1 Android AVD 앱 hit identity grid](../../../../plan/c13-1-android-avd-app-hit-identity-grid.md), fixed oracle은 [precomparison README](README.md)와 [capture validation](precomparison-validation.log)에 둔다. 계획 검토와 PR 변경 검토를 합치지 않는다.
