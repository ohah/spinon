# C11.4 ASCII letter-word 제한 구현 PR diff 적대 검토 30개

**대상:** PR [#1208](https://github.com/ohah/spinon/pull/1208)의 기능·fixture·문서·시뮬레이터 및 artifact evidence diff.

**기준 diff:** base `7bf86a97dd6ab6192049cf593b7d236c5168ccce`, 검토 head `00fbea486107e062e5f9c0321a777002c83a771d` (155 changed paths). 이 검토는 계획 PR의 diff 검토와 구현/runtime 20개 검토를 대체하지 않는다.

| # | 독립된 diff 실패 관점 | 실제 변경에서 확인한 내용 | 결과 |
|---:|---|---|---|
| 1 | 부분 구현이 공식 완료 checkbox나 수를 올린다 | `spec/STATUS.md`는 diff에 없고 plan·계약은 C11.4/C11 미완료와 `78/214`를 명시한다 | 통과 |
| 2 | 기존 내부 runtime interface의 버전을 범위 추가만으로 올린다 | `RuntimeRootHtmlTextPaintV1` 경로를 유지하고 내부 숫자 버전을 바꾸지 않았다 | 통과 |
| 3 | source gate가 ASCII letters보다 넓게 허용한다 | projection 변경은 모든 token byte에 `is_ascii_alphabetic`을 적용한다 | 통과 |
| 4 | empty source가 `all()`의 빈 참으로 허용된다 | token 존재 조건을 유지하며 empty·whitespace-only input을 거부한다 | 통과 |
| 5 | whitespace separator가 CSS contract 밖 문자까지 포함한다 | SPACE/TAB/LF/CR만 단어 사이 delimiter로 허용한다 | 통과 |
| 6 | Unicode, 숫자, punctuation positive path가 음성 테스트를 통과한다 | 전각문자·NBSP·NEL·em-space 및 `A1`, `A-B`, 기호 negative case를 유지한다 | 통과 |
| 7 | 글자 token이 붙은 text source가 trim 또는 normalize된다 | 수정한 projection은 source를 보관·재작성하지 않고 기존 direct text path를 사용한다 | 통과 |
| 8 | item 하나가 허용된 뒤 다음 item 오류로 partial layout이 게시된다 | Style-to-layout과 RuntimeSession의 invalid second-item rejection을 숫자 token으로 갱신해 atomically 거부한다 | 통과 |
| 9 | expected geometry가 구현 출력에서 역산되어 독립 oracle 역할을 잃는다 | 기존 Chrome observation에서 분리된 `expected.json`의 independent fields를 두 Rust matrix가 읽는다 | 통과 |
| 10 | matrix 일부 case가 실제로 생성되지 않는데 10개로 세어진다 | 두 테스트가 `cases.len() == 10`과 mutation 적용을 확인한다 | 통과 |
| 11 | min-content와 definite auto column 경로가 혼합된다 | 각 scene의 `columnsMode`를 별도 fixture로 적용하고 독립 track 기대값을 비교한다 | 통과 |
| 12 | auto 최소 크기가 40px container 폭으로 clamp된다 | `AAAAAAAA` 48.171875px track과 overflow frame을 포함한다 | 통과 |
| 13 | normal wrapping의 row contribution이 DOM 순서에 따라 틀린 item에 간다 | forward/reverse case에서 long·short item의 위치와 source order를 함께 검사한다 | 통과 |
| 14 | geometry만 비교하고 text paint나 grid track trace를 놓친다 | item frame, line/baseline count, row contributions와 used track list를 분리 검사한다 | 통과 |
| 15 | Text node와 item owner identity가 합쳐지거나 revision이 엇갈린다 | fixture-test trace는 원문 source·Text node identity·owner·generation·document/style/render-tree revision을 대조한다 | 통과 |
| 16 | regression input 변경으로 기존 invalid-token 테스트가 허용 word를 계속 거부하지 못한다 | 기존 `AA`/`A B` 거부를 제거하고 숫자·구두점 negative case로 교체하며 positive word matrix를 새 모듈에 둔다 | 통과 |
| 17 | Android pixel adapter가 기본 앱 source set 또는 공통 MainActivity에 들어간다 | `C114RootHtmlLetterWordPixelCopyAdapter`는 C11 fixture-test source set에만 추가됐다 | 통과 |
| 18 | ServiceLoader 등록이 기존 C11.1 adapter를 덮어쓴다 | service file은 기존 행을 보존하고 C11.4 adapter 행 하나를 추가한다 | 통과 |
| 19 | feature-off Android APK에 run/sample 문자열이 남는다 | 최종 기본 APK의 모든 `classes*.dex` scan에 adapter·task marker 부재가 기록돼 있다 | 통과 |
| 20 | feature-on positive control이 빠져 default 부재가 잘못된 빌드 결과일 수 있다 | feature APK DEX에서 새 adapter와 run/sample marker를 확인한다 | 통과 |
| 21 | iOS 표본 code가 default compile에 포함되거나 다른 run까지 선택한다 | pixel-readback compile define 아래에서만 handler를 추가하고 static run ID를 gate한다 | 통과 |
| 22 | iOS 기본 executable에도 작업별 marker가 남는다 | default Simulator executable marker 부재와 feature-on positive control을 같은 evidence에 기록한다 | 통과 |
| 23 | Android 표본 좌표가 density·surface identity와 맞지 않는다 | CSS sample, density mapping, report surface dimensions 및 exact RGBA를 비교한다 | 통과 |
| 24 | PixelCopy success가 sample count만 확인하고 잘못된 run 또는 stale generation을 읽는다 | run ID를 gate하고 `sourceIsCurrent`·generation·draw/surface identity를 capture에 전달한다 | 통과 |
| 25 | iOS Metal readback을 compositor 또는 실기기 표면 확인으로 과장한다 | README·PR 본문은 이를 iOS Simulator drawable, compositor 제출 전 표본으로 한정한다 | 통과 |
| 26 | 재실행 script가 해당 앱이 이미 실행 중인 상태를 덮어쓰거나 다른 pass evidence를 덮는다 | 시작 전 app process와 target pass 경로를 거부하고 원본 asset을 별도 복사한다 | 통과 |
| 27 | 한 샘플 cache 정리 요청으로 장치 전체 cache/log를 지운다 | review에서 발견한 `adb logcat -c`를 제거했다. 현재 script는 Android/iOS 앱 PID 로그만 수집하며 pass07로 다시 실행했다 | 수정 후 통과 |
| 28 | fixture 실행 뒤 build asset이나 앱 프로세스가 남아 다른 작업에 영향을 준다 | `EXIT` trap이 Android/iOS 실행 앱을 종료하고 bootstrap/iOS bundle asset의 byte-for-byte 복원을 기록한다 | 통과 |
| 29 | 실패한 시도·실제 시뮬레이터 산출물·PR 이미지 링크가 누락되거나 서로 다른 실행을 가리킨다 | pass01–pass07을 분리 보존했고 PR 본문 이미지는 pass07이다. Agent Browser에서 두 raw 이미지 모두 완전 로드와 1080×2400·1206×2622 intrinsic size를 확인했다 | 통과 |
| 30 | PR diff·labels·검증 상태가 공식 완료나 CI 통과로 오인된다 | `enhancement`와 layout/renderer/android/ios labels가 있고 status check는 보고되지 않는다. 공식 수 미변경·CI 미통과 주장을 PR 본문에 명시한다 | 통과 |

## 검토 결과

실제 diff에서 전체 Android `logcat` buffer를 지우던 동작을 발견했다. 이를 제거하고 Android 및 iOS의 현재 앱 PID 로그만 수집하도록 고친 다음 pass07에서 두 Simulator 표본과 asset restoration을 다시 확인했다. 그 외 범위 경계, source admission, text identity, pixel-test feature 분리, artifact scan과 PR 문구에서 수정할 결함을 찾지 못했다.

PR diff review record와 이 파일을 연결하는 README 변경만 추가된 뒤 최종 diff를 다시 확인한다. branch check run은 없고, 이는 CI 통과나 C11.4/C11 완료를 뜻하지 않는다.
