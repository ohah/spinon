# C13.1 float formatting boundary · 구현 실패 관점 20개

대상은 Chrome 3-case 관찰과 `c131_float_formatting_comparison_cases_fail_closed`다. 코드·fixture·원시 실행 로그를 서로 다른 관점에서 확인했다. 첫 검토에서 부모 computed value를 직접 확인하지 않고 root 개수만 보는 약점이 보여, 최종 테스트에 정확한 `display`와 두 overflow 축 검사를 추가했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 브라우저 버전이 계획의 oracle에서 벗어남 | PASS — Chrome for Testing `155.0.8059.40`, 고정 revision을 `chrome://version`으로 확인했다. |
| 2 | 같은 버전 표기지만 실행 바이너리가 다른 빌드임 | PASS — 실행 파일 SHA-256이 기존 C13.1 고정 oracle의 `428dccc…18b6a4`와 같다. |
| 3 | 비교 중 fixture 내용이 달라짐 | PASS — capture source SHA-256 `b7acd7e…4ef16a`가 고정 C13.1 fixture digest와 같다. |
| 4 | viewport 또는 DPR 차이가 box height를 바꿈 | PASS — 실측 viewport `1280×633`, DPR 1을 원본 JSON에 기록했다. |
| 5 | 아래쪽 요소가 viewport 밖이라 paint 관찰을 잘못 주장함 | PASS — y=608/776 rect는 DOM geometry만 기록했고 screenshot/pixel 결과로 확대하지 않았다. |
| 6 | `flow-root`가 cascade에서 `block`으로 치환됨 | PASS — runtime 계산 snapshot의 root `display`가 `flow-root`로 남는지 단언한다. |
| 7 | `hidden` computed overflow가 한 축만 전달됨 | PASS — root의 x/y computed 값 둘 다 `hidden`인지 단언한다. |
| 8 | plain `clip`이 overflow shorthand에서 빠짐 | PASS — root의 x/y computed 값 둘 다 `clip`인지 단언한다. |
| 9 | `clip + flow-root`가 overflow 값을 잃음 | PASS — 두 축 `clip`과 computed `flow-root`를 같이 확인한다. |
| 10 | case별 float가 다른 크기를 써서 height 비교가 무효임 | PASS — 세 oracle case 모두 48×36px이고 fixture와 runtime rejection test가 같은 크기를 쓴다. |
| 11 | `hidden`이 float 높이를 포함하지 않음 | PASS — pinned Chrome parent height는 64px다. |
| 12 | plain `clip`이 float를 포함해 BFC처럼 보임 | PASS — pinned Chrome parent height는 28px다. |
| 13 | `clip + flow-root`가 float를 포함하지 않음 | PASS — pinned Chrome parent height는 64px다. |
| 14 | box-model 기여분을 float 기여분으로 혼동함 | PASS — 각 block edge 14px, 빈 box 28px 및 float 36px 합으로 차이를 설명한다. |
| 15 | scrollHeight를 scroll container 구현 증거로 확대함 | PASS — `clientHeight`/`scrollHeight`는 JSON에 보존하고 C13 scroll 지원으로 주장하지 않는다. |
| 16 | 스타일 cascade 실패를 float rejection으로 오인함 | PASS — 세 inline case 각각에서 cascade root가 만들어진 뒤 layout 단계가 실패해야 통과한다. |
| 17 | float 오류가 있어도 성공 layout/render snapshot을 반환함 | PASS — 각 계산은 `layout.unwrap_err()`를 요구한다. |
| 18 | 오류 종류가 더 일반적인 layout 실패로 바뀜 | PASS — code가 `unsupported_inline_property`인지 정확 비교한다. |
| 19 | 잘못된 node/property를 실패 대상으로 보고함 | PASS — 정확한 float child NodeId와 `float` 속성을 비교한다. |
| 20 | author stylesheet가 inline 검사 경계를 우회함 | PASS — 별도 `<style>` source의 `.painted { float:left }`도 거부된다. |

검증 로그는 이 폴더의 `runtime-targeted-test.log`, `workspace-cargo-test.log`, `workspace-clippy.log`에 있다. 이 검토는 float layout, scroll API, 시뮬레이터 surface, 실기기 동작을 검증했다고 주장하지 않는다.
