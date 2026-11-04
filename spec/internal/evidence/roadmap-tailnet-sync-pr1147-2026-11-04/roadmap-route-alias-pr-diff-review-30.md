# PR #1147 Tailnet 경로 별칭 수정 diff 검토 30개

검토 대상은 공개 roadmap의 Chrome precomparison 링크를 `/spec/internal/` 별칭으로 옮기는 단일 href 수정과 이 검토 기록이다. 기존 Tailnet 경로 충돌, 선택 Rspress publication 경로, merged source 내용을 대조했다.

| # | 실패 관점 | 대조 및 판정 |
|---:|---|---|
| 1 | 기존 경로가 디렉터리라고 잘못 가정 | Tailnet의 기존 `internal/evidence/c11-root-html-grid-mixed-placement-precomparison-2026-11-04`는 regular JSON file이다. PASS. |
| 2 | 기존 파일을 directory로 덮어씀 | 새 href는 기존 파일 경로를 건드리지 않고 별칭 prefix를 바꾼다. PASS. |
| 3 | 충돌 파일 내용이 소리 없이 변경 | 기존 파일 SHA-256 `32acf59e8479007611ef9a0540df13e95208e6abf3a59f9c7841cd3233ffc142`를 기록했다. PASS. |
| 4 | 새 route도 같은 파일 충돌을 공유 | 선택 publication의 `/spec/internal/evidence/...` 경로는 regular `README.html`로 존재한다. PASS. |
| 5 | 잘못된 evidence directory 이름 사용 | source의 directory 이름과 새 href의 C11 mixed placement precomparison 경로가 일치한다. PASS. |
| 6 | README 대신 다른 산출물로 이동 | 새 route는 해당 directory의 `README.html`을 가리킨다. PASS. |
| 7 | `/spinon/` base 누락 | 새 로컬 href는 configured `/spinon/` base로 시작한다. PASS. |
| 8 | 절대 경로/상위 경로 traversal | href에는 `..`, 역슬래시, URI escape가 없다. PASS. |
| 9 | 잘못된 capitalization으로 route 누락 | source와 route 모두 `spec/internal/evidence` 대소문자가 일치한다. PASS. |
| 10 | 선택 publication에 route가 없음 | 보존된 Rspress 2.0.22 선택 publication의 대상 파일을 확인했다. PASS. |
| 11 | route 파일을 symlink로 취급 | 선택 publication 대상은 regular file이며 symlink가 아니다. PASS. |
| 12 | precomparison 내용이 다른 후보 자료 | 대상 파일 SHA-256은 `d4b8d0bbcddbd357dab89a7146362443332a600a056765db9969142658451ec2`다. PASS. |
| 13 | 링크 변경이 구현 범위 승격을 동반 | 변경은 href 하나뿐이며 지원 계약이나 완료 상태는 바뀌지 않는다. PASS. |
| 14 | 공식 완료 수 변경 | roadmap의 `72/213`은 유지된다. PASS. |
| 15 | C11.1을 전체 완료 처리 | C11.1과 C11 미완료 문구가 그대로다. PASS. |
| 16 | merge SHA가 링크 변경 중 손상 | 카드의 구현 PR #1147 merge SHA `c3b3c3d0ec8f841562b0a724ff596d65b0a3b83a`는 그대로다. PASS. |
| 17 | PR link를 precomparison route와 혼동 | 외부 GitHub PR #1147 href와 로컬 comparison href는 별도다. PASS. |
| 18 | runtime evidence route까지 변경 | Android/iOS runtime README와 PR diff review의 경로는 변경하지 않는다. PASS. |
| 19 | plan 문서의 상대 링크를 끊음 | 계획은 기존 `../spec/internal/evidence/.../README.md` source 경로를 계속 쓴다. PASS. |
| 20 | URL fragment가 잘못된 파일로 이동 | 이 변경 링크에는 fragment가 없으며 README 기본 route다. PASS. |
| 21 | pathname URL decoding 필요 | route segment에 인코딩 문자가 없어 그대로 해석된다. PASS. |
| 22 | static server rewrite 규칙에 기대 | 목적지는 일반 디렉터리/파일 route이며 server 설정을 바꾸지 않는다. PASS. |
| 23 | server를 재시작해야 하는 변경 | HTML href 변경만으로 기존 preview server 재시작이 필요하지 않다. PASS. |
| 24 | 기존 preview route 자료 정리·삭제 | 기존 충돌 파일은 보존하며 삭제 대상이 없다. PASS. |
| 25 | publication alias가 기록된 source와 다름 | 새 alias는 `spec/internal/evidence`의 tracked source tree에 대응한다. PASS. |
| 26 | local link count가 조용히 변함 | roadmap card의 링크 수는 그대로고 comparison 대상만 옮긴다. PASS. |
| 27 | selected build 범위를 전체 문서 검증으로 확대 | 이 변경과 검토는 전체 문서 사이트 link validation을 주장하지 않는다. PASS. |
| 28 | Tailnet HTTP 확인 전 이미 배포됐다고 주장 | 문서는 Tailnet 게시 성공을 주장하지 않는다. PASS. |
| 29 | 화면 캡처를 비교 결과로 오인 | 해당 링크 이름은 Chrome 비교이며 simulator screenshot이나 pixel readback이 아니다. PASS. |
| 30 | route 수정 근거를 재현 불가능하게 남김 | 선택 publication path, 두 SHA 및 기존 collision type을 검토 파일에 고정했다. PASS. |

이 검토는 경로 충돌을 피하는 source href 수정만 다룬다. 실제 Tailnet 반영은 merged main을 기준으로 별도 확인한다.
