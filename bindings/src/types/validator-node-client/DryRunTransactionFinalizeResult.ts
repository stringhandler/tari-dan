// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { FeeCostBreakdown } from "..\\FeeCostBreakdown";
import type { FinalizeResult } from "..\\FinalizeResult";
import type { QuorumDecision } from "..\\QuorumDecision";

export interface DryRunTransactionFinalizeResult {
  decision: QuorumDecision;
  finalize: FinalizeResult;
  fee_breakdown: FeeCostBreakdown | null;
}
