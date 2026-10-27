# C11.4 행 축 overlapping-span runtime slice PR diff 실패 관점 검토 30회

**대상:** 계획·fixture source·Android/iOS matcher와 capture·artifact 증거·내부 계약·공식 상태 문서 전체 diff
**판정:** 아래 30개의 별도 경계와 실패 경로를 검토했다. 중복된 STATUS 기록과 scanner 경로·staging 경계 문제를 수정하고 다시 확인했다.

| # | 검토 관점 | diff에서 확인한 조건과 판정 |
| ---: | --- | --- |
| 1 | C11.4 부분 fixture가 일반 CSS 지원으로 게시됨 | 계획·evidence·0059 모두 opt-in 고정 fixture 한정이며 author CSS admission 확대가 아니다. **통과** |
| 2 | C11.4 또는 C11 상위 완료 상태가 바뀜 | 두 checkbox를 미완료로 유지한다. **통과** |
| 3 | 공식 완료 수가 부분 runtime slice 때문에 증가 | `72/213`을 그대로 유지한다. **통과** |
| 4 | 내부 계약 숫자 버전이 기능 추가만으로 상승 | 기존 `0059` 버전 `0.1.0`을 유지한다. **통과** |
| 5 | 계획의 독립 oracle 없이 runtime 숫자를 정답으로 취급 | 계획은 CSS Grid 2025 CRD 유리수 기대값과 기존 Rust stage oracle을 연결하며 Chrome은 final-frame 교차 확인으로만 둔다. **통과** |
| 6 | 계획 입력과 실제 app HTML이 다른 CSS를 사용 | root/item CSS와 scene IDs가 고정되며 `grid-column:1 / 2`를 명시한다. 발견된 implicit-column 입력 결함도 기록했다. **통과** |
| 7 | 계산 한계를 넘어선 intrinsic 지원을 주장 | text/image, auto minimum, gap, implicit row, 다른 sizing function과 전체 Stage B를 제외한다. **통과** |
| 8 | 계획 review를 구현 review로 재사용 | 별도 계획 20회, 구현/runtime 20회, PR diff 30회 파일을 분리했다. **통과** |
| 9 | Rust dispatcher가 다른 C11.4 fixture를 바꿈 | 새 하위 모듈만 등록하고 기존 dispatcher 반환 순서·scene handler는 유지한다. **통과** |
| 10 | Android와 iOS scene allowlist가 불일치 | 두 플랫폼 모두 같은 네 ID를 opt-in 목록에 추가하고 같은 unknown-scene negative 입력을 실행한다. **통과** |
| 11 | 기존 permissive expected-frame fallback이 성공을 허용 | 신규 IDs는 플랫폼별 strict frame matcher로 먼저 분기한다. **통과** |
| 12 | unknown/profile-conflict negative 결과가 launch 실패만 의미 | 두 플랫폼에서 앱 launch 뒤 process/runtime argument-error marker를 수집해 거부를 확인한다. **통과** |
| 13 | Android route가 selector 충돌을 정상 route로 처리 | 고정 `--ez spinon_c1121_grid_placement true`를 conflict 입력으로 사용하고 final manifest에 scene·selector를 기록한다. **통과** |
| 14 | iOS route가 selector 충돌을 정상 route로 처리 | `--spinon-c1121-grid-placement` conflict 입력과 runtime error marker를 별도 기록한다. **통과** |
| 15 | JSON parser가 문자열·boolean을 number처럼 허용 | Android matcher는 primitive JSON 타입을 strict 검사하고 iOS는 NSNumber/CFBoolean 분리 helper를 사용한다. **통과** |
| 16 | trace pass 개수·순서만 확인하고 stage 안쪽 tracks 누락 | 두 platform matcher는 모든 stage의 9 internal Rows entry 및 각 CSS track/gutter 필드를 검사한다. **통과** |
| 17 | batch에 추가/중복/다른 span이 들어옴 | 고정 stage 명칭 배열과 길이로 `[2]` 또는 `[2,3]`만 허용한다. **통과** |
| 18 | contribution 유형·item 순서·숫자가 누락되어도 통과 | 종류, span, semantic node ID, source order, fixed block contribution 및 finite tolerance를 모두 검사한다. **통과** |
| 19 | forward/reversed geometry가 같은 화면이므로 순서 오류를 감춤 | 스크린샷을 유일 기준으로 쓰지 않고 node index와 raw contribution/frame 로그를 대조한다. **통과** |
| 20 | frame tolerance를 trace tolerance에 섞음 | frame은 `1/64 CSS px`, trace는 `1e-4 CSS px`로 따로 고정한다. **통과** |
| 21 | screenshot을 픽셀 동일성이나 paint 정확성으로 주장 | 문서와 PR 초안에서 screenshot은 표시·배치 참고로만 설명한다. **통과** |
| 22 | iOS 표본을 compositor 이후 출력으로 오해 | 현재 slice는 별도 pixel readback을 하지 않았고 compositor 이후 출력도 검증하지 않았다고 한정한다. **통과** |
| 23 | Android AVD를 실기기 또는 hardware GPU로 확대 해석 | API 37 ARM64 16 KB AVD라는 모델을 명시하고 실기기/hardware GPU 주장을 제외한다. **통과** |
| 24 | 앱 artifact가 검증한 source와 다름 | manifest는 source input hash와 Android APK/iOS executable artifact hash를 연결하며 runner 종료 전 source/script hash를 재검사한다. **통과** |
| 25 | screenshot이 다른 run에서 복사되거나 잘못 연결됨 | 최종 run 폴더별 PNG, scene ID, dimensions, SHA-256, runtime/build 로그를 manifest에 함께 둔다. **통과** |
| 26 | 불완전하거나 실패한 run을 성공으로 덮어씀 | 첫 implicit-column 실패 run을 별도 보존하고 최종 성공 run ID 두 개를 명시한다. 성공 screenshot은 마지막 source capture에 한정한다. **통과** |
| 27 | simulator 실행 중 기존 boot 상태나 무관한 앱이 바뀜 | Android runner는 지정 AVD의 Spinon만 force-stop하고, iOS runner는 초기 boot 상태를 기록하며 직접 boot한 대상만 shutdown한다. **통과** |
| 28 | artifact scanner가 잘못된 경로·dirty tree를 근거로 기록 | `Cargo.toml`·`spec/` marker에서 저장소 root를 찾고 unstaged tracked diff 및 staged source에 빠진 untracked 파일을 실패 처리한다. **통과** |
| 29 | 기본 archive 부재만 보고 positive control 없이 검사를 통과 | product marker 0/4·fixture ABI/source symbol 부재와 opt-in marker 4/4·ABI/source symbol 존재를 한 번의 scanner에서 확인했다. **통과** |
| 30 | 정적 archive 결과를 최종 bundle 또는 전체 적합성으로 확대 | 결과는 `spinon-ffi` 정적 archive 비교에 한정한다. APK/IPA exclusion, WPT, 일반 author CSS, 실기기와 전체 C11.4는 남은 범위로 둔다. **통과** |

## 최종 diff 경계

검토 이후 `spec/STATUS.md`에 동일한 C11.4 실행 기록이 두 번 들어간 것을 찾아 하나로 정리했다. 정적 archive scan은 source/doc diff를 모두 stage한 뒤 다시 실행해 결과 보고서의 diff digest를 최종 index와 맞춘다. 실행하지 않은 status check이나 CI는 통과했다고 기록하지 않는다.
