# C11.4 mixed text/image implementation PR diff 검토 20개

계획 변경 검토 및 runtime 동작 검토와 별도로 실제 PR diff의 파일 간 계약, 산출물, 문서와 근거를 확인했다.

| # | diff 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | 공개 CSS/API 또는 versioned 내부 계약을 바꾸는 범위가 섞였는가 | fixture-only 계산 adapter와 앱 harness만 추가했다. public CSS 지원이나 내부 contract version 변경은 없다. |
| 2 | C11.4 전체 완료나 공식 완료 수를 올려 미검증 범위를 숨기는가 | `spec/STATUS.md`는 네 scene의 제한 결과만 적고 C11.4 checkbox 및 완료 수는 미완료로 유지한다. |
| 3 | Cargo feature chain 중 한 crate에서만 gate가 빠지는가 | runtime → style-to-layout → layout feature 연결과 FFI/app fixture flags를 대조했다. default build와 fixture positive control을 각각 검사했다. |
| 4 | 신규 FFI scene adapter가 기존 runtime product module에 들어가는가 | scene 생성은 `runtime_gpu/fixtures/c11_4/mixed_multispan.rs`와 `fixtures` module 안에 있다. |
| 5 | Rust unit test가 fixture production include와 혼합되는가 | 순수 scene-array test는 `#[cfg(test)]` 하위 모듈에 있고 app runtime harness는 opt-in fixture feature에 있다. |
| 6 | 새 Rust 파일이 과도하게 커지거나 역할이 섞이는가 | mixed adapter의 style 검증과 unit test를 하위 `style.rs`, `tests.rs`로 나눴다. style helper는 상위 fixture 타입에 내부 가시성으로 접근한다. |
| 7 | 같은 장면 집합의 Rust runtime, FFI adapter, app harness, capture runner에서 ID가 다르게 쓰이는가 | 네 scene ID를 네 경계에서 비교했고 API 37 AVD·physical Android·iOS 실행이 모두 같은 ID로 성공했다. |
| 8 | 성공 장면 수 검사가 기존 150/151 scene 기준과 어긋나는가 | C11.4 image matrix 뒤 mixed 네 개를 154, auto-minimum flexible 후속을 155로 계산하고 unique-ID assertion을 유지했다. |
| 9 | `run-kind`/evidence route가 mixed run을 일반 multi-span 경로에 저장하는가 | 혼합 scene 집합에 전용 `mixed-multispan-contributions` 기본 run kind와 evidence directory를 사용한다. |
| 10 | scene filter가 네 mixed 장면 전부를 선택하지 못하거나 다른 scene을 섞는가 | `--scene-id`로 네 장면만 실행해 각 target JSON의 `sceneCount=4`를 확인했다. |
| 11 | 기대 track 값과 독립 rational baseline이 platform harness에서 다르게 반올림되는가 | 기대값은 `[25,36,36,0]` 또는 `[158/3,191/3,191/3,0]`; 허용 frame 오차는 `1/64` CSS px 이하로 고정되어 있다. |
| 12 | DOM frame 순서와 paint/node source order가 서로 다르게 검증되는가 | Rust trace의 source order 및 native frame node ID/order를 각 장면별로 확인한다. reverse-order pair는 frame geometry가 같고 output order는 DOM 순서를 따른다. |
| 13 | Android Java가 scene별 text/image intrinsic metric 또는 revision을 누락하는가 | four-scene matrix가 source kind/order, node, text metrics, natural size, intrinsic revision 1을 검사하며 physical/AVD summaries를 통과했다. |
| 14 | iOS Swift가 Android와 다른 scene IDs/frames/count를 기대하는가 | Swift allowlist, expected node frame, box count, intrinsic source checks를 Java 및 Rust fixture와 대조해 네 scene simulator 실행을 통과했다. |
| 15 | Android 테스트가 emulator 통과를 real device로 오인하거나 그 반대인가 | API 37 AVD는 별도 run으로 기록했고 SM-S731N은 `qemu=0`, serial, model 및 installed APK SHA-256을 별도 metadata로 기록했다. |
| 16 | 현장 화면 첨부가 notification shade 또는 잘못된 화면을 가리키는가 | 최종 실기기 PNG 네 장을 직접 확인했다. notification overlay가 있던 이전 임시 PNG 두 장은 제거했고, capture runner는 화면 저장 전 status bar를 접는다. |
| 17 | README의 증거 링크가 실패 로그만 가리키거나 intermediate capture를 최종으로 오인시키는가 | README는 final AVD/physical/iOS 디렉터리를 각각 링크하고 첫 allowlist 실패 로그는 별도 초기 실패로 표시한다. |
| 18 | default artifact 검사 결과를 전체 제품 크기/배포 파일 증거로 확대 해석하는가 | README가 검사를 Rust static archive로 한정하고 Xcode `nm` LLVM22 호환 진단도 기록한다. APK/iOS release size로 해석하지 않는다. |
| 19 | 첨부된 실행 화면에서 PixelCopy/Metal readback까지 통과했다고 주장하는가 | 문서와 PR 본문에 PixelCopy/Metal 표본 및 PNG texel 검증을 미실행으로 명시한다. |
| 20 | stage 범위에 다른 진행 중 session의 untracked evidence가 섞이는가 | commit stage에는 현재 C11.4 source/docs와 지정한 세 final target evidence 및 최초 allowlist 실패 로그만 포함한다. 다른 evidence 디렉터리는 stage하지 않는다. |

## 발견과 조치

- Java/iOS allowlist 누락으로 첫 Android run이 거부됐다. 두 앱 harness 기대값을 맞춘 뒤 AVD, 실기기, iOS에서 다시 실행했다.
- 실기기 notification overlay PNG 두 장을 제거하고 캡처 직전 notification shade를 접는 runner 변경을 넣었다. 마지막 Android run의 네 PNG를 모두 확인했다.
- 그 외 PR 경계·feature chain·expected geometry/source mapping 사이의 불일치는 찾지 못했다.

## 남은 범위

이 diff는 direct-leaf Ahem text와 고정 PNG의 네 Grid scene에만 적용된다. WPT, arbitrary CSS, all Grid sizing combinations, PixelCopy/Metal pixel, 최종 iOS compositor surface, PNG texel과 전체 C11.4 완료는 포함하지 않는다.
