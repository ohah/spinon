# Negative `text-indent` PR diff 실패 관점 검토 30개

대상은 `feat/c11-4-negative-text-indent-20261101`의 현재 변경 diff이며 기준 commit은 `9b6b527ab4b243c842827fa870609f6bfca4a99a`다. 계획 PR diff 검토 30개와 구현/runtime 검토 20개를 반복하지 않고, Rust allowlist·layout 수식·Android/iOS 기대값·capture validator·artifact boundary·상태 문서를 서로 다른 오류 관점에서 확인했다. PR diff 검토 중 발견한 Android max-content advance 기대값 오류는 수정 후 영향을 받는 검사와 AVD matrix를 다시 실행했다.

| # | 실패 관점 | 최종 확인 |
|---:|---|---|
| 1 | negative scene이 기본 제품 feature에 포함되는가 | default/opt-in static archive 비교에서 기본 archive는 0 scene·0 symbol, positive control은 30 scene과 adapter/validator symbol을 포함한다. APK/IPA 전체 배제라고 표현하지 않는다. |
| 2 | FFI source generator와 Rust runtime allowlist 길이가 어긋나는가 | 둘 다 30 unique ID를 갖고 fixture Node test 및 focused Rust test가 통과했다. |
| 3 | 기존 positive scene이 negative 전환 중 삭제·재정의되는가 | 기존 18개 scene과 결과를 회귀 행렬에 유지했고 Android/iOS 각 30/30이다. |
| 4 | scene 이름과 다른 computed value를 교차 입력할 수 있는가 | expected scene ID와 exact serialized value/typed value를 함께 검사한다. 불일치 입력 runtime test가 layout 전에 거부된다. |
| 5 | 임의의 범위 내 negative px가 수치 parser만 통과해 허용되는가 | exact allowlist 밖 `-7px`와 미등록 negative ID를 runtime test가 거부한다. |
| 6 | `-30px` 경계를 구현 입력으로 오인하는가 | `-30px`는 oracle-only로 남고 구현 mismatch control에서 layout 전에 거부된다. |
| 7 | -10/10 경계 밖 finite value가 통과하는가 | parser는 `[-10,10]`으로 제한하고 `-11px`·`11px`을 거부한다. |
| 8 | `-0px`, 소수 표기, plus sign, whitespace가 canonical output을 우회하는가 | exact computed serialization 비교와 parser tests가 비정규형을 거부한다. |
| 9 | percentage·다른 unit·`hanging`·`each-line` suffix를 px로 자르는가 | exact `px` parser와 invalid value tests가 percentage 및 keyword control을 거부한다. |
| 10 | NaN/infinity가 `f32` 변환 후 layout에 유입되는가 | finite/range 검사와 `NaNpx`·`infpx` controls가 모두 거부된다. |
| 11 | stale 또는 다른 HostDocument/style snapshot의 값이 재사용되는가 | runtime matrix의 source revision과 text intrinsic trace revision을 비교한다. 30 scene이 통과했다. |
| 12 | min-content 계산이 음수 indent를 모든 단어에 더하는가 | 첫 unbreakable word에만 indent를 더하고 각 later word를 floor로 유지한다. 독립 Ahem 공식과 runtime/capture 결과가 일치한다. |
| 13 | 긴 첫 단어와 짧은 첫 단어의 min-content가 같은 잘못된 경로를 타는가 | `XXX X`와 `X XXX`를 각각 negative 5/10px로 검사했고 later-word floor 30px를 보존한다. |
| 14 | single-word min-content가 negative indent를 무시하거나 두 번 더하는가 | `XX` min-content가 -5px에서 15px, -10px에서 10px이며 양 플랫폼 matrix와 일치한다. |
| 15 | max-content에서 indent가 word마다 적용되는가 | 전체 word/separator advance에 indent를 정확히 한 번 더한다. negative two-word expected values와 trace가 일치한다. |
| 16 | collapsed positive regression에서 공백 정규화가 intrinsic contribution을 바꾸는가 | 기존 raw text와 normalized text를 별도로 보존하고 기존 18 scene의 min/max와 row 결과를 재검사한다. |
| 17 | first-line indent가 line advance에 합쳐져 후속 line까지 이동하는가 | Rust text layout은 indent와 per-line advances를 따로 검사하며 first-line position과 각 line advance가 독립 기대값과 일치한다. |
| 18 | 두 단어 min-content wrap에서 행 수·높이가 바뀌는가 | `XXX X`/`X XXX`는 2 lines·20px, max-content는 1 line·10px를 양 플랫폼에서 검사한다. |
| 19 | 선택된 min/max contribution과 최종 track/frame이 다른 값을 쓰는가 | capture validator는 intrinsic input, item frame, Columns/Rows 최종 stage를 각각 독립 expected value와 비교한다. |
| 20 | text item의 available width를 max-content text advance와 혼동하는가 | trace는 `availableWidth`와 text layout width를 별도로 확인하고 30 scene 모두 통과했다. |
| 21 | Ahem font, size, line-height 또는 writing mode 변경이 상수 기대값을 우회하는가 | 고정 Ahem 10px, line-height 10px, horizontal-tb/LTR 조건을 style trace에서 확인한다. |
| 22 | Android FFI scene ID가 scene source generator와 일치하지 않는가 | 12 negative ID가 positive control 및 runtime allowlist와 대조되고 30-scene API 37 capture가 모두 통과했다. |
| 23 | Android expectation array index가 `minContentHeight` 포함 순서와 어긋나는가 | 10-field tuple을 runtime에서 검사했다. 처음 발견한 index 경계 오류는 수정 후 fixture build와 30/30 capture를 통과했다. |
| 24 | Android max-content의 한 줄 advance가 첫 단어 폭으로 예상되는가 | 검토 중 adapter의 잘못된 expectation을 발견했다. min-content만 첫째/둘째 word advances를 쓰고 max-content는 전체 unindented advance를 사용하도록 수정했다. static test와 새 AVD 30/30에서 최종 확인했다. |
| 25 | Android viewport environment가 고정 fixture scale을 덮어쓰는가 | C11.4 fixture environment 분기에서 scale 1.0을 유지하며 API 37 AVD 전체 실행이 통과했다. |
| 26 | Android capture가 app 시작 직후 PID를 찾지 못해 scene을 누락하는가 | capture runner는 explicit launch 결과 뒤 최대 10초 PID polling을 하고 최종 run에서 30 scene 로그를 수집했다. |
| 27 | iOS adapter에서 raw/normalized text 또는 per-line advance 순서가 뒤집히는가 | Swift adapter의 30-scene expectation은 raw/layout text, min-content 각 line advance, max-content 전체 advance를 각각 확인한다. iOS 26.2 Simulator matrix는 30/30이다. |
| 28 | 실행 대상이 잘못된 Android device 또는 iOS Simulator인가 | evidence device metadata는 API 37, ARM64, `qemu=1` AVD와 고정 iPhone 17 Pro/iOS 26.2 Simulator UDID를 기록한다. 실기기라고 주장하지 않는다. |
| 29 | 이전 앱 hash나 실패 실행이 최종 녹색 결과에 섞이는가 | 최종 Android APK/install hash 일치와 새 capture log를 확인했다. 앞선 Android run은 최종 결과에서 제외하고 iOS 근거는 별도 기록한다. |
| 30 | 문서가 제한 fixture 결과를 C11 지원/완료나 공식 집계 증가로 부풀리는가 | plan·내부 계약·`spec/STATUS.md`는 opt-in slice로 기록하며 C11/C11.4 checkbox와 72/213을 그대로 둔다. glyph pixels/WPT/실기기/hardware GPU 한계도 명시한다. |

최종 diff 검사는 통과했다. 발견된 Android expectation 오류를 수정하고 영향받는 CSS reference static test, fixture app build, Android API 37 AVD 30-scene capture를 다시 수행했다. 이 문서는 bounded fixture PR diff 검토이며 전체 C11 완료나 GitHub CI 결과를 뜻하지 않는다. 구현 PR #938은 2026-10-31T18:34:54Z Rebase and merge됐다(merge SHA 8c5ca0c62ed030642455f53122928b25bba80fda).
