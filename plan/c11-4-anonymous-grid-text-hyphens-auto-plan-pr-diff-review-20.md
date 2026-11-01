# C11.4 `hyphens:auto` 계획 PR diff 적대 검토 20개

기준: 전체 계획 PR staged diff (계획, C11 roadmap 동기화, 계획 검토, Chrome/Hypher 사전 비교 HTML·JSON·화면). 계획 자체 실패 관점 20개와 다른 PR 변경 검토다.

| # | 변경 실패 관점 | 결과 |
|---:|---|---|
| 1 | C11 roadmap 상단 상태가 최신 `hyphens:none|manual` 병합을 누락 | 발견 후 수정: 상태 요약에 PR #977을 추가했다. |
| 2 | roadmap의 “다음 계획”이 이미 병합된 none/manual 작업을 재선택 | 발견 후 수정: 다음 후보를 `hyphens:auto`로 바꾸고 새 계획/evidence를 연결했다. |
| 3 | 계획/roadmap 문구가 C11.4 또는 C11 전체를 완료로 세거나 72/213을 올림 | 통과: 미완료와 공식 수를 유지한다. |
| 4 | PR #977의 결과나 링크가 실제 status record와 다름 | 통과: source plan/evidence 경로와 확인값을 사용한다. |
| 5 | CSS Text 링크가 `auto` 사전 및 min-content 규칙을 직접 가리키지 않음 | 통과: §5.3 anchor를 인용한다. |
| 6 | CSS Text가 규정하지 않은 breakpoint를 Chrome parity로 요구 | 통과: 계획과 사전 비교 모두 정확한 지점의 UA별 차이를 보존한다. |
| 7 | Chrome 결과가 자동 하이픈 glyph paint 또는 Spinon runtime 결과로 표현 | 통과: 브라우저 관측/DOM range 분절/Spinon 구현 증거를 구분한다. |
| 8 | Agent Browser HTML과 측정 대상 간 padding/border 때문에 intrinsic 폭이 어긋남 | 발견 후 수정: 크기 계산에 영향 없는 `outline`을 쓰고 최종 화면을 재캡처했다. |
| 9 | 사전 비교 screenshot이 수정 전 HTML을 보여주거나 stale hash로 연결 | 통과: 최종 HTML·화면을 다시 캡처하고 SHA-256을 README/JSON과 대조했다. |
| 10 | Ahem resource 경로가 잘못되거나 다른 font를 조용히 사용 | 통과: 저장소 경로와 font-loaded 관측, Ahem hash를 기록한다. |
| 11 | Browser version/measurement method가 재현 불가 | 통과: Chrome/Agent Browser version, CSS profile, Range grouping 방법을 기록했다. |
| 12 | DOM line text 표가 자동 삽입 hyphen glyph를 원문 문자로 오인 | 통과: 표와 JSON은 원문 UTF-16 분절만 표시하고 glyph를 제외한다고 설명한다. |
| 13 | Hypher version/license/feature 정보가 다른 release에 기대거나 최신 가변 값에 의존 | 통과: `=0.1.8`, exact feature list와 lock checksum을 기록하고 upstream links를 둔다. |
| 14 | `default-features=false`를 빼고 다국어 resource set을 제품에 포함 | 통과: plan은 English-only optional dependency와 artifact negative/positive checks를 요구한다. |
| 15 | no-alloc Hypher API에 길이 45바이트 초과가 들어가 panic | 통과: 정확한 word allowlist와 API 전 byte-length guard를 요구한다. |
| 16 | 예상 밖 dictionary segments가 문자를 누락하거나 빈 분절을 만들고도 성공 | 발견 후 보완: plan은 non-empty segments와 원문 재결합 equality를 요구한다. |
| 17 | Hypher breakpoint가 ASCII 문자 중간 밖으로 나가거나 source offsets를 소비 | 통과: ASCII boundary와 normalized-only synthetic marker/source-map invariants를 명시했다. |
| 18 | automatic hyphen advance가 max-content에 들어가거나 줄이 끊기지 않아도 추가 | 통과: max-content 원문폭 control과 line-ending marker 확인을 요구한다. |
| 19 | language 미상, locale 불일치 또는 U+00AD 혼합이 English dictionary로 fallback | 통과: 정확한 `en` allowlist와 missing/other language/U+00AD expected rejects를 둔다. |
| 20 | 계획 PR의 20회 diff review가 계획 20회 검토나 후속 구현 review와 합산됨 | 통과: 이 파일을 별도 PR diff review로 기록하고 plan/runtime review와 분리했다. |

차단 결함은 남지 않았다. #1, #2, #8, #9, #16에서 발견한 문서·evidence 결함을 수정한 최종 diff를 재확인했다. Git diff whitespace check와 observation JSON/hash 검증도 통과했다.
