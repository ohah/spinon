# C11.4 anonymous Grid text + image 구현·runtime 실패 관점 검토 20개

범위는 이 작업의 최종 Rust projection/runtime fixture, Android·iOS adapter, capture verifier 및 다섯 scene 실행이다. 계획 검토와 구현 PR diff 검토는 각각 별도 기록으로 둔다.

| # | 실패 관점 | 실제 경계 대조와 판정 |
| --- | --- | --- |
| 1 | opt-in helper가 기본 API에서 노출됨 | helper와 exports는 `c11-4-grid-track-sizing-fixture-test` 뒤에 있다. 기본 `spinon-ffi` archive에는 신규 scene ID와 C fixture ABI가 모두 없다. |
| 2 | image-only 또는 text-only 기존 경로가 새 입력을 중복 준비함 | `runtime_layout.rs`는 결합 scene을 먼저 준비하고 인식된 경우 기존 mixed, anonymous text, text, image 입력 준비를 건너뛴다. 기존 단일 입력 분기는 그대로 둔다. |
| 3 | 임의 scene ID가 고정 fixture처럼 처리됨 | root ID에서 허용 scene만 일치시킨다. 해당 namespace의 알 수 없는 유사 ID는 unsupported 오류, 무관한 ID는 기존 경로로 남는다. |
| 4 | 잘못된 root 또는 다른 계산 profile이 허용됨 | HTML `div`, document root의 직접 자식, `RuntimeBlockFlexGridTrackSizingV1`과 고정 Grid style을 모두 확인한다. |
| 5 | 자식 순서·소유권이 달라도 알려진 fixture로 수용됨 | 다섯 scene의 직접 자식 개수·순서·parent 및 kind/content를 순서대로 확인한다. Rust test도 source-order node ID를 고정한다. |
| 6 | Chrome의 실제 comment를 HostDocument comment node로 오인함 | runtime topology는 인접 Text node 두 개로만 표현하고 literal comment node 지원을 주장하지 않는다. Chrome oracle와 host-tree 표현의 경계를 README/plan에 적었다. |
| 7 | whitespace-only anonymous run이 빈 Grid item을 만듦 | whitespace scene은 앞의 ` \t\n`이 intrinsic input에 없는지, 뒤의 `\n\tXX` source와 위치가 남는지 검증한다. 양 플랫폼 결과가 일치한다. |
| 8 | `display:none` subtree의 text가 visible run을 flush함 | hidden span의 style·자식 text를 확인하되 layout collector에서는 subtree를 건너뛴다. Rust와 양 플랫폼에서 두 visible text item과 image 순서가 유지된다. |
| 9 | 기본 inline `<img>`가 text run 내부로 흡수됨 | 고정 HTML image item 집합에 한해서 fixture opt-in projection이 computed inline을 Layout block으로 변환한다. 다섯 장면에서 image 앞뒤 text가 분리된다. |
| 10 | blockification gate가 전체 Grid image로 확장됨 | 일반 projection 옵션은 false이고 새 결합 adapter만 true다. predicate는 HTML image Grid item node 집합에 한정된다. |
| 11 | 다른 image tag/namespace 또는 descendants를 허용함 | image는 HTML namespace `img`, 정해진 ID/URL, 직접 child, 자식 없음만 받는다. |
| 12 | 이미지 크기 입력이 network나 stale resource에 의존함 | request identity는 document에서 읽고 decode payload는 고정 PNG byte로 제공한다. natural size 24×16과 revision 1을 Rust/runtime trace에서 확인한다. |
| 13 | text와 image intrinsic input이 다른 revision에서 섞임 | 같은 계산에 intrinsic snapshot을 전달하고 layout result에 그 revision을 bind한다. Rust test는 모든 item revision을 검사하고, 두 native validator는 diagnostics revision 1을 요구한다. |
| 14 | text run anchor/source mapping이 불안정하거나 손실됨 | Rust 회귀는 input node 순서와 text source set을 검사하고 app validator는 `textSources`, source order, Ahem metrics를 exact match한다. |
| 15 | root typography/font가 OS font로 바뀌어 metrics가 흔들림 | fixture 전용 no-system-font engine에 Ahem bytes를 등록한다. 10px Ahem metrics를 Rust·Android·iOS에서 확인한다. |
| 16 | grid style 변화가 계산에서 조용히 통과함 | root size, track 수/종류, row, gap, line names, flow, alignments와 inherited text style을 exact validation한다. image의 span, 48px height, edges와 natural ratio 조건도 검사한다. |
| 17 | track이 맞아도 잘못된 item/frame/revision이 통과함 | Rust는 node/sourceOrder/revision/source metrics/root-relative image frame/Columns·Rows trace를 검사한다. capture verifier는 app summary, track list, trace span batches, node frames와 draw result를 대조한다. |
| 18 | Android ADB 대상이 실기기거나 다른 emulator일 수 있음 | runner는 `ro.kernel.qemu=1`, API 37, ARM64를 요구했고 결과는 `emulator-5558` AVD로 저장했다. 실기기 실행을 하지 않았다. |
| 19 | iOS 앱 로그가 다른 시뮬레이터나 이전 run에서 옴 | runner는 지정 UDID의 booted iPhone 17 Pro와 iOS 26.2만 허용하고 PID 및 run ID를 log filter에 묶는다. 다섯 최종 결과는 고유한 run ID를 가진다. |
| 20 | 검증기 자체가 맞지 않거나 partial draw를 pass 처리함 | 첫 실행에서 run ID 길이와 text `Minimum` trace 가정이 잘못된 점을 찾아 수정했다. raw AVD trace로 예상 순서를 교정한 뒤 새 run으로 Android 5/5, iOS 5/5를 재실행했다. `drawPresented=true`만 입증하며 texel paint/compositor는 범위 밖이라고 기록했다. |

판정: 수정 후 차단 결함은 남지 않았다. focused Rust tests, artifact boundary, workspace Clippy, Android/iOS fixture build와 양 simulator의 전체 scene matrix가 통과했다. 최종 PR 변경 diff는 별도 30관점 검토 대상이다.
