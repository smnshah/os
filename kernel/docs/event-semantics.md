# Event Semantics Specification

Status: Draft  
Audience: Kernel developers, userspace collector developers, researchers

TODO: Provide formal definition of kernel event.

## 1. Purpose

This document defines the event model and causal semantics for the kernel event system.
It is the normative reference for:

- what the kernel guarantees,
- what it does not guarantee,
- how event loss is represented,
- and how userspace reconstructs the causal graph.

## 2. Design Goals

- Preserve causal soundness at event creation time.
- Keep kernel event recording bounded-time and allocation-free in hot paths.
- Support multicore systems without imposing a fake global total order. The kernel supports a causally correct partial order.
- Make overload and loss explicit (no silent loss).
- Allow userspace to build, persist, and analyze the full causal DAG.

## 3. Non-Goals

- Unbounded in-kernel retention of all historical events.
- A globally total-ordered event stream across all cores.
- Zero instrumentation overhead.
- Guaranteed full-history capture under arbitrary load.

## 4. Event Model

An event is an immutable record with:

- `id: EventId`
- `kind: EventKind`
- `cause: Cause`
- `data: EventData`

### 4.1 EventId

`EventId` is globally unique by construction:

- `core: u16`
- `sequence: u64`

`sequence` is monotonic per core.

### 4.2 Cause

`Cause` encodes causal edges:

- `Root(RootCause)`: starts a causal chain.
- `CausedBy(EventId)`: direct parent event.

The parent reference is authored by the kernel at the point where causality is known.

## 5. Buffering Architecture

Kernel buffering is per-core and bounded (ring buffer). This buffer is a staging layer, not the canonical long-term store.

Responsibilities:

- Kernel:
  - capture events,
  - assign IDs,
  - attach cause edges,
  - expose drain/read interfaces,
  - emit explicit loss metadata.
- Userspace:
  - drain promptly,
  - persist durable logs,
  - materialize indexes/graph structures,
  - perform analysis and query.

## 6. Guarantees

The kernel guarantees:

1. Per-core total order for accepted events by increasing `sequence`.
2. Causal edge soundness: each non-root event carries an explicit parent reference.
3. No fabricated global order across cores.
4. Loss transparency: overload loss is explicit and machine-readable.
5. Bounded recording behavior suitable for critical paths (no dynamic allocation).
6. Kernel stability isolation: collector failure may reduce capture fidelity, but must not compromise kernel correctness.

## 7. Non-Guarantees

The kernel does not guarantee:

1. Full-history completeness under arbitrary sustained overload.
2. Global total order across all CPUs.
3. Zero perturbation of runtime behavior.

## 8. Overflow and Loss Semantics

When producers outpace drain capacity, ring buffers can overwrite old data.
This is expected behavior for bounded non-blocking capture.

Required semantic rule:

- loss MUST be represented explicitly as events (for example dropped sequence ranges/counters), never silently.

This allows downstream tools to distinguish:

- complete causal segments,
- and segments with known fidelity gaps.

## 9. Causal Completeness Definition

For this kernel, "causal completeness" is defined as:

1. causal links are correct for all retained events,
2. and any missing events are explicitly accounted for.

This is stronger than best-effort tracing and weaker than impossible "no-loss under all loads."

Operationally, full-history completeness is conditional on sustained drain throughput meeting or exceeding event production.

## 10. Multicore Semantics

- Per-core ordering is primary.
- Cross-core causality is represented only through explicit parent references across cores.
- No implicit ordering is inferred from timestamp or core ID alone.

Cross-core mechanisms (IPC, IPI, scheduler wakeups, ownership transfers) must propagate causal context explicitly.

## 11. Invariants

The implementation should maintain:

1. `EventId(core, sequence)` uniqueness.
2. Monotonic `sequence` per core.
3. `CausedBy(parent)` references either a retained event or an explicitly dropped range that contains it.
4. Overflow metadata is emitted on every detected loss condition.




