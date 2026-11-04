# C11.1 mixed explicit/one-axis placement 실제 변경 diff 검토 20개

이 검토는 후보 구현과 runtime의 별도 20개 실패 관점 검토 뒤, 실제 Rust/platform/fixture/spec diff를 다시 읽어 수행했다. 아래 관점은 runtime review 표와 겹치지 않도록 diff 경계·호출 순서·기존 경로 회귀·artifact 문서 일관성에 초점을 맞췄다.

| # | diff 관점 | 판정 |
|---:|---|---|
| 1 | 새 `GridPlacementMode` variant의 validation/normalization match arm 중 하나가 빠져 exhaustive check 또는 잘못된 fallback이 생기는가 | Grid validation, placement dispatch와 item normalization에서 모두 기존 sparse normalizer로 연결된다. scoped builds/tests 통과. |
| 2 | `RootHtmlOneAxisAutoPlacementV1`의 기존 동작을 새 mode가 변경하는가 | 기존 selector는 fully definite item을 계속 제외하고 one-axis mode도 변경하지 않는다. layout one-axis 회귀와 style integration 회귀 통과. |
| 3 | style projection에서 mixed mode가 explicit-only 또는 grid alignment mode에 의해 뒤늦게 덮이는가 | mixed selector와 old selector는 input class 조건상 배타적이다. mode 지정 순서와 named-area/alignment validation 인자를 확인했다. |
| 4 | dense/column route 또는 다른 owner가 mixed selector를 우회하는가 | dense owner를 selector에서 제외하고 owner별 item set을 만든다. row·column mixed owner가 같은 문서에서 각자 맞는 sample을 냈다. |
| 5 | item endpoints의 computed representation에서 `Line`/`Span` 외 syntax가 우연히 accepted되는가 | selector의 numeric endpoint allowlist와 layout named placement reject가 대응한다. selector 3/3. |
| 6 | fully definite-only 또는 partial-only 목록이 candidate로 남는가 | selector와 layout validator 양쪽에서 두 집단이 모두 하나 이상이어야 한다. full/all-auto existing controls와 missing-class negative test가 이를 고정한다. |
| 7 | `display:none` child가 owner의 axis 분류를 오염시키는가 | runtime node collection과 test에서 숨긴 opposite-axis item이 candidate 계산에 빠지는 것을 확인했다. |
| 8 | `visibility:hidden` item을 candidate에서 잘못 빼서 occupancy가 달라지는가 | hidden full item은 selector와 layout occupancy에 남는다. column-locked 후속 표본은 세 번째 row에서 기대 색상이다. |
| 9 | fully definite row blocker sample이 실제로 뒤의 partial item을 막는가 | diff review에서 pass18 HTML의 blocker 위치가 같은 row가 아니란 점을 찾았다. HTML/Swift/Java 표본을 수정했고 최종 row 1의 second partial 색상은 column 3에서 19/19 exact다. |
| 10 | Java sample ID, CSS point, expected RGBA 배열 index가 서로 어긋나는가 | 세 배열 모두 19개이며 row-full-blocker green, row-second purple의 새 index/좌표를 PixelCopy log로 확인했다. |
| 11 | Swift `nodeId`가 HTML element id와 다르거나 sample count가 다르게 고정되는가 | 신규 row sample의 `nodeId`는 HTML id와 동일하고 최종 run에서 19 sample이 모두 matched다. |
| 12 | Android 기존 one-axis run ID/scene가 mixed 분기 추가로 깨지는가 | `c11oap-20261104`와 mixed run/scene/sample set을 별도 분기한다. 기존 adapter source route는 보존했다. |
| 13 | iOS mixed run 감지가 prefix만 확인해 다른 run을 오인하는가 | one-axis prefix 뒤의 정확한 `c11mix19-20261104` suffix를 확인하고 허용 run ID list도 제한했다. 최종 run summary의 scene ID가 일치한다. |
| 14 | 기본 iOS 빌드에 신규 static sample literal이 포함되는가 | sample table을 compile condition 안으로 이동했다. 기본 app scan은 새 ID/scene/sample strings 0건, opt-in positive control은 포함한다. |
| 15 | Android fixture Java class가 기본 source set에 포함되는가 | Gradle source set은 C11 pixel fixture flag에서만 경로를 추가한다. 기본 APK DEX에는 class/ID/sample이 없고 opt-in DEX에는 있다. |
| 16 | simulator evidence가 `layout=ready` 이전 또는 draw가 없는 run에서 나온 것이 아닌가 | Android와 최종 iOS process log에서 ready/draw-presented 다음 sample summary를 확인했다. |
| 17 | iOS drawable 표본이 OS compositor 이후 값으로 과장되는가 | README와 계약은 Metal drawable readback을 compositor 제출 전으로 한정한다. OS screenshot은 별도 파일로 둔다. |
| 18 | Chrome observation/expected/hash와 fixture correction이 서로 다른 입력을 가리키는가 | Agent Browser 재실행의 JSON/화면 hash가 committed observation/screenshot과 동일하고 verifier는 9 scene·19 frame 통과다. |
| 19 | unsupported rejection을 mobile runtime에서도 atomicity로 검증했다고 문서가 과장하는가 | negative controls를 Rust tests/Chrome oracle에 분리한 fail-fast limitation을 계획·contract·README에 반영했다. |
| 20 | 제한 구현을 C11 완료·공식 상태 증가·실기기 GPU 지원으로 과장하는가 | C11 parent checkbox와 `72/213`은 그대로이고 계획/contract/STATUS 모두 simulator·limited-profile 경계를 명시한다. |

**결과:** 20개 관점에서 blocking diff defect는 남지 않았다. pass18 blocker-fixture 결함을 수정한 뒤 Android PixelCopy와 iOS Metal drawable 모두 19/19 exact, focused Rust 9/9 + 8/8 + 3/3, scoped Clippy/rustfmt 및 default/opt-in scans를 최종 확인했다. 이 검토는 실제 GitHub PR의 별도 30개 review를 대체하지 않는다.
