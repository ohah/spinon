# C11.4 행 축 overlapping-span runtime 계획 적대 검토 20회

**대상:** [행 축 runtime slice 계획](c11-4-row-axis-overlapping-span-runtime.md) 및 이미 병합된 [행 축 stage 사전 비교](../spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/README.md)
**판정:** 구현 전 계획 검토 완료. 아래 20개 관점의 조건을 계획에 반영한 뒤 구현을 시작한다.

| # | 실패 관점 | 검토 결과와 계획의 방지 조건 |
| ---: | --- | --- |
| 1 | 기존 column scene ID 재사용 | Rust·Android·iOS에 이미 `competing-span2-*`와 `span2-before-span3`가 있다. 새 ID는 모두 `row-axis-` 접두어로 분리한다. **통과** |
| 2 | Rows 입력이 실수로 Columns에 놓임 | 기존 `multispan_ordering.rs`는 네 intrinsic Columns와 고정 Rows다. 새 입력의 Columns 220px, Rows 네 `minmax(0,max-content)`와 `grid-row` 명시를 고정한다. **통과** |
| 3 | reversed DOM order에서 item/node 매핑이 틀림 | scene별 semantic A/B/C→node index 및 authored order를 매핑하고 모든 node frame을 검사한다. **통과** |
| 4 | batch 누락이어도 frame만 보고 성공 | Rows pass 한 개와 batch 순서 `[2]` 또는 `[2,3]` 및 전체 stage 순서를 strict matcher로 검사한다. **통과** |
| 5 | 신규 expected frame 누락 시 기존 null fallback이 성공 | 네 신규 scene은 `expected == null => true` 경로를 쓰지 않고 빠진 frame/stage 기대값은 실패하게 한다. **통과** |
| 6 | Chrome 반올림으로 규범 stage를 덮어씀 | 유리수 규범 stage와 Chrome 최종 geometry를 구분하고 별도 tolerance를 적는다. **통과** |
| 7 | serialized null을 +∞로 단정 | null은 finite number 미직렬화로만 설명하고 +∞ 단독 증거로 확대하지 않는다. **통과** |
| 8 | root frame 하나로 source-order 결과를 축약 | 네 scene 각각의 전체 node frame과 의미 item frame을 forward/reversed 쌍으로 확인한다. **통과** |
| 9 | opt-in fixture가 기본 archive로 유출 | feature off 기본 archive negative 및 fixture-test positive control을 각각 검사하고 final APK/IPA exclusion은 주장하지 않는다. **통과** |
| 10 | 제한 runtime slice를 C11.4 완료로 과장 | 공개 CSS 지원과 C11.4 checkbox를 미완료로 두고 72/213을 유지한다. **통과** |
| 11 | 다른 profile 인자와 함께 scene을 실행 | opt-in route의 기존 profile 상호배타성 검사를 유지하고, 잘못된 추가 selector·미등록 ID가 거부되는 negative case를 확인한다. **수정 반영** |
| 12 | trace chunk 누락·재조립 순서 오류 | 각 platform log에서 모든 trace chunk 번호·총수를 검사해 부분 trace가 assertion에 들어가지 않도록 한다. **통과** |
| 13 | fixed Columns control 또는 추가 pass를 놓침 | pass 순서를 `[Columns, Rows]`로 고정하고 Columns의 3 internal entry 중 단일 CSS track이 220px인 `FixedTrackEarlyReturn`, Rows pass 한 개를 확인한다. **수정 반영** |
| 14 | fixed-size item 값을 intrinsic text/image 지원으로 확대 해석 | 입력을 고정 크기의 빈 div로 제한하고 intrinsic text/image 및 automatic minimum은 명시적으로 제외한다. **통과** |
| 15 | DOM 순서를 normative span batch 순서로 착각 | forward/reversed 모두 작은 span부터 `[2,3]`로 처리되는 stage를 확인하고, 같은 span group에서는 track별 최대 증가량을 비교한다. **통과** |
| 16 | 마지막 track size만 비교해 중간 단계 회귀를 놓침 | Initialize, 각 batch, IntrinsicSizingComplete, MaximizeTracks, ExpandFlexibleTracks, StretchAutoTracks snapshot의 base/growth 값을 비교한다. **수정 반영** |
| 17 | gutter·collapsed track 제거 후 잘못된 수의 track도 통과 | 각 Rows stage 원본 배열 9개가 gutter/CSS track으로 번갈고 boundary gutter만 collapsed인지 검사한다. 4 CSS track과 5 zero-base gutter를 모두 확인하고 implicit track 추가를 거부한다. **수정 반영** |
| 18 | screenshot을 pixel equality로 과장 | screenshot은 장면/배치 보조 근거로만 쓰고 별도 readback 표본을 검사하지 않으면 pixel equality를 주장하지 않는다. **통과** |
| 19 | 소스나 앱 artifact가 run 이후 달라짐 | 최종 source revision의 입력 hash·앱 artifact hash와 raw log·screenshot hash를 manifest로 연결한다. **통과** |
| 20 | simulator 실행/정리 과정에서 무관한 기기를 종료 | 실행 전 boot 상태를 기록하고 기존 booted device는 유지한다. 스크립트가 직접 boot한 지정 iOS UDID만 종료하며 다른 device는 건드리지 않는다. 현재 대상 AVD는 `emulator-5554`, booted iPhone 17 Pro는 `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`다. **수정 반영** |

## 검토 근거

- 관점 1–10은 독립 read-only hostile review에서 얻었다. 기존 scene ID와 dispatcher는 [multispan_ordering.rs](../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/multispan_ordering.rs), 플랫폼 allowlist는 Android `C0410RuntimeGpuDemo.java` 및 iOS `C0410RuntimeGpuDemo.swift`, permissive frame fallback은 두 플랫폼의 `c114ExpectedNodeFramesMatch`에서 확인했다.
- 관점 11–20은 계획 입력, 기존 앱 route와 trace/fallback 경계를 별도로 읽고 작성했다.
- 이 검토는 계획과 현재 source 경로 검토다. 구현 코드, 빌드, runtime, simulator 결과의 검토를 대신하지 않는다. 기능 구현 뒤 새 실패 관점 20개와 PR diff 30개를 따로 기록한다.
