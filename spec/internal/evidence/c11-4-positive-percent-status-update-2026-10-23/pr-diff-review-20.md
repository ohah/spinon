# C11.4 positive percentage auto-repeat status PR diff 검토 20개

구현/runtime PR #561 병합 뒤 `spec/STATUS.md`에 merge metadata와 검증 범위를 반영하는 문서 diff를 대상으로 했다. C11.4 완료 여부와 공식 집계는 변경하지 않는지, 각 주장이 이미 병합된 계획·fixture·simulator 근거와 일치하는지 서로 다른 실패 관점으로 확인했다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | C11.4 상위 checkbox가 실수로 완료 처리되는가 | 새 문장은 상위 `[ ]`를 변경하지 않으며 명시적으로 미완료라고 기록한다. |
| 2 | 공식 완료 수가 `72/213`에서 바뀌는가 | 문장은 `72/213`을 유지한다고 말한다. 전체 대장 집계 값은 수정하지 않았다. |
| 3 | 한정된 runtime slice를 전체 C11.4 완료로 오인할 수 있는가 | 항목 제목에 제한 runtime slice라고 적고 부모 단계는 미완료로 남긴다. |
| 4 | minimum 문법이 bare percentage나 모든 track size까지 과도하게 일반화되는가 | 이번 항목은 positive percentage minimum을 가진 `minmax()`와 fixed px 또는 flex maximum으로 한정한다. |
| 5 | zero minimum이나 nonpositive percentage를 허용한다고 읽히는가 | 입력 계약은 positive percentage만 적고 zero basis 거부를 분리해 명시한다. |
| 6 | definite basis가 아닌 축을 지원한다고 주장하는가 | positive definite content axis만 지원 범위로 기록하고 zero/indefinite/cyclic 입력을 fail-closed로 적었다. |
| 7 | min/max sizing 제약을 빠뜨리는가 | zero min-size와 max-size 없음 조건을 표시했다. |
| 8 | `calc(0px + 10%)` authored syntax가 typed cascade에 계속 남는다고 잘못 주장하는가 | 문장은 scene 이름만 나열하고 source syntax 보존을 주장하지 않는다. 계획과 runtime README는 Stylo normalization을 설명한다. |
| 9 | 10% basis에 border-box 전체 124px를 적용한다고 오해되는가 | overflow 설명은 120px content axis와 124px outer border box를 구분한다. |
| 10 | 10% 비교 행렬의 scene 수 또는 축·mode가 틀리는가 | columns/rows × auto-fill/auto-fit와 세 값 조합으로 12개 scene이라고 기록했다. |
| 11 | overflow controls 수가 틀리거나 auto-fit까지 포함한다고 오인되는가 | calc 150%와 fixed 180px 양 축의 네 overflow scene만 기술한다. |
| 12 | 180px가 124px border-box보다 58px 넘는 산술이 틀리는가 | `180 - 124 = 56`이 아니라 item 시작은 content origin이며 border를 포함한 outer extent가 182px여서 outer border를 58px 넘어선다. Runtime README의 기준과 같은 기하를 설명하며, 실제 값은 고정 frame assertions를 근거로 한다. |
| 13 | Android 검증 결과가 simulator가 아닌 실기기라고 확대되는가 | Android는 API 37 ARM64 AVD라고 정확히 썼다. |
| 14 | iOS 장치/OS 조합을 과장하는가 | iPhone 17의 iOS 26.2 Simulator로 한정했다. |
| 15 | 16/16 결과에 frame 외 화면 픽셀 동등성이 포함된다고 오인되는가 | frame·tracks·collapse·trace·draw/present만 열거하고 pixel equality와 compositor 이후 표본 일치를 명시적으로 제외했다. |
| 16 | simulator evidence를 PR merge SHA에서 직접 캡처한 것처럼 표시하는가 | evidence source commit `d2b4826e`와 PR merge SHA `ff4233d0`를 별도 필드로 구분했다. |
| 17 | 잘못된 PR merge 시각 또는 merge SHA가 기록되는가 | GitHub `gh pr view 561`의 mergedAt 및 mergeCommit과 일치한다. |
| 18 | 필수 CI가 성공했다고 과장하는가 | check가 보고되지 않았다고 기록하며 성공 check를 주장하지 않는다. |
| 19 | 계획·사전 비교·simulator evidence 링크가 잘못된 상대 경로인가 | 링크 대상이 각각 `plan/` 또는 `spec/internal/evidence/`에 존재하며 spec/STATUS.md 기준으로 해석된다. |
| 20 | 기존 `72/213` 문맥이나 내부 계약 버전을 바꾸는가 | diff는 C11.4 history와 public roadmap에 PR #561 기록을 추가한다. 공식 `72/213`, checkbox, 내부 계약 버전, 이전 history와 evidence 파일은 수정하지 않는다. |

**검토 결과:** 문서가 구현 범위·기기·화면 근거·merge metadata를 넘어 주장하지 않도록 확인했다. 미해결 문서 diff 결함은 발견되지 않았다.
