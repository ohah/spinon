# C11.4 depth-three nested Grid text multi-span PR diff 실패 관점 검토 30개

구현/runtime 실패 관점 20개와 별도로, 최종 변경 파일 간 allowlist·feature·문서·simulator·artifact·PR 화면 증거가 서로 어긋나는 병합 실패 경로를 검토했다. 검토 대상은 PR [#464](https://github.com/ohah/spinon/pull/464) 전체 diff와 실제 simulator/browser 증거다.

| # | Diff 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | runtime fixture 새 파일이 제품 module graph에서 기본 활성화되는가 | `fixtures.rs` 신규 module은 `c11-4-grid-track-sizing-fixture-test` 아래에만 선언된다. |
| 2 | FFI generator와 runtime adapter의 Cargo feature 경계가 달라 fixture ABI가 기본에 들어가는가 | FFI module은 기존 C11.4 fixture module 아래 있고, archive positive/negative build를 따로 확인했다. |
| 3 | 새 `prepare()` 호출이 기존 depth-three/direct text route보다 뒤에 있어 scene을 놓치는가 | `c11_4_text::prepare`에서 nested text multi-span adapter를 기존 depth adapters보다 먼저 부르며, 고유 접두사 외 ID는 `Ok(None)`으로 반환한다. |
| 4 | 등록한 scene ID가 runtime adapter, FFI source, app harness, capture verifier 사이에서 달라지는가 | 네 scene 문자열이 네 경계에 일치한다. Android/iOS capture 모두 각 scene을 선택·실행했다. |
| 5 | runtime/FFI allowlist에 와일드카드 scene을 우연히 허용하는가 | 네 고정 scene만 source를 생성하며 unlisted scene Rust negative test가 거부를 확인한다. |
| 6 | 추가한 FFI source 모듈이 기존 dispatcher에서 호출되지 않거나 잘못된 branch에 등록되는가 | `c11_4_grid_track_sizing_runtime_source`가 새 module을 조회하며 opt-in FFI test는 정확히 네 source만 생성함을 검사한다. |
| 7 | 테스트 파일이 기본 runtime target이나 앱 source에 섞이는가 | test module은 fixture-test feature 및 `#[cfg(test)]` unit-test graph에서만 컴파일되고 별도 fixture builder도 pure test helper다. |
| 8 | 새 fixture 문자열이 기본 FFI static archive에 이미 다른 route로 포함되는가 | 기본 archive의 네 신규 ID 검색은 일치 항목이 없고 fixture-test positive control은 네 ID와 payload를 포함한다. Hash/size는 artifact 근거에 있다. |
| 9 | FFI fixed source의 네 Grid hierarchy ID가 adapter에서 기대하는 identity와 다른가 | `root/middle/branch/inner` ID suffix와 inner item ID/order가 runtime adapter의 exact check와 일치한다. |
| 10 | direct text item style이 Ahem 측정 또는 span placement와 어긋나는가 | FFI source와 test fixture style이 10px Ahem, fixed row/column span, zero edge/gap을 같이 사용한다. |
| 11 | A/B/C intrinsic metrics와 CRD 계산이 simulator 기대값에 복사 오류가 있는가 | Rust, Java, Swift, JS 기대값이 A/B/C의 50/70/180px min-content와 95/180px nested widths를 공유한다. |
| 12 | source order를 바꾸면 item node ID 순서와 expected frame mapping이 바뀌지 않는가 | forward/reversed scene의 `a/b/c` item order와 생성 node 번호를 Java, Swift, JS 모두에서 확인한다. |
| 13 | 앱 harness가 잘못된 root frame도 empty item-frame special case로 통과시키는가 | 새 scene은 별도 95/180px root expectation, expected text intrinsic inputs, 전체 ordered node frames를 함께 확인한다. |
| 14 | Android와 iOS scene allowlist 중 한쪽만 네 ID를 포함하는가 | Java와 Swift 각각 같은 네 ID를 허용하며 둘 다 같은 four-scene capture run을 통과했다. |
| 15 | simulator `node_frames_css_px` 비교가 x/y/width/height나 node order를 빠뜨리는가 | 두 harness와 JS verifier는 모든 frame scalar, order, node identity 및 1/64px 허용치를 확인한다. |
| 16 | nested trace는 맞아 보여도 inner text source order 또는 span contribution 목록이 다른가 | JS capture verifier는 inner Column contribution 종류·span·값·순서를 전체 기대 배열과 대조한다. Rust test는 source order·min/max input·revision을 확인한다. |
| 17 | parent Grid contribution의 source node가 text leaf로 잘못 귀속되는가 | inner text input owner는 inner Grid이고 각 outer trace는 직계 nested Grid node의 span-1 min-content contribution을 확인한다. |
| 18 | 전체 capture matrix의 신규 scene count 또는 등록 ID 인벤토리가 불일치하는가 | `--list-scenes` inventory는 202 ID이며 신규 네 scene 모두 포함한다. 원본은 `scene-inventory.log`다. |
| 19 | 신규 multi-span adapter가 기존 direct-leaf text multi-span scene을 가로채는가 | direct-leaf FFI allowlist test와 기존 네 scene Android/iOS capture가 각각 4/4 통과했다. |
| 20 | Android 결과가 실기기 또는 다른 emulator에서 나온 것으로 오인되는가 | run metadata는 `emulator-5558`, `qemu=1`, API 37, `arm64-v8a`를 고정한다. 실기기 검증으로 확대하지 않는다. |
| 21 | iOS 결과가 다른 booted simulator 또는 runtime에서 나온 것인가 | metadata는 iPhone 17 Pro, iOS 26.2 runtime과 지정 UDID를 함께 기록한다. |
| 22 | 설치되지 않은 Android APK를 캡처한 것처럼 보고하는가 | 최종 AVD capture의 local APK와 installed `base.apk` SHA-256이 같고 `matched=true`다. |
| 23 | iOS screenshot이 검증한 app bundle과 다른 빌드에서 나온 것인가 | capture metadata에 app SHA-256과 simulator UDID가 함께 기록되어 있다. |
| 24 | Simulator app build가 fixture feature flag 없이 만들어 scene이 우연히 다른 앱을 테스트하는가 | Android Gradle/iOS build는 `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`로 실행했고 build logs가 성공을 기록한다. |
| 25 | CRD 이상값과 Chrome CSSOM quantized 관측을 같은 oracle로 합치는가 | 새 status/spec은 기대값을 [사전 비교](../c11-4-nested-grid-multispan-precomparison-2026-10-22/README.md)로 분리해 연결하고 1/64px 허용 비교를 사용한다. |
| 26 | screenshot 링크가 초기/중간 capture를 가리키거나 checksum 대상과 다르는가 | PR 본문 raw links는 최종 Android/iOS run folder의 representative images이며 `SHA256SUMS.txt`가 해당 files와 result metadata를 확인한다. |
| 27 | evidence README의 counts 또는 device/hash 설명이 raw JSON과 다른가 | scene 수, box count, trace chunks, draw/present, device/app/APK hashes를 각 `result.json`·`simulator.json`에서 확인했다. |
| 28 | plan, status와 internal contract가 범위를 C11.4 전체 완료로 올리거나 count를 증가시키는가 | 계획·0059·STATUS는 fixed fixture로 제한하며 parent checkbox를 그대로 두고 공식 `72/213`을 유지한다. |
| 29 | 라벨 누락 또는 checks가 보고되지 않았는데 CI 통과로 표현하는가 | PR에는 기존 `enhancement`·`area: layout` labels가 있다. 현재 main branch protection은 required PR과 linear history를 요구하고 required status checks는 `null`, 승인 수는 0이다. ruleset은 없으며 `gh pr checks 464`는 checks 미보고를 반환해 CI green으로 간주하지 않는다. |
| 30 | PR body에 붙인 raw image가 링크만 존재하고 실제 표시되지 않는가 | Agent Browser 0.31.1에서 두 `<img>` 모두 `complete=true`; Android 1080×2400, iOS 1206×2622를 반환했다. 확인 후 Agent Browser session과 해당 Chrome process를 종료했다. |

현재 PR #464는 open이며 확인 당시 GitHub merge state는 `CLEAN`/`MERGEABLE`이었다. main branch protection과 rulesets를 조회해 required-check policy를 확인했다. No checks reported는 CI pass와 같은 뜻이 아니며, 이 PR에는 required status checks가 설정되어 있지 않다.
