# C11.4 `safe|unsafe end` PR 변경분 적대 검토 30개

구현·테스트·계약·상태 대장·로드맵·실행 evidence의 최종 변경 후보를 서로 다른 실패 관점으로 검토했다. 각 항목은 이 PR diff의 실제 코드 또는 근거 경계를 확인한다.

| # | 검토 관점 | 확인 결과 |
|---:|---|---|
| 1 | computed `safe end`가 다른 enum variant로 투영되는가 | `End + Safe` typed 조합을 확인하고 layout 기대 좌표가 일치한다. 통과. |
| 2 | computed `unsafe end`가 safety modifier 없이 투영되는가 | `End + Unsafe`를 별도 허용하고 overflow end 위치를 유지한다. 통과. |
| 3 | exact two values 외 allowlist가 넓어졌는가 | auto-row fixed profile에 두 exact string만 추가됐다. 통과. |
| 4 | positive free space에서 safe 값이 start fallback하는가 | Chrome·독립 계산·Rust 모두 `[10,20,30]px`. 통과. |
| 5 | positive free space에서 unsafe 값이 잘못 이동하는가 | Rust geometry가 `[10,20,30]px` control과 일치한다. 통과. |
| 6 | overflow에서 `safe end`가 unsafe end처럼 역방향 배치되는가 | safe fallback `[0,56,112]px`를 layout, projection, runtime, Simulator에서 확인. 통과. |
| 7 | overflow에서 `unsafe end`가 safe fallback으로 잘못 바뀌는가 | 기대 `[-82,-26,30]px`, screenshot의 마지막 visible item과 일치. 통과. |
| 8 | plain `end` regression이 추가 허용값으로 바뀌는가 | positive·overflow plain end controls가 기대 frame을 유지한다. 통과. |
| 9 | alignment가 auto-track sizing을 수정하는가 | 세 row는 계속 `[10,10,10]px`; layout trace에서 확인. 통과. |
| 10 | row-gap `%` basis가 잘못되거나 rounding되었는가 | definite 40px block basis에서 two-term calc가 46px로 resolve된다. 통과. |
| 11 | gutters가 정렬값에 따라 변하는가 | positive는 `[0,0]`, overflow는 `[46,46]px`로 모든 end control이 동일하다. 통과. |
| 12 | 기존 center safety가 회귀하는가 | safe `[0,56,112]`, unsafe `[-41,15,71]` Rust/runtime regressions 통과. |
| 13 | 미지원 `safe start`가 조용히 기본값 처리되는가 | style projection이 명시 거부하고 RuntimeSession은 partial result를 내지 않는다. 통과. |
| 14 | 미지원 `unsafe start` 오류가 다른 property로 귀속되는가 | 발견한 `grid-auto-rows` 오귀속을 수정; 현재 `align-content` 및 실제 값 반환. 통과. |
| 15 | modifier 순서 변형이나 unsupported combination이 허용되는가 | `center safe`, `safe end center`, `unsafe center end`와 distributed safety control은 reject. 통과. |
| 16 | baseline 또는 unknown serialization이 auto-row 경로로 들어오는가 | 기존 negative allowlist controls 유지. 통과. |
| 17 | 일반 Root HTML content-alignment 경로가 넓어지는가 | 전역 parse 또는 generic projection 수정 없이 profile helper만 변경했다. 통과. |
| 18 | legacy `RootHtmlIntrinsicTrackSizingV1`도 안전 값 허용으로 바뀌는가 | mode 제한을 유지하고 기존 일반 profile 회귀를 보존했다. 통과. |
| 19 | 40×40 definite content box 이외 shape가 지원된다고 기록되는가 | 명세·fixture·runtime gate가 계획 profile의 고정 shape와 일치한다. 통과. |
| 20 | text children의 DOM order 또는 source provenance가 변하는가 | fixed empty item 뒤 direct text `A`/`B`, source order를 유지했다. 통과. |
| 21 | safe-start rejection이 partial layout snapshot을 publish하는가 | RuntimeSession의 rejected state에서 `completed=None`을 검사한다. 통과. |
| 22 | rejected state 이후 동일 session이 다시 동작하는가 | light/supported → dark/unsupported → light/supported 순서로 recovery 확인. 통과. |
| 23 | style projection recovery 테스트가 actual parsed Root HTML을 우회하는가 | RuntimeSession 검사는 기본 Root HTML route로 cascade와 layout을 실행한다. 통과. |
| 24 | 앱 캡처가 opt-in fixture FFI를 켜야 성공하는가 | Android/iOS pass metadata에서 모든 C11 fixture flag 비활성. 통과. |
| 25 | Android 캡처가 지정 API/ABI 또는 app을 잘못 썼는가 | API 37 ARM64 AVD `emulator-5554`, 기본 app route, `layout=ready` 6 boxes. 통과. |
| 26 | iOS 캡처가 다른 시뮬레이터 또는 route를 썼는가 | iPhone 17 Pro/iOS 26.2 UDID와 기본 Root HTML route 기록. 통과. |
| 27 | 화면 밖 unsafe item이 없는 것처럼 판정되는가 | analyzer는 화면 밖 item을 `null`로 분리하고 Rust negative-y geometry를 별도 검증. 통과. |
| 28 | screenshot 경계가 native surface readback으로 과장되는가 | 문서·PR 화면 설명은 visual bounds로 한정하고 PixelCopy/Metal은 수행하지 않았다. 통과. |
| 29 | 시뮬레이터 실행이 설치 앱·bootstrap 또는 build를 오염시키는가 | pass04–06 모두 앱/산출물 복원 hash 일치와 restoration PASS 기록. 통과. |
| 30 | 공식 C11 상태·전체 counter 또는 현재 순서가 잘못 승격되는가 | C11/C11.4 체크는 미완료, `78/214` 유지; safe-end를 현재 bounded candidate로 표시하고 PR #1252 open 상태를 연결한다. 통과. |

한 가지 검토 발견은 #14이며 style projection 오류 attribution을 고친 뒤 새 property-specific regression을 통과시켰다. 이 review는 구현 diff 검토이며 CI, WPT, physical device·hardware GPU, PixelCopy/Metal readback 또는 C11 완료를 주장하지 않는다.
