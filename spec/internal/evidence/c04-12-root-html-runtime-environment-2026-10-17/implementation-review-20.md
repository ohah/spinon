# C04.12 root HTML 환경 cascade 구현 실패 관점 검토

- 대상: viewport/color-scheme 해석, root-only cascade profile, 내부 FFI, Android/iOS 환경 갱신 경로
- 기준 변경: `feat/c04-12-document-environment-20261017`의 구현 변경
- 검토 방식: 구현 경로를 실패 조건별로 나눠 코드·고정 fixture·실행 로그와 대조했다. 이전 parser/asset/기본 앱/계획 검토 표는 재사용하지 않았다.
- 실행 결과: [근거 README](README.md), Cargo 및 두 Simulator 원본 로그를 참조한다.

## 검토 항목

1. **root HTML이 없는 세션의 API 호출** — metadata 없는 세션은 `InvalidEnvironment`로 끝난다. 기존 JavaScript-only 동작과 경계가 유지된다.
2. **metadata 소유 요소** — 실제 HTML namespace `html > head > meta`만 입력으로 읽는다. body metadata나 다른 namespace 요소는 viewport를 바꾸지 않는다.
3. **중복 viewport metadata** — 여러 viewport meta 중 마지막 요소를 적용한다. 고정 Chrome duplicate fixture와 Rust 결과가 일치한다.
4. **content descriptor 갱신** — 지원 descriptor의 마지막 유효 width/scale만 앞 값을 대체하고 invalid 값은 이전 유효 값을 지우지 않는다. 미지원 descriptor는 기본 계산을 유지한다.
5. **입력 유효성** — surface/DPR, 양수 scale, 크기 상한을 확인했다. 0 DPR·NaN·무한 크기 거부 fixture가 통과했다.
6. **기본 모바일 viewport** — meta 부재 시 980 CSS px와 overview scale을 계산하는 고정 Chrome fixture가 통과했다.
7. **width/scale 산식** — device-width, 고정 narrow width 확장, `initial-scale=0.5` 표본이 fixture와 일치한다.
8. **DPR/page-scale 분리** — 결과 구조와 C ABI에서 두 값을 합치지 않는다. native transform 미적용 경계도 명세에 기록했다.
9. **OS와 document scheme 분리** — `prefers-color-scheme`은 OS 입력, meta scheme은 별도 initial document scheme이다. dark metadata + light OS fixture가 통과했다.
10. **scheme keyword와 중복 처리** — `only dark`, supported list 선택, 첫 scheme meta, 잘못된 첫 값의 light fallback을 fixture로 확인했다.
11. **root CSS `color-scheme` output** — computed snapshot에 보존한다. wrapper property projection 누락을 발견해 수정하고 session assertion을 통과시켰다.
12. **typed layout 속성 output** — 새 profile의 `aspect-ratio` extraction 누락을 발견해 element output 경로를 고치고 `Some(2.0)`을 확인했다.
13. **root-only allowlist** — viewport dimension media query와 `color-scheme`은 새 root profile에만 적용된다. default profile 분리 테스트가 통과했다.
14. **media query grammar** — bare feature, dangling `and`, trailing comma, `only (...)` 오수락을 수정했다. malformed query negative cases가 통과했다.
15. **media feature/value 범위** — width/height px·0 및 scheme/pointer/hover만 받는다. orientation·resolution·rem·음수·비허용 property는 거부한다.
16. **cascade revision** — 동일 입력은 revision을 유지하고 viewport/DPR/media 변화는 새 revision을 준다. session test와 Simulator log가 일치한다.
17. **비동기 최신 계산** — 환경 변경은 latest-wins worker 요청을 만들며 setter가 계산 완료를 보장하지 않는다. snapshot 경계를 확인했다.
18. **Android bootstrap race** — 초기 캡처 config를 놓칠 수 있어 session 게시 후 현재 config를 재적용한다. API 37 AVD에서 `0→1→2`를 확인했다.
19. **iOS surface/trait 갱신** — 현재 host view bounds, scale, trait를 읽고 layout/trait 변화마다 갱신한다. iPhone 17 Pro에서 `402×874`, DPR 3, revision `0→1→2`를 확인했다.
20. **FFI 및 통합 검증 한계** — C ABI scheme은 `0=light`, `1=dark`; null 인자 test가 통과했다. Rust FFI test binary 내 V8 생성은 isolate 오류로 제외했으며 양 Simulator 앱에서 C API 호출을 확인했다.

## 발견 및 수정

- Root profile에 `color-scheme` computed output 연결이 빠진 호출 지점을 추가했다.
- 새 computed-style profile에서 `aspect-ratio` typed extraction을 누락한 분기를 추가했다.
- root-only media allowlist가 malformed query를 수락하던 parser를 엄격한 상태 전이로 바꿨다.
- Android bootstrap 중 환경 변경을 놓칠 수 있는 경합을 session 게시 후 현재값 재적용으로 닫았다.
- 위 네 항목은 이 변경에 포함됐고 관련 테스트·Simulator 근거를 남겼다. page-scale transform, Canvas/native theme, HTML layout/paint 및 전체 Chrome metadata parity는 남아 있다.
