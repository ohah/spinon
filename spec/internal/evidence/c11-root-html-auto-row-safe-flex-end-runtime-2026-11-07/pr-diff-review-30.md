# C11.4 `safe|unsafe flex-end` PR #1260 diff 실패 관점 30개

PR #1260의 base `11e4b04e022365445d717cea0dcf8c56d872038d`부터 구현 commit `1bc9a21fc72a1426061337e63bdb323e714b70d9` 및 증거·상태 문서 commit `fcef14aeb`까지의 코드·fixtures·문서·Simulator artifacts·Tailnet sync를 검토했다. 계획 실패 관점 20개와 구현/runtime 실패 관점 20개와 별도로 기록한다. 이어진 `569b9f9814feaa808672eac9ec3c5c01b3121bd9`는 이 검토 문서와 링크만 추가했으며 runtime 코드는 바꾸지 않았다.

| # | 독립 검토 관점 | 판정 |
|---:|---|---|
| 1 | Computed CSS admission이 `safe|unsafe flex-end` 두 값에만 열리는가 | `RootHtmlAutoRowStretchV1` candidate의 exact computed-value match에 두 문자열만 추가했다. parser/global CSS admission은 바꾸지 않았다. 통과. |
| 2 | Style projection과 Layout allowlist의 typed 값 집합이 일치하는가 | Projection의 두 computed 문자열은 Layout의 `FlexEnd + Safe/Unsafe` 두 typed variant로 연결된다. 통과. |
| 3 | 새 허용값이 다른 placement mode로 새어 나가는가 | Layout predicate의 추가 match는 `RootHtmlAutoRowStretchV1` 분기에만 있다. 통과. |
| 4 | Baseline/unknown/잘못 조합된 positional 값이 함께 허용되는가 | baseline 및 malformed/unknown controls는 닫혀 있고 error owner는 `align-content`다. 통과. |
| 5 | 기존 negative control이 새 positive 구현으로 무효화됐는가 | 기존 safe center/start/end rejection controls에서 이제 지원되는 `safe flex-end`를 `baseline`으로 교체했고 전용 support matrix를 따로 추가했다. 통과. |
| 6 | Plain `end` 또는 다른 axis/속성의 allowlist가 바뀌는가 | `align-content` fixed profile 외에는 변경이 없다. `justify-content`, `justify-items`, `align-items` 값은 그대로다. 통과. |
| 7 | Positive space에서 safe flex-end가 end처럼 동작하지 않는가 | Chrome/independent matrix의 `[10,20,30]px` offsets를 layout·projection·runtime에서 확인한다. 통과. |
| 8 | Positive space에서 unsafe modifier가 잘못 분기하는가 | `unsafe flex-end`도 positive control `[10,20,30]px`와 computed serialization을 보존한다. 통과. |
| 9 | Overflow에서 safe fallback이 음수 위치로 나가는가 | Safe expected offsets `[0,56,112]px`; Rust frames와 두 Simulator pixel bounds가 일치한다. 통과. |
| 10 | Overflow에서 unsafe alignment가 safe fallback으로 바뀌는가 | Unsafe expected offsets `[-82,-26,30]px`; Rust frames가 전체를 검사하고 screenshot은 visible item만 검출한다. 통과. |
| 11 | Row-gap percentage/math가 viewport나 inline axis basis를 쓰는가 | Definite 40px block basis의 `calc(120% - 2px)` 46px를 기존 bounded profile에서 사용한다. 통과. |
| 12 | Content alignment이 track base size를 바꾸는가 | 모든 matrix scene의 auto rows `[10,10,10]px`, gutters `[0,0]` 또는 `[46,46]px`를 검사한다. 통과. |
| 13 | Matrix가 y만 비교해 너비·높이·x 이동을 놓치는가 | Layout/RuntimeSession의 72 frame coordinates에서 x, y, width, height를 허용오차 내 대조한다. 통과. |
| 14 | Computed style snapshot이 safety modifier를 정규화해 지우는가 | Style projection이 computed `align-content` exact string을 각 scene에서 검사한다. 통과. |
| 15 | Cascade와 Layout 결과가 다른 revision에서 섞이는가 | RuntimeSession test가 completed snapshot key와 cascade key equality를 검사한다. 통과. |
| 16 | Unsupported style 뒤 stale/partial snapshot이 보이는가 | Dark media baseline rejection에서 Failed와 `completed=None`을 확인한다. 통과. |
| 17 | 같은 RuntimeSession이 invalid result 뒤 회복하지 못하는가 | Light media로 돌아와 Ready completed snapshot을 재발행한다. 통과. |
| 18 | 기존 safe center/start/end/flex-start matrix가 함께 회귀하는가 | Changed-crate focused regression filters는 Layout 9/9, projection 6/6, RuntimeSession 9/9다. 통과. |
| 19 | Runtime fixture가 styling JavaScript나 fixture-only branch로 geometry를 만든다고 오해하는가 | Fixture는 기본 parsed Root HTML과 고정 CSS box/text를 사용한다. Fixture C11 flags는 꺼진 상태로 실행한다. 통과. |
| 20 | Simulator script가 물리 Android 기기를 선택할 수 있는가 | Script는 `emulator-5554`에 고정되고 pass 실행 시 ADB 목록에는 해당 AVD만 있었다. 통과. |
| 21 | Android device profile이 임의 API/ABI로 바뀌는가 | Preflight가 API 37과 `arm64-v8a`를 단언하고 device properties를 pass마다 남긴다. 통과. |
| 22 | iOS pass가 잘못된 simulator/device에서 실행되는가 | Runner는 고정 iPhone 17 Pro UDID와 Booted 상태를 확인하고 iOS 26.2 device list를 기록한다. 통과. |
| 23 | Missing V8 failure가 test result와 섞이거나 숨겨지는가 | `pass01` source checkout missing, `pass02` Android archive missing을 그대로 보존하고 성공 pass03–05와 분리한다. 통과. |
| 24 | 실패/성공 pass가 설치 앱을 실행 중인 채 바꾸거나 original bootstrap을 덮어쓰는가 | Running app을 거부하고 bootstrap input, build outputs와 설치 앱을 marker-protected temp backup으로 복원한다. 통과. |
| 25 | 복원 성공 표기가 hash와 무관하게 찍히는가 | 복원 APK/executable SHA가 원본과 일치해야 `asset_and_simulator_restoration=PASS`가 된다. 성공 세 pass 모두 통과. |
| 26 | Screenshot analyzer가 candidate screenshot에서 임의 위치를 root origin으로 잡는가 | Positive pass03의 Grid background top을 origin으로 사용하고 Grid 폭도 40 CSS px인지 검사한다. 통과. |
| 27 | Offscreen unsafe items가 screenshot에 없다는 사실을 geometry failure로 보거나 pixel 증거로 과장하는가 | 두 negative frames는 Rust coordinate result로 확인하고 OS screenshot의 visible item만 기록한다. 통과. |
| 28 | Official status나 roadmap이 미완료 C11을 완료로 계산하는가 | `STATUS.md`, plan, contract와 public roadmap은 C11.4/C11 미완료 및 `78/214`를 유지한다. Tailnet도 source hash와 동일하다. 통과. |
| 29 | Tailnet copy가 이전 제공본 없이 교체되거나 잘못된 서버/URL을 수정하는가 | Script는 기존 hash/served bytes, Tailscale backend, listener, server command를 preflight하고 pass별 백업 및 same-directory atomic rename을 사용한다. 통과. |
| 30 | PR screenshot/CI 문구가 evidence 범위를 넘는가 | PR #1260 본문에 Android/iOS의 positive·safe overflow·unsafe overflow 화면 6장을 넣었다. Agent Browser에서 모두 `complete=true`, 원본 크기 Android 1080×2400·iOS 1206×2622로 렌더됨을 확인했다. PixelCopy/Metal readback, physical device, WPT, CI checks를 증명했다고 쓰지 않는다. 통과. |

검토한 변경에서 수정 필요 blocker를 찾지 못했다. Agent Browser에서 PR #1260 본문 이미지 6/6의 실제 렌더를 확인했다. GitHub check rollup은 별도 확인 전까지 CI 통과로 표시하지 않는다.
