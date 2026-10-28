# C11.4 `minmax(auto, max-content)` 구현 PR 변경분 검토 · 실패 관점 30개

| # | 제출 diff에서 확인한 독립 실패 경로 | 판정 |
|---:|---|---|
| 1 | 기존 테스트 파일 연결 없이 새 파일만 추가됨 | 통과 — 부모 test module에 명시 `#[path]`로 하위 suite를 등록했다. |
| 2 | 새 테스트 코드가 production library 모듈로 빌드됨 | 통과 — 등록된 부모 자체가 `#[cfg(test)]`이고 runtime/ABI 경로는 건드리지 않았다. |
| 3 | vendored Taffy 또는 product implementation을 시험에 맞춰 변경함 | 통과 — diff는 기존 테스트 파일에 2줄, 새 unit-test 파일에 한정된다. |
| 4 | fixture-only Cargo feature 또는 mobile build flag가 추가됨 | 통과 — 순수 Rust unit test여서 별도 feature/ABI가 없다. |
| 5 | 기존 `1fr` regression을 새 배열로 교체함 | 통과 — 기존 14개 `CASES`와 helper는 그대로 두고 별도 `max_content` child를 쓴다. |
| 6 | case matrix가 코드에서 암묵적으로 생성되어 한 값이 조용히 빠짐 | 통과 — 10개 기대값을 case별 고정 상수로 적었다. |
| 7 | case id가 중복되어 실패 시 잘못된 입력으로 보임 | 통과 — 두 축에 `column-`/`row-` 접두어가 있어 전부 구분된다. |
| 8 | Rows/Columns loop가 axis를 반대로 선택함 | 통과 — 각 loop가 해당 axis variant만 필터링한다. |
| 9 | off-axis overflow가 target axis 값과 함께 바뀌어 축별 선택 오류를 가리거나, 단위 입력이 baseline과 달라졌는데 숨겨짐 | 통과 — Columns에서 Y, Rows에서 X를 `visible` control로 고정하고, 계획에 이 축 격리 이유와 타깃 축 baseline 일치를 명시했다. |
| 10 | enum overflow와 기대 그룹이 어긋남 | 통과 — `visible`/`clip`과 `hidden`/`auto`/`scroll`이 각 기대 source와 직접 연결됐다. |
| 11 | Ahem fixture를 새로 복사해 서로 다른 font bytes를 사용함 | 통과 — 부모 모듈의 고정 `AHEM` bytes를 참조한다. |
| 12 | Ahem 폭이 font size와 글자 수에 맞지 않음 | 통과 — 10 glyph `X`, 10px Ahem으로 100px contribution을 검사한다. |
| 13 | Columns container width가 60px이 아니거나 definite가 아님 | 통과 — root width/viewport width 모두 60px fixed이다. |
| 14 | Columns cross-axis height가 frame 판단에 끼어듦 | 통과 — height 20px와 row track 20px로 고정하고 item height를 확인한다. |
| 15 | Rows image natural size가 oracle과 다름 | 통과 — intrinsic snapshot에 정확히 24×16px를 넣는다. |
| 16 | Rows inline width/column track width가 달라 transferred size가 변함 | 통과 — 둘 다 15px로 고정한다. |
| 17 | Rows root height가 definite 5px가 아님 | 통과 — root style과 viewport에서 height 5px를 준다. |
| 18 | Rows item이 stretch되어 expected overflow frame을 잃음 | 통과 — `align_self`를 `FlexStart`로 설정하고 image height 10px을 검사한다. |
| 19 | host snapshot과 intrinsic-size snapshot의 source revision이 서로 다름 | 통과 — HostDocument generation/document/render-tree revision으로 intrinsic snapshot source를 만든다. |
| 20 | 반복된 Rows case가 같은 HostDocument owner를 공유함 | 통과 — 각 case에 `4000 + index` owner를 지정한다. |
| 21 | root/item ownership 및 placement가 잘못되어 다른 item의 trace를 읽음 | 통과 — 단일 direct child와 한-cell line placement를 사용하고 node ID로 trace를 찾는다. |
| 22 | 중간 track trace나 collapsed/gutter row를 final target으로 선택함 | 통과 — 마지막 stage에서 일반 track만 남기고 정확히 하나인지 확인한다. |
| 23 | track base와 Grid item frame만 비교하고 root frame은 빠짐 | 통과 — root와 item의 모든 frame dimension/position을 검사한다. |
| 24 | Minimum trace가 누락되거나 중복됐는데 첫 값만 비교함 | 통과 — trace count가 정확히 1인지 먼저 단언한다. |
| 25 | AutomaticMinimum source 조건 중 일부가 diff에서 빠짐 | 통과 — auto min track, span 수, flexible maximum, content-based flag를 모두 검사한다. |
| 26 | `OverflowAutomaticMinimum`의 source는 맞아도 값이 양수임 | 통과 — source와 0px value를 각각 검증한다. |
| 27 | MaxContent contribution이 누락·중복되거나 minimum 값으로 대체됨 | 통과 — 별도 kind로 정확히 하나의 trace 및 기대값 100/10px을 확인한다. |
| 28 | layout error가 case context 없이 실패해 원인을 찾기 어려움 | 통과 — 모든 compute 실패에 해당 case id를 포함한다. |
| 29 | Chrome screenshot을 일반 링크로 넣어 Rspress가 PNG를 HTML route로 해석하고 dead-link 오류를 냄 | 통과 — 첫 build에서 발견한 일반 링크를 Markdown image embed로 바꿨고, 재실행 로그에는 이 evidence 경로 오류가 없다. |
| 30 | focused gate만 통과한 상태에서 package regression/lint/format도 통과했다고 주장함 | 통과 — 기록은 focused 2/2, package 297/297, rustfmt check, crate all-target Clippy의 실제 실행 결과로 제한한다. |

구현/runtime 실패 관점 20개와 구별되는 최종 PR diff 검토다. 이 PR은 CSS admission, painting, scroll offsets, app runtime 또는 simulator 동작을 바꾸지 않는다.
