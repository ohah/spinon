# C11.4 generic direct-text review closure · PR #704 diff review 20

검토 대상은 PR #704의 fixture-test 전용 변경, 테스트 근거, 연결된 계획 기록, PR 본문과 라벨이다. 각 항목은 다른 실패 경로 또는 주장 경계를 점검한다.

| # | 공격 관점 | 판정과 근거 |
|---:|---|---|
| 1 | SVG namespace의 local name `img`를 HTML replaced image로 오인할 수 있음 | **PASS** — SVG `img` decoy를 두 valid HTML text-item 옆에 두고 collector가 계속 input을 반환하는지 검사한다. |
| 2 | HTML local name `image`를 `<img>`와 같은 태그로 오인할 수 있음 | **PASS** — HTML `image` decoy를 별도 case로 두어 tag name 조건을 검사한다. |
| 3 | decoy가 후보 item으로 세어 candidate cardinality를 바꿀 수 있음 | **PASS** — decoy 포함 후에도 paragraph owner가 정확히 두 개임을 확인한다. |
| 4 | decoy가 valid item 앞·사이·뒤에 있을 때 결과가 달라질 수 있음 | **PASS** — SVG `img`와 HTML `image`를 각각 첫째·가운데·마지막 root child 위치에서 검사한다. |
| 5 | 실제 HTML `<img>`를 감지해도 generic failure code가 달라질 수 있음 | **PASS** — `unsupported_c114_text_source`를 exact equality로 비교한다. |
| 6 | 오류가 root나 첫 text item을 가리켜도 `node_id.is_some()`만으로 통과할 수 있음 | **PASS** — 기대값을 fixture의 실제 세 번째 element handle에서 얻고 오류 node id와 exact 비교한다. |
| 7 | numeric NodeId를 하드코딩해 fixture 재생성에 취약할 수 있음 | **PASS** — NodeId를 실행 시 생성된 `fixture.items[2]`에서 취득한다. |
| 8 | root sibling `<img>`만 검사하고 image node identity는 다른 구조에서 바뀔 수 있음 | **PASS** — root sibling 재현은 기존 uncovered branch이며 기존 nested/item image 검사는 그대로 유지한다. |
| 9 | decoy가 하나의 valid text owner를 대체해도 count만 맞을 수 있음 | **PASS** — returned paragraph owner NodeId 집합이 두 실제 text-item handle 집합과 정확히 같은지 검사한다. |
| 10 | legacy 경계가 collector 반환만 확인하고 실제 dispatch가 끊겨 있을 수 있음 | **PASS** — test는 collector의 `None`을 확인한 뒤 부모 `c11_4_text::prepare`를 호출한다. |
| 11 | fallback input이 다른 text owner에 매핑될 수 있음 | **PASS** — returned paragraph map에 legacy fixture item의 HostDocument NodeId가 포함되어야 한다. |
| 12 | 임의 root가 legacy allowlist에 있다고 잘못 성공할 수 있음 | **PASS** — fallback positive는 기존 `TEXT_CASES[0]` root, item, text를 그대로 사용한다. |
| 13 | legacy fallback 검증이 computed style mismatch를 무시할 수 있음 | **PASS** — 전용 stylesheet는 그 case의 120×40 root, auto tracks, Ahem 10px text, wrap style을 실제 cascade한다. |
| 14 | 지원하지 않는 shorthand가 예상 fallback 실패를 만들 수 있음 | **PASS** — stylesheet는 author CSS allowlist가 지원하는 longhand만 사용하며 첫 시험의 unsupported `border` shorthand는 제거했다. |
| 15 | non-grid collector 경계가 legacy integration 확인으로 덮일 수 있음 | **PASS** — 기존 non-grid `None` 검사와 legacy full-dispatch 검사를 분리해 보존한다. |
| 16 | feature gate 밖 제품 빌드에 fixture test 경로가 새어 나갈 수 있음 | **PASS** — 변경은 이미 `c11-4-grid-track-sizing-fixture-test`로 gated된 test module에만 있다. |
| 17 | 테스트 추가가 runtime behavior를 변경했다고 잘못 표현할 수 있음 | **PASS** — 변경 파일은 test source와 근거/계획 기록이며 제품/runtime Rust source는 수정하지 않았다. |
| 18 | Simulator나 screenshot을 실행하지 않고 성공 증거로 과장할 수 있음 | **PASS** — README는 simulator/build/surface 검증을 하지 않았다고 명시한다. |
| 19 | 변경을 C11.4 완료 또는 API 계약 확장으로 승격할 수 있음 | **PASS** — 공식 checkbox, `72/213`, contract version과 공개 지원 범위는 변경하지 않는다. |
| 20 | 좁은 테스트 통과를 전체 Clippy 통과로 뭉뚱그릴 수 있음 | **PASS** — 원래 Clippy의 무관한 type-complexity 실패와 그 lint 하나를 허용한 재검사를 구분해 기록한다. |

**결과: PASS 20 / PARTIAL 0 / FAIL 0.** 이는 변경된 test assertions와 저장소 문서 diff에 대한 검토다. 앱 runtime, Simulator 결과, PR CI 또는 전체 C11.4 적합성 검증은 포함하지 않는다.
