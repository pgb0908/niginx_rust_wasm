# 배포와 롤백 초안

## 배포 단위

- Wasm 파일은 플러그인별로 독립 버전 관리한다.
- route, tenant, plugin 설정은 `config/` 디렉터리 단위로 함께 배포한다.
- 호환성 검증은 `config/compatibility.yaml` 기준으로 수행한다.

## 권장 절차

1. 새 Wasm 파일 빌드
2. `config/` 변경과 함께 이미지 생성
3. `nginx -t`로 설정 검증
4. canary 인스턴스에 우선 배포
5. access log와 plugin decision 지표 확인
6. 이상 없으면 전체 rollout

## 롤백 절차

1. 이전 gateway 이미지 태그 선택
2. 이전 `config/` 세트 복원
3. gateway 재기동 또는 reload
4. `401` 비율, `5xx` 비율, plugin error 비율 재확인

## 확인 포인트

- `auth-filter` 차단 비율이 배포 직후 급증하지 않는지
- `x-gateway-filter-version`이 기대한 버전인지
- tenant별 오류 편차가 없는지
- route별 fallback 동작이 정책과 일치하는지
