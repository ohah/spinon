# C11.4 multi-span 로드맵 병합 SHA 보완 diff 검토 30개

**범위:** 이미 PR #411을 링크한 `spec/public/roadmap.html`의 2026-10-21 direct image multi-span 문단에 병합 표기와 merge SHA를 보완한 변경 한 줄.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 잘못된 PR 번호를 연결함 | GitHub에서 병합 상태의 PR 번호가 411임을 확인했다. |
| 2 | 병합 SHA가 원본 head SHA와 혼동됨 | merge commit 필드의 전체 SHA `f0192565197fe9b9a2f0e98b3e4b60ddf0d12d81`를 확인했다. |
| 3 | PR 링크의 저장소 owner/repo가 틀림 | URL은 `ohah/spinon/pull/411`을 가리킨다. |
| 4 | URL 경로가 PR이 아닌 이슈를 가리킴 | `pull/411` 경로로 확인했다. |
| 5 | 링크 문구가 작업을 오해시킴 | 기존 `PR #411` 링크를 `병합 PR #411`로 명확히 한다. |
| 6 | 완료 수가 늘어난 것처럼 보임 | 공식 상태/완료 집계를 바꾸지 않는 기존 문단을 유지했다. |
| 7 | C11.4 전체 완료로 오인됨 | 전체 C11.4 미완료 문장을 그대로 둔다. |
| 8 | PNG texel paint가 확인된 것처럼 보임 | 기존 미검증 범위 문장을 그대로 둔다. |
| 9 | CSS border paint 결과를 과장함 | 기존 border paint 미검증 문장을 그대로 둔다. |
| 10 | WPT 통과로 오해됨 | 기존 WPT 미완료 문장을 그대로 둔다. |
| 11 | PR 링크가 잘못된 장면 근거에 붙음 | 기존 direct image multi-span 문단의 PR 링크를 유지했다. |
| 12 | 링크가 text intrinsic 문단에 들어감 | 2026-10-21 image multi-span 문단의 끝을 수정했다. |
| 13 | 중복 PR 링크가 생김 | 해당 문단 안에 PR #411 링크는 하나다. |
| 14 | 중복 HTML id 또는 anchor 충돌 | id/anchor를 추가하거나 변경하지 않았다. |
| 15 | 문단 닫는 태그가 유실됨 | 기존 `</p>`는 그대로 보존했다. |
| 16 | 기존 evidence 링크가 삭제됨 | 계획, 실행 README, 실기기 화면 및 STATUS 링크를 유지했다. |
| 17 | 링크를 갱신하며 문단 앞 텍스트가 변함 | 문단 끝 링크 문구와 SHA 외에는 동일하다. |
| 18 | merge SHA가 잘리거나 타이포됨 | GitHub merge commit 전체 SHA와 정확히 대조했다. |
| 19 | SHA가 PR head commit을 가리킴 | `mergeCommit.oid` 필드에서 값을 가져왔다. |
| 20 | 현 main과 관련 없는 커밋을 가리킴 | `origin/main` 최신 커밋이 같은 merge SHA임을 확인했다. |
| 21 | HTML escaping이 필요한 문자를 잘못 삽입함 | URL과 텍스트의 `#`는 속성 밖 일반 텍스트에 있다. |
| 22 | 문장 구분자가 링크 경계에서 깨짐 | 기존 구분자와 같은 `·`를 앞에 두었다. |
| 23 | 링크 문구가 한국어 문서 규칙을 어김 | PR 설명 문구를 한국어로 썼다. |
| 24 | 외부 PR 링크가 preview 내부 경로와 충돌함 | GitHub 절대 URL로 지정했다. |
| 25 | 변경이 다른 roadmap 노트에 들어감 | 고유한 multi-span 문단 하나만 선택했다. |
| 26 | 수정 스크립트가 중복 문단을 조용히 수정함 | 변경 스크립트에서 marker가 정확히 하나인지 검사했다. |
| 27 | 예상 끝 문자열이 바뀌었는데 덮어씀 | 기존 PR 링크 suffix를 변경 전에 정확히 검사했다. |
| 28 | 상태 대장 파일이 함께 변경됨 | `spec/STATUS.md`는 diff 대상이 아니다. |
| 29 | 전체 roadmap 스크립트/칸반 계산이 바뀜 | 변경은 static note의 링크 문구와 SHA suffix로 제한된다. |
| 30 | PR 링크와 merge SHA가 실제 검토 시점과 불일치 | 병합 결과 확인 직후 현재 main merge SHA로 기록했다. |

## 상태

변경은 기존 PR 링크를 병합 상태로 명확히 하고 SHA를 추가하며 제품 동작·공식 지원 범위·C11.4 상태는 바꾸지 않는다.
