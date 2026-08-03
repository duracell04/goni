# Roofline Primer (Local ITCR)

This appendix provides a concise roofline framing for local inference-time
compute reasoning (ITCR). It defines why decoding is typically memory-bound and
how that shapes platform contracts.

## 1. Arithmetic intensity

Arithmetic intensity is:

I = FLOPs / Byte.

For a fixed hardware roofline, performance is bounded by the smaller of:

- compute roof: peak FLOPs,
- memory roof: sustained bandwidth * I.

Low-I workloads are memory-bound; high-I workloads are compute-bound.

## 2. Why decoding is usually memory-bound

Autoregressive decoding repeatedly reads model weights and KV cache for each
token. The reuse per byte is limited, so I stays low. Prefill/encoding can
exhibit higher I due to larger batch/context reuse, but decode is still dominated
by memory traffic.

Consequence: Increasing TOPS alone does not improve latency unless memory
traffic is reduced or bandwidth increases.

## 3. Platform implications for Goni

- Prefer hardware with stable, high sustained bandwidth and predictable latency.
- Treat KV cache residency and paging strategy as first-class system concerns.
- Route memory-bound stages to the highest-bandwidth path; avoid unnecessary
  host-device copies.

These are encoded as platform requirements in `blueprint/hardware/10-requirements.md`.
