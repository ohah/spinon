# C11.1 parsed Root HTML integer `repeat()` PR diff 적대 검토 30개

대상은 [PR #1038](https://github.com/ohah/spinon/pull/1038)의 전체 코드·fixture·명세·실행 근거·공식/공개 로드맵 변경이다. 항목마다 별도 실패 경로, 불변 조건 또는 공개 주장 경계를 대조했다. 계획 실패 관점 검토와 구현/runtime 검토를 재사용하지 않았다.

| # | 공격 관점 | diff·근거 대조 및 판정 |
|---:|---|---|
| 1 | repeat 허용이 default Root HTML 외 프로필로 새는가 | `element.rs`에서 새 허용은 `RuntimeRootHtmlTextPaintV1` 옵션 한 곳에 한정된다. 통과. |
| 2 | 기존 C11.3/track-sizing 프로필의 기존 admission 값이 바뀌는가 | 기존 두 프로필은 기존대로 integer repeat 허용이고, `allow_intrinsic_tracks`·`allow_auto_repeat` 값은 이 diff에서 바뀌지 않았다. 통과. |
| 3 | 정수 repeat 허용이 `auto-fill`/`auto-fit`까지 연다 | default Root HTML runtime rejection에서 `auto-fill`은 `Failed`, 진단 존재, completed layout 부재다. 통과. |
| 4 | repeated `minmax()`나 intrinsic track이 direct-size subset을 우회한다 | `repeat(2, min-content 50px)` 및 `repeat(2, minmax(0px,50%))` negative controls가 거부된다. 통과. |
| 5 | CSSOM 문자열을 adapter가 직접 재파싱하는가 | 변경된 기본 구현은 cascade의 typed expanded tracks를 기존 projection으로 전달한다. adapter 재파싱 코드는 추가되지 않았다. 통과. |
| 6 | percentage basis를 definite하지 않은 축이나 반대 축에서 가져오는가 | 기준에는 200px definite columns와 100px definite rows가 분리되어 있고 percentage expected frame을 독립 산술과 비교한다. 통과. |
| 7 | 반복 pattern 안의 track source order가 바뀌는가 | `[30,50,30,50]`, `[20,30,20,30]`, 혼합 percentage/fraction order를 typed vector와 item 누적 frame으로 확인한다. 통과. |
| 8 | 여러 integer repeat group 사이의 direct track이 소실되는가 | `repeat(2,20px) 10px repeat(2,30px)`에서 expected `[20,20,10,30,30]`와 각 item 위치가 일치한다. 통과. |
| 9 | repeat count 1에서 pattern이 누락되거나 중복되는가 | `repeat(1,30px 50px)` control은 정확히 두 typed tracks 및 item 위치를 유지한다. 통과. |
| 10 | expansion이 selector, tag, class, ID 또는 고정 NodeId에 종속되는가 | 같은 computed 입력에서 tag/class/ID와 selector를 바꾸고 source order 역전 control을 확인한다. 통과. |
| 11 | DOM source order를 repeat expansion이 덮어쓰는가 | reverse-order control의 네 semantic item이 변경된 DOM order에 맞는 각 예상 frame에 배치된다. 통과. |
| 12 | invalid `repeat(0, …)`가 앞선 유효 declaration을 지우는가 | valid direct tracks 뒤의 invalid zero-count declaration control이 원래 tracks와 전체 geometry를 보존한다. 통과. |
| 13 | 명시 track 수가 10,000 경계에서 off-by-one으로 거부되는가 | style cascade에서 `repeat(10000,1px)` 결과 10,000 tracks 및 10,001 empty line groups를 직접 검사한다. 통과. |
| 14 | 10,001 expanded tracks를 잘라 성공하거나 부분 scene을 내는가 | `repeat(10001,1px)`는 runtime 실패와 진단을 남기며 completed layout/scene을 게시하지 않는다. 통과. |
| 15 | track cap 이내에서 repeated line-name byte metadata quota를 우회하는가 | 10,000회·128-byte 이름 입력은 기존 1 MiB quota 경계를 넘어 fail closed한다. 통과. |
| 16 | track vector reservation 실패가 잘못된 property/node 문맥을 남기는가 | `Tracks` failpoint는 `grid-template-columns`와 해당 root node의 `GridAllocationFailure`를 반환한다. 통과. |
| 17 | line-group reservation 실패 뒤 부분 결과가 남는가 | `LineGroups` failpoint가 contextual error를 내며 동일 view 재시도에서 전체 metadata가 복구된다. 통과. |
| 18 | line-name vector/text reservation 실패가 panic, 절단 문자열 또는 stale state를 남기는가 | `LineNames` 및 `LineNameText` failpoint를 각각 실행하고 오류 후 전체 이름 순서 재계산을 확인한다. 통과. |
| 19 | failpoint가 다음 테스트나 재계산까지 남아 cascade를 오염시키는가 | 테스트 전용 thread-local failpoint은 RAII guard drop에서 해제되고 각 주입 뒤 같은 thread의 재계산을 성공시킨다. 통과. |
| 20 | named authored lines가 기존 미지원 경계를 넘어 layout으로 노출되는가 | repeat 내부 named line은 runtime projection에서 거부되며 scene이 없다. cascade allocation test의 typed metadata 보존은 app admission을 주장하지 않는다. 통과. |
| 21 | `grid-template-areas`의 empty tracks 추론이 새 프로필로 우회되는가 | areas + `none` control은 완전 cascade를 보존하지만 Root HTML projection에서 failed이며 completed scene은 없다. 통과. |
| 22 | cascade와 layout의 서로 다른 revision/key가 합쳐지는가 | seven-scene positive matrix에서 cascade 완료 key와 layout 완료 key의 동일성을 확인한다. 통과. |
| 23 | html/body transparent wrapper가 item box로 잘못 paint되거나 사라지는가 | scene의 `RuntimePaint::None` box는 wrapper 두 개뿐이며 전체 box 수는 37로 고정한다. 통과. |
| 24 | 기존 non-repeat direct tracks와 주변 Grid 기능이 회귀하는가 | repeat를 펼친 동등 direct-track control과 Root HTML regression, style/style-to-layout regression 및 scoped Clippy가 통과한다. 통과. |
| 25 | Chrome 관측값을 독립 expected 계산값으로 오인하거나 장면/track/item 수를 잘못 적는가 | 보존 JSON을 별도 비교해 7 scenes, 35 column+row tracks, 28 items를 확인했고 plan·STATUS·PR 숫자를 35로 맞췄다. 통과. |
| 26 | Android와 iOS가 서로 다른 fixture revision을 실행하는가 | 두 bundled `index.html` hash가 fixture SHA-256 `f2d0d35e…12ccd5d`와 같고 양측 runtime은 7 scenes/37 boxes다. 통과. |
| 27 | Android 캡처가 fixture build flag나 다른 route의 결과인가 | Android API 37 ARM64 AVD APK hash와 build/runtime logs를 보존했고 C11 task fixture flags는 꺼져 있다. SwiftShader 경로임을 명시한다. 통과. |
| 28 | iOS 캡처가 다른 binary/fixture 또는 실제 기기 증거처럼 보이는가 | iPhone 17 Pro/iOS 26.2 Simulator binary/bundle hash와 runtime marker를 보존하고, Simulator Metal·draw/present 범위만 주장한다. 통과. |
| 29 | screenshots가 표면 pixel readback이나 Chrome parity로 과장되는가 | PR body·evidence·roadmap 모두 캡처를 visual smoke로 한정하고 Android PixelCopy, Metal drawable readback, Chrome pixel equality를 주장하지 않는다. 통과. |
| 30 | 부분 구현이 공식 C11 완료·공개 CSS 적합성·CI 성공으로 집계되는가 | `spec/STATUS.md`, C11 계획, 공개 roadmap과 PR body가 C11 미완료·`72/213` 유지 및 no-checks 상태를 기록한다. PR은 draft이며 labels와 Android/iOS inline image URLs가 확인됐다. 통과. |

## 재확인 결과

- `cargo fmt --all -- --check`: 통과.
- 이전 기록에서 `root_html_grid` 30/30, style Root HTML 13/13, style-to-layout Root HTML 15/15와 scoped Clippy 통과를 확인했다. 이후 Rust 코드는 바뀌지 않았다.
- PR #1038에는 GitHub checks가 보고되지 않았다. CI 통과로 간주하지 않는다.
- 이 검토에서 발견한 문서 bullet 들여쓰기 차이는 수정한 뒤 `git diff --check`로 재확인했다. 남은 blocking finding은 없다.
