# C11.4 anonymous Grid text computed `font-size` 계획 PR diff 검토 30개

계획 PR에 포함한 상태 문서·계획·Agent Browser fixture·사전 비교 evidence의 diff를 각각 다른 실패 경로로 검토했다. 이 기록은 계획 PR만 검토하며 구현/runtime 검토를 대신하지 않는다.

| # | PR diff 실패 관점 | 확인·방어 | 판정 |
| ---: | --- | --- | --- |
| 1 | C11 parent checkbox를 완료 처리 | `spec/STATUS.md` C11은 `[ ]` 유지 | 통과 |
| 2 | C11.4 checkbox를 완료 처리 | C11.4는 `[ ]` 유지 | 통과 |
| 3 | 공식 완료 수가 잘못 증가 | 새 status entry와 계획이 `72/213` 유지 | 통과 |
| 4 | 계획-only evidence를 runtime 결과처럼 기록 | status와 README에서 구현/runtime 결과가 없다고 구분 | 통과 |
| 5 | 앞선 line-height 계획 상태가 아직 구현 전으로 남음 | status entry에 #905 merge/evidence를 반영하고 한계를 유지 | 통과 |
| 6 | plan PR에 public roadmap 완료 항목을 추가 | `spec/public/roadmap.html`은 변경하지 않음 | 통과 |
| 7 | 이미 구현된 white-space slice 이름과 중복 | 신규 scope를 computed `font-size`로 고유하게 표시 | 통과 |
| 8 | 상세 계획과 C11.4 master plan이 서로 연결되지 않음 | `plan/c11-4-grid-track-sizing.md` tail에서 집중 계획·evidence를 링크 | 통과 |
| 9 | fixture가 저장소 밖 시스템 font에 의존 | HTML이 pinned 저장소 Ahem TTF를 상대 경로로 로드 | 통과 |
| 10 | Ahem file 교체 뒤 old evidence hash를 사용 | HTML/README/expected-values의 Ahem SHA-256이 파일과 일치 | 통과 |
| 11 | generated matrix가 10 cases가 아닌 다른 수 | 5 computed values × 2 track functions을 HTML·JSON·capture에서 대조 | 통과 |
| 12 | fixture page가 네트워크 또는 runtime app에 의존 | 로컬 file page와 상대 TTF만 사용하고 app assertion 없음 | 통과 |
| 13 | text를 anonymous direct child 대신 explicit element로 만듦 | 각 Grid root에 `XX XX` text node 하나만 append | 통과 |
| 14 | font weight/style 또는 Ahem family가 장면마다 바뀜 | Ahem 400 normal로 고정, loaded marker 기록 | 통과 |
| 15 | padding/border/gap/line-height가 font-size 효과와 섞임 | zero spacing·border·gap, absolute line-height 10px로 고정 | 통과 |
| 16 | glyph advance를 유추만 하고 확인 안 함 | pinned TTF `unitsPerEm`과 `X`/space hmtx를 직접 읽어 README에 기록 | 통과 |
| 17 | min-content width에 공백이 포함 | longest unbreakable word `XX`의 2em 산식과 값 5개 확인 | 통과 |
| 18 | max-content width에서 collapsed space를 제외 | `XX XX`의 두 글자·space·두 글자로 5em 산식 확인 | 통과 |
| 19 | min-content scene의 row height control 누락 | auto row와 forced wrap 두 lines, 20px row 기대값을 5 size에 기록 | 통과 |
| 20 | max-content scene의 row height control 누락 | one-line 10px row 기대값을 5 size에 기록 | 통과 |
| 21 | decimal scene 값이 browser가 관찰하지 않은 정밀도를 주장 | Chrome JSON에 7.5px/12.5px track 값을 보존 | 통과 |
| 22 | root width/height를 Grid track size로 혼동 | root frame과 `gridTemplateColumns`/`Rows`를 JSON에서 나눔 | 통과 |
| 23 | Chrome user agent reduced version을 exact binary로 표기 | Agent Browser doctor Chrome binary version과 UA reduced string을 각각 기록, `fullVersionList` 빈 배열 보존 | 통과 |
| 24 | screenshot이 다른 입력 또는 잘린 matrix를 보여줌 | screenshot의 10 labeled rows와 fixture source를 시각 대조 | 통과 |
| 25 | font가 load되기 전에 observation을 저장 | fixture `document.fonts` ready promise와 JSON `fontLoaded:true` 확인 | 통과 |
| 26 | 규범 reference 링크가 잘못된 section을 가리킴 | Agent Browser에서 Grid `#algo-content`가 §11.5인지, CSS Sizing·Fonts anchors를 확인 | 통과 |
| 27 | 기대값 표가 Chrome 관찰을 복사한 단일 oracle | font-table hmtx 산식과 browser JSON을 별도 파일·열로 분리 | 통과 |
| 28 | evidence README의 상대 경로가 checkout 밖으로 나감 | fixture/font/screenshot/script 상대 링크를 파일 존재성으로 점검 | 통과 |
| 29 | JSON matrix와 expected IDs/track 값이 다름 | parser check에서 10/10 ID·computed font-size·column·row와 root width 일치 | 통과 |
| 30 | 사전 비교 뒤 작업 브라우저나 다른 session을 종료 | named Agent Browser session을 닫고 doctor에서 기존 default PID 66343만 확인 | 통과 |

검토 중 규범 링크가 §11.3 algorithm overview를 가리키던 초안을 Agent Browser에서 anchor 확인 후 §11.5 `#algo-content`로 고쳤다. line-height status의 오래된 “구현 결과 없음” 문장도 #905 merge 근거와 맞췄다. `git diff --check`는 통과했고, plan review 20개와 이 PR diff review 30개는 분리했다. 전체 C11.4 구현, simulator, WPT, glyph paint 또는 공식 완료 승격은 검증 대상이 아니다.
