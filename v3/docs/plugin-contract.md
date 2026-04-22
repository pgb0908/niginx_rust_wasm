# 플러그인 계약

`v3`에서 각 플러그인은 아래 공통 규칙을 따른다.

## 실행 순서

1. `auth-filter`
2. `header-filter`
3. `observe-filter`

앞 플러그인이 요청을 중단하면 뒤 플러그인은 실행되지 않는다.

## 공통 헤더

- `x-request-id`
  - 요청 식별자
- `x-trace-id`
  - 추적 식별자
- `x-tenant-id`
  - tenant 식별자
- `x-gateway-route`
  - gateway가 해석한 route 이름
- `x-gateway-policy-profile`
  - route 또는 tenant에 의해 선택된 정책 프로필
- `x-gateway-plugin-chain`
  - 현재 활성화된 플러그인 체인
- `x-gateway-filter-version`
  - 응답을 남긴 플러그인 버전
- `x-gateway-decision`
  - `allow` 또는 `deny`
- `x-observe-span`
  - 관측성 연동용 span 식별자

## 실패 규칙

- 인증 플러그인은 기본적으로 `fail-close`
- 헤더 표준화 플러그인은 기본적으로 `fail-open`
- 관측성 플러그인은 기본적으로 `fail-open`

## 확장 규칙

- 새 플러그인은 기존 공통 헤더를 삭제하지 않는다.
- route 판단 결과는 `x-gateway-route`로 전달한다.
- 차단 응답은 JSON 형식으로 통일한다.
- 플러그인 버전은 `name/version` 형식으로 남긴다.
