# PR #614 C11.4 fixed-repeat `minmax(px, fr)` 계획 diff 적대 검토 30개

**검토한 PR head:** `efc6d922c5721da01e55779416e96eda1214e631`
**기준 main:** `18204bfd9979cd37061eea1c671a78267fb209e8`
**검토 범위:** 실제 PR diff와 PR 본문, 계획, 첫 계획 검토 20개, 개정 계획 검토 20개, 사전 비교 README·JSON·Chrome observation·HTML fixture·PNG, `spec/STATUS.md`, 정적 `spec/public/roadmap.html`, 저장소 PR template.
**방법:** 읽기 전용 비교와 파일/hash/링크 확인만 했다. 테스트·빌드·브라우저는 실행하지 않았다. PR 본문 링크와 embedded raw PNG는 HTTP HEAD 200, PNG 형식·12,113 bytes·1280×633으로 확인했다.

| # | 적대 관점 | 구체적 실패 경로와 근거 | 판정 |
|---:|---|---|---|
| 1 | 의도치 않은 변경 섞임 | PR diff는 계획·리뷰·사전 비교 자료·fixture와 상태/roadmap 한 줄, 총 10개 경로만 추가·갱신한다. 제품 runtime/스타일/layout 구현 파일은 없고, 본문도 계획 전용이라고 밝힌다. | **통과** |
| 2 | 오래된 base 위에서의 문서 작성 | PR base SHA `18204bfd…`와 checkout의 `origin/main`이 일치한다. 현재 main과 다른 기준 위에 문서가 놓인 경로는 없다. | **통과** |
| 3 | 사전 비교를 앱 구현 증거로 오인 | PR 본문은 앱 runtime/시뮬레이터를 실행하지 않았다고 표시하고 Chrome 자료를 구현 증거와 분리한다. Android/iOS 실행·기본/opt-in artifact 대조·PR screenshot 확인은 계획의 미래 관문으로 남기며 실기기, hardware GPU, APK/IPA 전체 배제 또는 pixel equality로 확대하지 않는다. | **통과** |
| 4 | 계획 PR에 필요한 분류 누락 | PR에는 `planning` 및 `area: layout` 라벨이 붙어 있다. 작업 유형·영역 라벨 요구를 만족한다. | **통과** |
| 5 | PR template 확인란의 허위 체크 | PR 본문은 STATUS 갱신과 API 변경 없음, 계획에 범위/오류/검증 관문 기재를 체크했다. 실제 플랫폼 동작 근거 항목은 미체크로 남겨 계획 PR의 증거 범위를 과장하지 않는다. | **통과** |
| 6 | 계획 병합을 C11.4 완료로 승격 | 공식 상위 C11.4 행은 여전히 `[ ]`다. 새 상태 행은 구현이 미시작이라고 명시하며 완료 체크를 추가하지 않았다. | **통과** |
| 7 | 완료 수 집계의 우발적 변경 | STATUS diff는 새 설명 행 하나만 더한다. 체크박스 완료 상태 변경은 없고 새 행 자체에도 체크 표시가 없다. PR·roadmap의 공식 집계 72/213과 일치한다. | **통과** |
| 8 | STATUS 요약과 상세 계획의 범위 불일치 | 추가된 STATUS 행은 1–2 track pattern, columns/rows·fill/fit·다중 fixed group, 0.5fr/0fr, 독립 기대값 및 구현 미시작을 요약한다. 상세 계획과 사전 비교의 여섯 scene 범위를 벗어난 기능 완료를 주장하지 않는다. | **통과** |
| 9 | 공개 roadmap에서 후보를 구현 완료처럼 노출 | HTML 항목은 “계획 초안”, C11.4 미완료, 72/213 유지, 구현 미시작을 표시한다. 구현 PR·runtime 결과·체크리스트 완료를 새로 주장하지 않는다. | **통과** |
| 10 | 정적 roadmap의 계획/evidence route 오기 | 새 계획·두 리뷰·README `.html` route와 PNG 경로는 추가된 실제 파일을 가리킨다. Rspress 문서 링크의 기존 `/spinon/...html` 규칙과 맞는다. | **통과** |
| 11 | STATUS에서 새 리뷰 자료로 가는 깨진 링크 | STATUS의 계획, 첫 20-view, 개정 20-view, 사전 비교 README 상대 경로가 모두 PR에 추가된 실제 파일을 가리킨다. 경로 깊이와 파일명이 일치한다. | **통과** |
| 12 | negative/non-finite typed operand가 Taffy까지 유입 | 음수·NaN·±infinity computed operand를 CSS parser 오류로 오분류하거나 node/property 없는 profile 오류로 내보낼 수 있다. JSON은 이들을 synthetic input으로 분리하고 CSS syntax 판정과 무관하게 Taffy 전 `UnsupportedComputedGridValue` 및 node/property 보존을 요구한다. | **통과** |
| 13 | PR 본문이 보정 전 계획 SHA를 가리킴 | 최초 본문 링크는 수정 전 `5d203147`을 가리켜 반올림 결함이 남은 문서를 노출할 수 있었다. 최종 본문은 계획·두 리뷰·README·이미지를 모두 최종 head `efc6d922…`에 고정했고 네 문서 및 이미지 URL이 모두 HTTP 200이다. | **보정 후 통과** |
| 14 | PR 본문 embedded image가 존재하지 않거나 잘못된 타입 | raw GitHub URL은 최종 head의 diff 내 PNG 파일로 응답하며 HTTP 200과 `image/png`를 반환한다. 단순 Markdown 텍스트 URL이 아니며 PR 본문에 실제 이미지 삽입 문법을 사용했다. | **통과** |
| 15 | 화면 증거가 다른 크기/파일인데 metadata만 맞음 | 로컬 PNG SHA-256은 README의 `aef6a7e5…`와 일치하고 12,113 bytes·1280×633이다. 관찰 JSON의 viewport 1280×633, DPR 1과 크기 조건이 맞는다. | **통과** |
| 16 | 사전 비교 README의 상대 링크가 다른 fixture를 선택 | README의 HTML fixture·Chrome observation·기대값 JSON·PNG 상대 경로가 모두 같은 evidence 디렉터리 구조를 가리킨다. HTML 입력과 observation/PNG의 hash도 README에 기록된 값과 일치한다. | **통과** |
| 17 | 최종 기대값 JSON이 기록된 hash와 달라짐 | 수정된 기대값 JSON SHA-256 `dc76dc21…`은 README 표의 값과 일치한다. 계획 변경 후 기대값 JSON만 바뀐 hash가 검토 기록에서 최종 상태와 구분되어 있다. | **통과** |
| 18 | Chrome 관찰 파일과 scene 누락/중복 | observation과 독립 기대값은 모두 6개 고유 ID `columns-fill`, `columns-fit`, `rows-fill`, `rows-fit`, `columns-groups`, `columns-flex-factor-low`를 같은 순서로 포함한다. | **통과** |
| 19 | 화면 라벨과 JSON scene ID가 서로 다른 장면을 가리킴 | HTML의 `columns-groups` element에 보이는 제목은 `columns-multiple-groups`지만 CSS selector/id, Chrome observation 및 기대값 ID는 동일한 `columns-groups`다. 하나의 장면을 설명하는 label 차이이며 추가/누락 장면은 없다. | **통과** |
| 20 | viewport/grid 크기 또는 축 설정이 기대값과 다름 | fixture는 기본 grid를 200×120px, gap 4px로 고정하고 columns 그룹 장면만 폭 300px로 바꾼다. rows 장면은 명시 column 200px와 row repeat, viewport는 README/observation에서 고정되어 장면별 축 혼동을 막는다. | **통과** |
| 21 | auto-repeat count 계산에서 minimum/gutter를 잘못 산정 | 독립 README 산술은 `28m+28`, `28m+92`, `28m+12` 경계를 따로 제시하고 각각 6·7·6 반복 수를 산출한다. JSON에는 입력별 count inequality와 결과가 있으며 Chrome observation과 별도 출처로 둔다. | **통과** |
| 22 | flex freeze·배분 순서가 허용오차로 가려짐 | six-scene 기대값은 1fr/2fr 최소값 동결과 suffix flex 결과를 track 단위로 제시한다. 관찰값과 기대값은 별도로 저장되어 산술 기대를 Chrome 결과로 덮어쓰지 않는다. | **통과** |
| 23 | auto-fit placement/collapse 후 item line 또는 gutter가 달라짐 | columns/rows fit 장면은 점유 auto track, collapse된 track index, exact zero, fixed suffix item 시작점과 전체 track 결과를 명시한다. CSS fixture도 item을 line 1·7·8 및 row 1·4·5에 배치한다. | **통과** |
| 24 | `rows-fit` fractional item offset을 exact 비교해 정상 CSSOM 값을 거부 | 이전 문구는 관찰 y `[41.328125, 82.65625]`와 독립값 `[124/3, 248/3]`을 동시에 exact로 요구하는 실제 모순이었다. 최종 계획·JSON·README는 fractional track과 item offset에 1/64 CSS px, 정수 offset과 collapsed zero에 exact를 적용한다. 계산한 최대 offset 차이 0.0104167px는 1/64=0.015625px 이내다. | **발견 후 보정, 최종 통과** |
| 25 | Chrome 관찰을 규범 oracle로 승격 | README는 Chrome 156을 구현 비교기로 한정하고 CSS Grid 2025 CRD에서 독립 계산한 expectation JSON을 분리한다. CSSOM 양자화값도 규범 rational과 다른 출처로 남긴다. | **통과** |
| 26 | 문법 invalid와 profile 미지원 입력을 같은 오류로 취급 | JSON은 valid percentage/intrinsic controls를 `UnsupportedComputedGridValue`, invalid `minmax(1fr,12px)`을 CSS parser rejection으로 분리한다. plan도 syntax-invalid flex minimum을 profile 진단에 넣지 않는다. | **통과** |
| 27 | authored calc 표기만으로 computed 타입을 오판 | 계획과 JSON은 `calc(20px)`가 Stylo에서 `LengthPx(20)`로 정규화된 경우만 허용하고 unresolved computed `Calc`는 거부한다. 새 evaluator나 authored-text 기반 admission을 범위에 넣지 않는다. | **통과** |
| 28 | finite 개별 fraction의 aggregate가 Taffy에서 f32 infinity | 계획/기대값은 fixed-repeat의 네 `1e38fr` 합과 최대 auto-repeat 확장 합을 각각 Taffy 진입 전 거부하도록 분리한다. ordinary/fixed/auto factor의 expanded-axis upper bound도 별도 projection gate로 명시한다. | **통과** |
| 29 | projection descriptor cap과 Taffy used-track cap 혼동 | JSON은 component와 projected descriptor의 10,000/10,001 경계를 분리하고 auto-repeat pattern은 projection에서 한 번만 센다고 명시한다. Taffy의 실제 used-track 10,000 ceiling을 같은 개수로 취급하지 않는다. | **통과** |
| 30 | line metadata 예산 초과 또는 reserve 실패 후 부분 결과 | JSON/계획은 expanded metadata 1,048,576/1,048,577-byte 경계와 MinMax 경로의 track/group/name reservation failpoint를 요구한다. 초과/실패 시 부분 결과가 없어야 하고 같은 worker의 다음 유효 호출이 성공해야 하며 named seam도 보존해야 한다. | **통과** |

## 결론

30개 관점 중 미해결 blocker는 없다. #24에서 계획의 fractional item offset tolerance 모순을 발견해 부모가 PR follow-up `efc6d922…`에서 계획·JSON·README를 보정했다. 최종 문서의 plan SHA-256은 `076d8c3f2afa2363cd7778d3a07939592b6d8556a3062f648fc643cbe7a80e01`, 기대값 JSON SHA-256은 `dc76dc2198b649ff573d38ee26db99918819be417f752ec40f44172c78555acd`이고 README 및 observation과 대조했다. PR 본문 링크와 embedded image도 최종 head를 가리키도록 갱신되어 모두 응답한다. 이 검토는 문서/정적 근거 검토이며 테스트·빌드·브라우저 통과를 뜻하지 않는다.
