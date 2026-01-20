# Rust-based Threshold Signature Scheme (TSS) Implementation

## INTRODUCTION

이 프로젝트는 Distributed key generation 기술 중 하나인 TSS와 그것을 바탕으로 실제 트랜잭션을 전파하는 코드를 구현한 것입니다.
해당 프로젝트는 [multi-party-ecdsa](https://github.com/ZenGo-X/multi-party-ecdsa) 와 [Fast Multiparty Threshold ECDSA with Fast Trustless Setup](https://eprint.iacr.org/2019/114.pdf) 을 기반으로 구현되었습니다.

## INTRODUCTION

이 프로젝트는 Distributed key generation 기술 중 하나인 TSS와 그것을 바탕으로 실제 트랜잭션을 전파하는 코드를 구현한 것입니다.
해당 프로젝트는 [multi-party-ecdsa](https://github.com/ZenGo-X/multi-party-ecdsa) 와 [Fast Multiparty Threshold ECDSA with Fast Trustless Setup](https://eprint.iacr.org/2019/114.pdf) 을 기반으로 구현되었습니다.

## 프로젝트 구조

-- src/utils.rs: client - server 소통을 위한 모듈입니다.

-- src/client: client끼리 소통하기 위한 모듈입니다.

-- src/provider, transaction: transaction 생성 및 전파하는 코드입니다.

-- bin/manager: server를 구현한 코드입니다. manager는 client끼리 메시지를 주고받는 것을 도와줍니다.

-- bin/keygen: key를 생성하는 코드입니다.

-- bin/sign: 생성한 key를 바탕으로 서명 및 전파하는 코드입니다.

## installation

```
cargo build

// server 활성화
cargo run --bin manager
// keygen
// t와 n은 threashold와 n에 대한 숫자를 입력하면 됩니다. 만약 n이 3인 경우, 3개의 터미널에서 이 명령어를 실행해야 합니다. room은 서로 message를 주고받기 위한 room name으로 모든 참여자가 같아야 합니다.
// 정상적으로 종료되면 keys/roomname 안에 key 파일들이 생성됩니다.
cargo run --bin keygen -- --room roomname --threshold t --number-of-parties n

// sign: index는 가지고 올 key의 index를, parties는 이번 서명에 참가하는 key들의 index를 의미합니다.
// 예를 들어, key 1과 key 2를 가지고 서명을 만든다면 다음과 같이 명령어를 입력해야 합니다.
// cargo run --bin sign -- --room roomname --index 1 --parties 1 2 --to 0x.... --ethers 1
// cargo run --bin sign -- --room roomname --index 2 --parties 1 2 --to 0x.... --ethers 1
cargo run --bin sign -- --room roomname --index i --parties --to 0x.... --ethers 1
```

## 논문 리뷰
