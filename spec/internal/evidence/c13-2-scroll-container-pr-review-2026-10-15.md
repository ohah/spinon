# C13.2 계획 PR 변경 관점 30개 검토

- **검토 종류:** 이 PR 변경분 전용 독립 검토 30개
- **대상:** C13.2 plan/contract/oracle/fixtures, CSS reference capture tool/tests, 상태 대장, C13.1 fixture-boundary assertion 정합성 수정
- **검증:** 전체 CSS reference 436/436 PASS · C13.2 reference 5/5 PASS · `node --check` · `git diff --cached --check`
- **기준:** Chrome `155.0.8059.40`, agent-browser `0.31.1`, `640×480` DPR1 · source `e2f346f3dda6d56f68fd6ef2d0255610a282d0f3`

| # | 변경 관점 / 깨뜨리는 조건 | 결과 |
| --- | --- | --- |
| 1 | C13.2 plan PR가 C13 전체 또는 제품 CSS 지원을 완료로 표시한다. | `STATUS.md`의 C13 상위 체크는 미완료다. 신규 행은 사전 oracle·계획만 기록하고 runtime 미시작을 분리했다. |
| 2 | 다음 단계 link가 기존 C13.1 계획/구현 자료를 대체하거나 덮어쓴다. | C13 parent plan에 C13.2 상세 계획만 추가 연결했다. 기존 C13.1 oracle와 evidence는 유지했다. |
| 3 | internal contract ID가 다른 active contract와 충돌한다. | `0062`는 기존 `0060` C13.1 및 `0061` U05.1 뒤 새 ID이며 index도 순서대로 둔다. `0.1.0-draft`를 사용한다. |
| 4 | 기준 Chromium revision이나 executable version이 빠져 oracle가 움직인다. | capture는 `Chrome/155.0.8059.40` 및 pinned WebKit commit mismatch를 실패시킨다. JSON에 V8, protocol, UA도 보존한다. |
| 5 | CSS geometry/DPR가 달라 scroll width 및 offset 결과를 재현하지 못한다. | 두 fixture 모두 `640×480`, DPR1을 capture metadata와 test에 고정했다. |
| 6 | fixture HTML 또는 캡처 PNG가 바뀌었는데 JSON은 예전 내용을 계속 참조한다. | reference에는 두 source 및 screenshot SHA-256이 있고 5개 oracle test가 실제 파일과 대조한다. |
| 7 | Chromium visual overflow fixture가 border 밖 paint를 실제로 표시하지 않는다. | agent-browser PNG에서 visible child의 blue area가 parent의 오른쪽/아래 border 밖에 보인다. |
| 8 | out-of-flow descendant가 scroll area를 늘리는지 frame 밖 paint만 보는 것으로 대체한다. | 별도 `absolute` case의 `scrollWidth=178`, client 114, max x 64를 JSON과 assertion에 고정했다. |
| 9 | box-shadow를 scrollable overflow로 포함한다. | ink-only case의 scroll/client `(114,76)` 일치와 max zero를 독립 검증한다. |
| 10 | `visible`의 큰 scrollWidth를 실제 scroll offset 허용으로 착각한다. | visible boundary case는 scrollWidth 160이지만 write 뒤 offset zero다. test가 두 값을 따로 확인한다. |
| 11 | `clip`과 `hidden`의 차이가 documentation에만 있고 oracle에 없다. | 동일 script write에서 hidden `(55,44)`, clip `(0,0)`을 캡처하고 자동 검증한다. |
| 12 | forced `scroll`과 no-overflow `auto`가 source에서 같은 computed value가 된다. | no-overflow dimensions는 동일해도 computed overflow `auto`/`scroll`을 분리 보존한다. |
| 13 | one-axis case가 실제 cross-axis range를 만들지 않는지 확인하지 않는다. | x max `(86,0)`, y max `(0,74)` exact controls가 있다. |
| 14 | LTR negative writes가 음수 offset으로 들어가거나 RTL 지원으로 잘못 확장된다. | negative x/y setter observation은 zero다. RTL/logical origin은 plan/contract에서 C13.6로 남겼다. |
| 15 | nested scroller가 ancestor와 child offset을 같은 state로 합친다. | outer `(22,17)`와 inner `(42,29)`를 별도 값으로 캡처하고 검증한다. |
| 16 | content growth가 유효 offset을 reset한다. | growth 및 동일 owner descendant reparent 뒤 `(30,20)` 보존 결과가 있다. |
| 17 | content shrink 뒤 range 밖 offset이 남는다. | oversized write max `(146,124)` 뒤 scroll area를 없애면 offset/max가 `(0,0)`이다. |
| 18 | 같은 scrollport node가 tree detach/reinsert 뒤 이전 offset을 되살린다. | Chrome reference는 연결 전 `(37,28)`, 재연결 후 `(0,0)`이며 plan/contract/test 모두 연결한다. |
| 19 | body overflow가 root `visible`에서 viewport로 전파되는 것을 body own scroll로 기록한다. | `window.scrollTo`/HTML은 움직이고 body offset은 0인 body-auto case를 포함했다. |
| 20 | `document.scrollingElement === HTML`가 항상 실제 움직이는 body/viewport owner를 뜻한다고 가정한다. | html hidden/body auto case에서 `scrollingElement`는 HTML이어도 body 자체만 `(64,83)`으로 움직인다. |
| 21 | root scroll가 fixed descendant까지 translation한다. | 네 root mode의 fixed rect가 `[580,444,52,28]`로 같음을 oracle test가 확인한다. |
| 22 | Chrome CSSOM fractional rounding을 runtime internal contract로 승격한다. | `.25/.49→55`, `.5/.51/.75→56` 값을 reference에 남기고 C29/S05에 격리했다. C13.2 runtime parity는 정수 offset이다. |
| 23 | `ScrollStateRevision`이 document/style/environment revision을 재사용한다. | 계약/계획에서 별도 monotonic key, scroll-only stale rejection, no-op 기준을 정의했다. |
| 24 | paint key와 hit-test key가 서로 다른 scroll offset을 본다. | immutable snapshot과 동일 `ScrollStateRevision`을 요구하고 같은 ancestor chain을 두 경로에 적용한다. |
| 25 | allocation/invalid geometry/error 뒤 일부 offset이나 revision만 먼저 publish한다. | 계약/계획이 후보 전체 생성 뒤 atomic publish와 이전 snapshot 보존/worker recovery를 요구한다. |
| 26 | fixture route가 기본 artifact에 포함되는 것으로 오해한다. | 이번 PR은 제품 FFI route를 만들지 않는다. 향후 opt-in archive absence/positive control을 C13.2 pass gate로 둔다. |
| 27 | CSSOM/gesture/scrollbar/snap/WPT 완료로 범위가 부풀려진다. | C29/S05, C13.3/.4/.5/.6과 WPT 미실행 경계를 plan·contract·evidence에 적었다. |
| 28 | platform/simulator 또는 physical device 검증이 이미 된 것처럼 읽힌다. | 이 PR은 Chrome oracle만 제공한다. Android API 37 ARM64 AVD와 지정 iOS Simulator는 구현 PR pass gate이며 실기기는 요구하지 않는다. |
| 29 | 새 검증 도구가 전체 CSS reference 회귀를 깨뜨린다. | frozen dependencies 이후 전체 CSS reference 436/436, C13.2 5/5, JS syntax/diff check가 통과했다. |
| 30 | C13.1 fixture-boundary guard가 현재 `dep:sha2` opt-in feature를 거부하거나 임의 dep를 허용한다. | 이전 main assertion은 전체 CSS reference를 1개 실패시켰다. 현재 검사는 `c04-runtime-gpu-fixture-test`와 `dep:sha2`를 정확히 모두 요구하고 5/5 통과한다. |

## 판정

검토한 변경에서 계획 PR의 병합을 막는 미해결 P0/P1을 찾지 못했다. 구현은 시작하지 않았으며 AVD/iOS Simulator, Rust runtime behavior 또는 public CSSOM 결과를 주장하지 않는다. 이 PR 변경 관점 30개는 C13.2 구현 전 계획 실패 관점 20개와 별개다.
