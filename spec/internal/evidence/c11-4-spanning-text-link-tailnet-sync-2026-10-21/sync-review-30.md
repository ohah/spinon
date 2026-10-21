# C11.4 Tailnet pass35 동기화 적대 검토 30개

검토 대상은 PR #416 병합 뒤 만든 selected root, RSPress 산출물, Tailnet 교체·백업, 원격 응답과 Agent Browser 화면이다. PR #416의 링크 diff 검토 30개와 별개다.

| # | 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | 로컬 source가 병합 전 commit을 가리키는가 | **통과.** HEAD와 `origin/main` tree가 일치하고 source commit은 `4eaab3fa`다. |
| 2 | pass34 선택본을 덮어써 과거 preview 입력이 없어지는가 | **통과.** pass35 입력은 별도 경로이며 pass34 root는 그대로 존재한다. |
| 3 | 복제 중 큰 evidence 파일이 손상되거나 누락되는가 | **통과.** APFS clone 뒤 전체 파일 path·SHA-256을 pass34와 대조했다. |
| 4 | clone 도구가 실제 복사로 fallback해 저장 공간을 소진하는가 | **통과.** 같은 APFS volume의 `cp -cR` clone을 사용했고 결과 hash 검증을 마쳤다. |
| 5 | selected root 안 symlink가 preview 밖 파일을 노출하는가 | **통과.** selected tree 검사에서 symlink는 허용하지 않는다. |
| 6 | 기존 C11.4 150-scene 근거가 pass35 root에서 달라지는가 | **통과.** 기존 mirror와 저장소 근거의 tree hash가 일치했다. |
| 7 | 새 spanning-text evidence mirror끼리 서로 다른 내용인가 | **통과.** pass34 mirror를 먼저 상호 검증하고 source를 세 목적지에 복사한 뒤 각각 대조했다. |
| 8 | `STATUS.md`의 공식 완료 체크를 잘못 바꾸는가 | **통과.** source의 상태 파일만 복사했고 C11.4 체크·집계는 미완료로 유지했다. |
| 9 | 공개 roadmap이 병합된 source와 다른 snapshot인가 | **통과.** source `spec/public/roadmap.html`을 selected `public/roadmap.html`로 복사했다. |
| 10 | 계획 문서가 이전 단계 버전으로 남는가 | **통과.** C11.4 track sizing과 image multi-span 계획을 현재 merged source에서 갱신했다. |
| 11 | RSPress가 다른 버전으로 빌드되는가 | **통과.** 고정된 RSPress `2.0.22`로 빌드했다. |
| 12 | build config가 잘못된 root나 output을 선택하는가 | **통과.** pass35 selected root와 고유 pass35 output 경로를 사용했다. |
| 13 | output directory가 기존 pass34 산출물을 덮어쓰는가 | **통과.** pass35 build 전에 해당 경로가 없음을 확인했다. |
| 14 | output이 root 밖이라 기존 파일을 정리하는가 | **통과.** RSPress 경고를 기록했고 새 경로에만 썼다. |
| 15 | build exit code만 성공하고 page가 빠져 있는가 | **통과.** roadmap과 evidence README HTML route의 존재·크기를 확인했다. |
| 16 | 이미지 링크가 HTML에는 있으나 selected site에 PNG가 없는가 | **통과.** 새 Android 및 iOS PNG와 결과 JSON target을 확인했다. |
| 17 | C11.4 문서가 여전히 `/spec/internal/...png` 경로를 만드는가 | **통과.** 생성 HTML에서 해당 잘못된 PNG route가 없고 21개 자산 target이 존재한다. |
| 18 | 새 화면 대신 이전 재실행 PNG를 표시하는가 | **통과.** Android href가 09:24 run ID와 동일한 경로를 가리킨다. |
| 19 | preflight 뒤 source output이 바뀌어 검증과 다른 bytes가 복사되는가 | **통과.** copy 단계가 preflight의 source size/hash를 다시 비교한다. |
| 20 | preview server PID를 오래된 값으로 가정하는가 | **통과.** sync 단계에서 8768 listener와 process command를 새로 발견했다. |
| 21 | preview 서버를 종료하거나 다른 포트에 쓰는가 | **통과.** PID 1191의 127.0.0.1:8768 listener를 유지했다. |
| 22 | target inventory가 오래된 pass34 수치인가 | **통과.** pass35 직전 30,463 files와 1,399,069,301 bytes를 재수집했다. |
| 23 | backup 경로가 이전 pass를 덮어쓰는가 | **통과.** pass35 전용 backup root가 없음을 확인하고 새 경로를 만들었다. |
| 24 | 교체가 원본 backup을 남기지 않는가 | **통과.** 125개, 13,507,290 bytes를 복사하고 개별 SHA-256을 확인했다. |
| 25 | 대상 inventory에 없는 파일을 암묵적으로 삭제하는가 | **통과.** sync plan의 delete 수가 0이며 copy 로직은 selected path만 쓴다. |
| 26 | 열려 있는 사용자 파일을 atomic rename하는가 | **통과.** 교체마다 직전 `lsof`를 실행하고 preview 이외 handle은 거부한다. |
| 27 | 로컬 preview에서 새 target 또는 backup bytes가 손상되는가 | **통과.** 2,445 target과 125 backups 모두 SHA-256 일치다. |
| 28 | Funnel이 HTTP 200을 반환해도 오래된 bytes를 제공하는가 | **통과.** 원격 2,445/2,445 응답 SHA-256이 source와 일치한다. |
| 29 | Agent Browser 화면에서 링크가 404나 fallback 문서로 열린다 | **통과.** roadmap·근거 페이지와 Android/iOS 이미지, JSON을 브라우저에서 확인했다. 내부 근거 링크 21개는 모두 200이다. |
| 30 | 화면 캡처를 글리프/overflow pixel 또는 전체 C11.4 완료로 확대 해석하는가 | **통과.** 문서와 로드맵은 box geometry만 확인했다고 밝히고 전체 C11.4 미완료를 유지한다. |

삭제는 없었다. pass34 입력과 Tailnet preview server는 보존했다.
