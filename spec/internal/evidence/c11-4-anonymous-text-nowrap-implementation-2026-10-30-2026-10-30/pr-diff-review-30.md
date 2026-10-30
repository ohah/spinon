# 구현 PR diff 적대 검토 30개

대상 PR [#861](https://github.com/ohah/spinon/pull/861), base `2f4d1cece718b0f65d3af3cfc458865cc410da6b`, 기능 변경 head `6688ce639cd474f92021740e293fac6da034779d`. layout/runtime/FFI, 양 플랫폼 matcher, capture catalog, 계획·상태·근거, 실제 PR 본문과 이미지 링크를 함께 대조했다. 이 기록은 검토 뒤 추가하며, 그 외에는 기능 변경을 하지 않았다.

| # | 적대 검토 관점 | 판정 |
|---:|---|---|
| 1 | 변경이 일반 author CSS admission을 실수로 넓히는가 | **통과.** 공개 CSS parser/admission 파일은 diff에 없고 변경은 기존 opt-in anonymous Grid text fixture collector와 validator에 한정된다. |
| 2 | 새 public API나 버전 없는 interface를 추가하는가 | **통과.** public API/ABI 선언은 추가하지 않았고 기존 fixture-test FFI route를 사용한다. |
| 3 | partial fixture 결과를 C11.4 또는 C11 완료로 표시하는가 | **통과.** checkbox와 공식 `72/213`은 그대로 미완료다. |
| 4 | 기존 `normal` case의 expected outcome을 바꾸는가 | **통과.** 새 20px normal control은 min-content 20px·두 줄을 고정하고 기존 120px normal scene도 유지한다. |
| 5 | nowrap의 whitespace-collapse 값이나 wrapping mode가 서로 다르게 고정되는가 | **통과.** browser fixture·runtime validator·ParagraphStyle이 collapse + nowrap을 같은 scene ID에서 확인한다. |
| 6 | min-content와 max-content를 같은 숫자라서 하나만 검사하는가 | **통과.** Rust layout tests와 platform trace는 min/max content와 row height를 각각 판정한다. |
| 7 | root width 20px와 item width 50px overflow를 conflated 하는가 | **통과.** root frame과 anonymous item frame을 별도로 기대값과 대조한다. |
| 8 | auto root height를 fixed pixel height로 해석하는 validator regression이 남는가 | **통과.** validator는 신규 장면에서 `ComputedCssDimension::Auto`를 요구하고 실제 초기 실패 수정이 기록됐다. |
| 9 | 원래 anonymous text scene 수 또는 고정 scene ID 집합을 교체·축소하는가 | **통과.** 5개에서 7개로 확장하며 나머지 5개 ID를 그대로 유지한다. |
| 10 | FFI test가 generated source 내용과 scene count를 검증하지 않는가 | **통과.** fixed seven-scene test는 uniqueness, DOM source text, dimensions, wrap mode와 text node count를 확인한다. |
| 11 | runtime scene route가 ID는 받지만 direct text source를 빠뜨리는가 | **통과.** normal/nowrap 두 장면 모두 같은 `XX XX` direct text를 사용한다. |
| 12 | runtime validator가 새 ID에 잘못된 Grid columns/rows를 허용하는가 | **통과.** 두 새 ID에 단일 `min-content` column과 auto row를 요구하고 Grid mutation rejection tests가 남아 있다. |
| 13 | `PreLine`이 normal/nowrap 허용 조건에 우연히 포함되는가 | **통과.** allowlist는 두 enum만 받고 `PreLine` 명시 오류 테스트가 있다. |
| 14 | nonzero `order` negative control이 다른 노드나 error를 검사하는가 | **통과.** `order:1`은 anonymous paragraph owner의 `UnsupportedTextParagraph`를 검사한다. |
| 15 | source node grouping, comments, hidden child continuation, block separator 동작을 변경하는가 | **통과.** 이 알고리즘은 건드리지 않았고 기존 관련 regression tests와 fixture IDs를 유지했다. |
| 16 | capture catalog의 전체 수와 anonymous subset 수가 새 scene 수와 맞지 않는가 | **통과.** anonymous subset 5개에서 7개, 전체 622에서 624로 수정했고 현재 도구 테스트 23/23이다. |
| 17 | 다른 CSS-reference tests에 stale `622` expectation이 남는가 | **통과.** capture/tool에서 검사한 scene total의 오래된 622 assertion이 남지 않았다. |
| 18 | simulator selection이 두 새 ID를 실제 선택하지 못하는가 | **통과.** selected-run classifier가 두 ID를 anonymous text set으로 처리하고 AVD/iOS 결과에는 두 장면이 모두 있다. |
| 19 | Android matcher가 root frame 또는 box count를 잘못 판정하는가 | **통과.** 두 expected root frame, one-box count와 scene classifier가 업데이트됐고 API 37 AVD 결과와 맞는다. |
| 20 | iOS matcher가 Android scene matrix와 달라지는가 | **통과.** iOS allowlist, expected frames, box count와 classifier가 같은 geometry를 요구하며 Simulator trace가 맞는다. |
| 21 | Android/iOS result JSON의 `drawPresented`와 retries를 근거 없이 요약하는가 | **통과.** 각 platform result에는 두 scene 모두 present 성공, retry 0, track size와 trace chunk count가 있다. |
| 22 | screenshots를 glyph pixel oracle로 과장하는가 | **통과.** PR과 README가 screenshots를 surface draw/present 증거로 한정하고 geometry/line count는 trace로 판정한다고 설명한다. |
| 23 | 실패한 Android capture를 통과로 합산하거나 지우는가 | **통과.** validator 실패와 SurfaceFlinger 장애 실행은 보존하고 최종 cold-boot AVD 실행만 성공으로 분류한다. |
| 24 | plan review 20과 implementation/runtime review 20을 같은 검토로 재사용하는가 | **통과.** 계획 검토와 실제 코드/runtime 검토가 별도 파일이고 PR diff는 이 문서의 별도 30 관점이다. |
| 25 | 기본 artifact 검사에서 신규 fixture payload/exports가 실제 빠지는가 | **범위 내 통과.** `c04-runtime-gpu` archive에 신규 ID와 두 host exports가 없고 fixture-test archive에는 모두 있다. 기존 RuntimeSession constructor symbol 제한을 README에 공개한다. |
| 26 | static archive 검사를 final APK/IPA 또는 전체 runtime 격리로 확장 해석하는가 | **통과.** evidence와 PR body는 static archive 한정이며 final app artifact claim을 하지 않는다. |
| 27 | 외부 이미지 링크가 PR UI에서 실제로 로드되는가 | **수정 후 통과.** Agent Browser에서 Chrome, Android normal/nowrap, iOS normal 임베드 4개가 natural dimensions로 로드됐다. iOS nowrap 임베드는 0×0이어서 제거하고 같은 원본의 직접 링크로 바꿨다. 원본 direct URL은 Browser에서 1206×2622로 열렸다. |
| 28 | PR 본문 검사 수나 파일 수가 실제 diff와 다르게 적혔는가 | **통과.** PR #861은 base `main`에서 code head `6688ce6`의 45-file implementation/evidence diff로 열렸다. 이 review artifact는 문서 변경으로 추가하며 이후 기능 코드는 바꾸지 않는다. |
| 29 | GitHub check rollup이 비어 있는데 CI 성공이라 말하는가 | **통과.** PR 조회에서 check rollup은 비었고 main protection에도 required status checks가 없다. 이를 CI success로 세지 않는다. |
| 30 | repository merge policy와 최종 merge proof를 건너뛰는가 | **통과.** 저장소 설정은 Rebase and merge만 허용한다. merge 직전 check rollup/state를 다시 확인하고 merged state, `mergedAt`, `mergeCommit`을 검증해야 한다. |

**판정:** 30개 diff 관점을 검토했다. 검토 중 PR 본문 iOS nowrap 이미지 한 장이 임베드 실패했다. 해당 그림은 PR에서 깨진 이미지로 남기지 않고 직접 열리는 원본 링크로 바꿨으며, 최종 4개 embedded image의 natural dimensions를 Agent Browser에서 확인했다. PR check rollup은 비어 있으므로 필수 CI 통과 기록은 없다.
