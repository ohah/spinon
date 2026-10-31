# C11.4 `white-space:pre` preserved-tab PR diff 실패 관점 30개

계획 검토 20개와 구현/runtime 검토 20개 이후에 별도로 staged PR diff를 점검했다. 검토 대상은 base `e93663f963171216f93bf54337ccba33bdb46a20` 대비 변경 patch SHA-256 `ccf03dc30097849f876d3468207bbcde6ab3e1537a5372118baa02258499dae6`이다. 이 hash는 이 검토 보고서 파일 자체를 제외한 최종 staged diff를 가리킨다.

| # | Diff 공격 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | 변경이 C11/C11.4 상위 완료 checkbox를 몰래 올리는가 | **PASS** — `spec/STATUS.md`는 C11과 C11.4를 unchecked로 유지하고 `72/213`을 명시한다. |
| 2 | plan 문서가 사전 계획 상태에 머물러 구현을 감추는가 | **PASS** — plan status는 40/40 simulator 결과와 미완료한 후속 PR 점검을 기록한다. |
| 3 | 내부 계약 version을 구현 뒤 조용히 바꾸는가 | **PASS** — 0059와 0070은 기존 `0.1.0` 숫자 버전을 유지한다. |
| 4 | 공개 CSS/API 또는 ABI를 추가하는가 | **PASS** — 변경은 내부 profile과 fixture 경로에 한정된다. |
| 5 | `Pre`가 아닌 whitespace mode로 새 behavior가 흘러가는가 | **PASS** — tab profile 진입은 `Pre` + tab 존재에 한정되고 `PreWrap` gate는 별도다. |
| 6 | 기존 `PreWrap` tab normalization이 회귀하는가 | **PASS** — 별도 profile와 기존 regression tests가 유지되고 simulator의 기존 31 scene도 통과했다. |
| 7 | unit mode에서 탭 폭이 font metrics와 무관한 8 code point로 오인되는가 | **PASS** — profile은 Ahem 10px/10px로 제한해 CSS advance 80px와 같은 결과를 만든다. |
| 8 | exact-stop/tab arithmetic가 독립 기대값 없이 자기 코드만 검증하는가 | **PASS** — plan은 CSS Text 규칙·Chrome 156 관측·독립 geometry를 고정하고 테스트는 그 expected values를 사용한다. |
| 9 | `tab-size:8` computed value가 실제 Stylo snapshot에서 확인되지 않는가 | **PASS** — Stylo test가 initial 8과 explicit 8/2를 확인하고 앱 matrix가 40 scene을 실행했다. |
| 10 | length syntax나 arbitrary tab-size를 일반 기능으로 받아들이는가 | **PASS** — 길이 기반 기능을 추가하지 않고 negative `2`는 anonymous text profile에서 거부한다. |
| 11 | negative 거부가 너무 늦어 layout 일부가 publish되는가 | **PASS** — 두 simulator 결과는 expected error 1/1, frame/trace/presentation success 0이다. |
| 12 | tab expansion이 source UTF-16 offset을 output byte offset으로 잘못 복사하는가 | **PASS** — range validator는 전체 확장 길이와 단위별 offset을 비교한다. |
| 13 | joined text source의 node ownership이 바뀌는가 | **PASS** — Rust split-source test는 tab node와 original code-unit range를 고정한다. |
| 14 | forced break 및 CRLF가 tab cursor나 source map을 오염시키는가 | **PASS** — unit tests와 `forced-break` runtime trace는 줄별 reset/CRLF mapping을 확인한다. |
| 15 | 제한 외 character, font 또는 inline hierarchy가 partial success가 되는가 | **PASS** — `UnsupportedPreTabProfile` regression tests가 intrinsic와 final layout을 모두 거부한다. |
| 16 | min-content나 narrow fixed track에서 `pre`가 wrap되는가 | **PASS** — min-content 100px와 25px track의 unwrapped line advance 100px를 양 simulator validator가 검사한다. |
| 17 | scene fixture가 기존 scene ID를 재사용하거나 플랫폼 간 개수가 달라지는가 | **PASS** — Rust runtime, FFI, capture catalog는 40개 고유 ID를 고정한다. |
| 18 | FFI scene payload에서 negative scene의 `tab-size:2` 값이 누락되는가 | **PASS** — FFI source test는 positive 8과 negative 2를 직접 검사한다. |
| 19 | simulator catalog count만 바꾸고 실행 matrix 선택은 빠뜨리는가 | **PASS** — module-boundary Node test 24/24가 657 total catalog와 anonymous 40-scene 선택을 검사한다. |
| 20 | Android와 iOS 중 한쪽이 tab scene/error code를 빠뜨리는가 | **PASS** — 양 adapter의 allowlist, expected frames, source text, negative diagnostic이 같은 scene IDs를 갖고 실제 matrix가 통과했다. |
| 21 | contribution trace가 min-content·fixed-width scene에서 다른 category를 선택하는가 | **PASS** — capture verifier는 `MinContent`와 25px fixed-track 경계를 명시적으로 확인한다. |
| 22 | trace report의 새 `normalizedSources` field가 제품 ABI로 새어 나가는가 | **PASS** — FFI fixture `trace_report` 내부 JSON이며 feature-enabled simulator 경로에서만 생성된다. |
| 23 | fixture ABI 및 scene string이 default FFI archive에 포함되는가 | **PASS** — default archive는 새 ID 0/9·정의 symbol 0이고 opt-in archive만 9/9·2 symbols다([archive proof](./artifact-boundary.md)). |
| 24 | default/opt-in test가 같은 overwrite artifact를 잘못 비교하는가 | **PASS** — Cargo feature fingerprint가 다른 두 `target/debug/deps` archive hash와 size를 비교했다. |
| 25 | capture가 기존 상대 simulator 또는 실기기로 실행되는가 | **PASS** — Android target은 `emulator-5554` QEMU AVD이고 iOS target은 지정 iPhone 17 Pro Simulator UDID다. |
| 26 | 설치물이 다른 build output일 가능성을 숨기는가 | **PASS** — Android APK와 iOS app executable 모두 install 전후 SHA-256이 일치한다. |
| 27 | superseded run을 성공으로 세거나 retry를 누락하는가 | **PASS** — 두 result 모두 retry 0이며 40 scene 각각의 final result를 기록한다. |
| 28 | 화면 캡처를 glyph ink나 최종 compositor pixel 증거로 과장하는가 | **PASS** — README는 C15.3 glyph paint가 꺼진 blank surface와 iOS compositor 한계를 명시한다. |
| 29 | raw log에 포함된 trailing spaces 때문에 `git diff --check`가 실패하는가 | **PASS** — 원본 로그는 수정하지 않고 gzip tar archive로 보존하며 staged diff check가 통과한다. |
| 30 | 변경 근거가 없어지거나 불필요한 저장소 cleanup이 포함되는가 | **PASS** — 201 raw log files는 archive에, 78 PNG·result metadata와 hashes는 별도 보존했다. 무관한 snapshot/cache/worktree는 건드리지 않았다. |

## 최종 재확인

- `git diff --cached --check` — 통과.
- `cargo fmt --all -- --check`와 workspace 및 fixture-feature Clippy — 통과.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator — 각각 40/40 통과, draw/present 39/39, retry 0.
- 이 review artifact 자체는 위의 subject patch hash를 만든 뒤 추가했다. merge 전 GitHub PR head를 다시 가져와 product-and-evidence patch가 위 검토 대상과 일치하는지 비교한다.

이는 제한 fixture candidate의 PR diff 검토다. 전체 CSS Grid, 전체 WPT, hardware device/GPU, compositor pixel 또는 C11 완료 판정이 아니다.
