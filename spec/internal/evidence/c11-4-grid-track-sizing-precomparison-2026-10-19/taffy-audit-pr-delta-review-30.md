# C11.4 Taffy 자동 최소 계획 수정 · PR 변경 검토 30개

**검토 대상:** Taffy source audit, A/B 단계 경계 수정, 감사 반영 계획 재검토.
**범위:** 문서 변경만 검토했다. Rust 구현이나 Grid runtime 지원을 승인하는 검토가 아니다.

| # | 변경에 대조한 실패 경로 | 결과 |
| ---: | --- | --- |
| 1 | pin 버전이 workspace lock과 불일치 | `Cargo.lock`의 Taffy 0.14.0 및 checksum을 기록했다. |
| 2 | GitHub source tag가 pin 버전과 불일치 | 링크 모두 v0.14.0 tag로 고정했다. |
| 3 | source hash가 다른 버전 파일에서 산출 | 로컬 registry의 0.14.0 source에서 두 SHA를 계산했다. |
| 4 | CSS 조건을 규범 원본 없이 설명 | CSS Grid 1 §6.6 및 track sizing normative source를 연결했다. |
| 5 | track 조건을 item 조건으로 혼동 | 계획은 item별 placed span에서 조건을 계산한다고 명시한다. |
| 6 | `axis_tracks`와 span slice를 같은 것으로 간주 | 감사는 전체 축 검사와 `item_axis_tracks` 사용을 구분해 기술한다. |
| 7 | auto-min 조건의 단수 span 예외 누락 | span 길이 1과 multi-span 조건을 별도 control로 요구한다. |
| 8 | flexible 예외의 검사 대상 누락 | span 밖 flex가 영향을 주지 않는 pair control을 추가한다. |
| 9 | 다른 위치의 auto track 영향 누락 | span 밖 auto가 영향을 주지 않는 control을 추가한다. |
| 10 | replaced item의 transferred suggestion을 모든 item에 적용 | plan review는 transfer를 replaced item의 ratio와 definite opposite size에 한정한다. |
| 11 | preferred specified suggestion을 content suggestion으로 덮음 | plan review에 specified suggestion 우선 검증이 있다. |
| 12 | content suggestion을 max-content 값으로 혼동 | plan review는 min-content 입력을 별도 확인한다. |
| 13 | max-size clamp 누락 | 계획의 C11.4 matrix와 신규 review가 축별 max constraint를 남긴다. |
| 14 | cyclic percentage를 final used size와 합침 | 기존 cyclic-percentage oracle 및 review 제약을 보존했다. |
| 15 | fixed max track과 fit-content clamp 혼동 | 신규 review는 fit-content limit을 fixed max track clamp와 분리한다. |
| 16 | padding/border가 두 번 더해짐 | A 입력 계약은 outer contribution의 edge 반영을 한 번으로 유지한다. |
| 17 | column contribution에 row basis 사용 | 계획은 양 축 실행과 axis별 입력 보존을 유지한다. |
| 18 | line-name를 track span으로 오인 | 변경은 named-line/area semantics나 C11.3 입력 경계를 바꾸지 않는다. |
| 19 | C11.3 기존 profile을 새 profile로 확장 | 새 intrinsic track profile은 B의 전용 profile로 분리했다. |
| 20 | Stage A가 placement가 필요한 자동 최소를 계산한다고 약속 | 계획에서 automatic minimum을 B의 placed-item 계산으로 이동했다. |
| 21 | Stage A image callback 재사용 범위 과장 | 계획은 측정 경로 재사용으로만 적고 Grid validation은 fixed-track 한정으로 남긴다. |
| 22 | text intrinsic definite snapshot 오염 | 기존 definite snapshot 보존 조건을 유지했다. |
| 23 | C11.4 stage/output trace 요구가 약화 | B의 단계별 base/growth limit 및 contribution recording 요구를 유지했다. |
| 24 | unsupported overflow fallback 은폐 | typed C13 overflow가 없는 조합의 fail-closed 조건을 유지했다. |
| 25 | Taffy 최종 frame을 독립 표준 oracle로 취급 | differential 후보와 CSS/Chrome 기준의 역할을 분리했다. |
| 26 | 계획 변경을 runtime 성공으로 과장 | audit와 review 모두 source/document review 한계라고 명시한다. |
| 27 | 공식 roadmap completion count를 조용히 올림 | C11.4 상태나 72/212 count를 바꾸지 않았다. |
| 28 | 공개 제품 CSS/API 지원 선언으로 확대 | 문서가 C11.4/C11 또는 공개 지원 완료가 아님을 명시한다. |
| 29 | 새 markdown 상대경로 오류 | plan의 audit/review 링크 target 존재를 직접 확인했고 `git diff --check`가 통과했다. |
| 30 | 문서 build 전체 실패를 이 변경 성공으로 둔갑 | RSPress build는 기존 다른 문서의 dead link 오류로 실패했다. build 통과로 보고하지 않는다. |

## 실행 검증과 남은 한계

- `git diff --check`: 통과.
- RSPress 2.0.22 docs build: 실패. 출력은 `c12-4-worker-stack-residency`, `s03-node-lifecycle-contract-fixture`, `c11-style-grid-projection` 등 이 변경 파일이 아닌 문서의 dead local links를 보고했다. 이 실패를 C11.4 audit 링크의 검증 성공으로 확대하지 않는다.
- Taffy, Chrome, Rust, WPT, Android API 37 AVD, iPhone 17 Pro/iOS 26.2 Simulator는 이 문서 변경에서 실행하지 않았다.
