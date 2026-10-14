# C13.1 hit identity 구현 실패 관점 검토 · 20개 · 2026-10-14

대상은 13개 유효 C13.1 runtime row의 축별 정수 hit identity assertion과 positioned absolute content-box 별도 control이다. 관찰 및 사전 판정은 [README](README.md), [Chrome identity runs](chrome155-element-hit-identity.json), [capture script](capture-element-hit-identity.js)에 고정했다.

| # | 실패 관점 | 적대적 확인 결과 |
| --- | --- | --- |
| 1 | 브라우저 버전 또는 revision drift | Chrome for Testing 155.0.8059.40과 고정 revision을 test에서 확인한다. |
| 2 | UA·viewport·DPR이 달라 hit 위치가 변함 | UA, 1280×1100 CSS px 및 DPR 1을 test에서 assert한다. |
| 3 | fixture 변경 뒤 observation을 계속 사용 | 원본 fixture SHA와 선행 observation SHA를 test에서 assert한다. |
| 4 | CSS selector 이름과 Rust test row가 뒤섞임 | 각 raw row의 id와 target ID를 대조한다. target ID 규칙과 다른 row는 실패한다. |
| 5 | raw row에 target box가 빠졌는데 빈 sweep 성공 | 각 축 run은 target frame의 floor(start)에서 ceil(end)-1까지 간격 없이 연속해야 한다. |
| 6 | browser의 wrapper element를 제품 node로 오인 | `.case`/`main`/`body`/`html`은 단일 synthetic root로 접는다. 별도 DOM wrapper의 identity를 증명한다고 주장하지 않는다. |
| 7 | target positive range가 부모 fallback으로 바뀜 | Chrome `target` run마다 exact target NodeId를 요구한다. |
| 8 | clip 바깥 점에서 None만 확인하고 잘못된 상위 node를 놓침 | Chrome `box` run마다 clip parent NodeId를 요구한다. |
| 9 | clip parent box edge 뒤에도 parent가 계속 hit됨 | `positioned-contained` x=532..535에서 box fallback, x=536..545에서 root fallback을 확인한다. |
| 10 | root가 hit target 범위 끝까지 살아남지 않음 | clip parent box 바깥의 `root` run마다 exact root NodeId를 요구한다. |
| 11 | nested clip leaf 뒤 fallback이 inner node로 잘못 귀결 | nested x=324..337 및 y=1016..1019에서 Chrome의 outer box fallback을 확인한다. |
| 12 | nested overflow clip을 union하여 leaf 반환 | nested target run은 Chrome target 좌표까지만 target NodeId를 허용한다. |
| 13 | axis-only clip의 visible 축을 불필요하게 제한 | target y/x 각 sweep에서 browser가 기록한 visible-axis run을 그대로 검사한다. |
| 14 | clip margin이 target/parent 경계를 잘못 확장 | padding/content/border margin 행의 target과 box/root runs를 비교한다. |
| 15 | scrollable overflow 행에서 clip margin을 적용 | mixed `hidden`/`auto`의 background fallback run도 Chrome observation과 대조한다. |
| 16 | positioned in-flow hit 동작을 absolute-child 규칙으로 변경 | 세 기존 selected rows와 새 행 모두 동일 identity helper로 검사한다. in-flow parent-border hit가 유지된다. |
| 17 | positioned-contained absolute가 clip edge에서 부모로 fallback하지 않음 | x=531은 target, 532–535는 clip box, 536부터 root를 기대한다. |
| 18 | containing-block escape가 clip parent에 잘못 fallback | escape target의 frame 안 축별 모든 identity run을 Chrome과 비교한다. |
| 19 | content-box control에서 target 외 정확한 경계가 틀림 | live observation x=119 target, 120/122/123 clip, 124 root를 기록하고 Rust NodeId를 비교한다. |
| 20 | 1D sweep를 전체 hit 적합성으로 확대 | 축별 target frame 안의 표본만 검증한다. 전체 2D, frame 바깥 전 영역, DOM event 및 surface는 미검증이다. |

결과는 matrix module 8/8, dedicated content-box control 1/1, render crate 전체 52 passed·1 ignored다. 이 검토는 invalid margin과 float 세 fail-closed row를 성공 layout으로 계산하지 않으며 C13.1 완료를 선언하지 않는다.
