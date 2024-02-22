// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Amount } from "./Amount";
import type { Arg } from "./Arg";
import type { ComponentAddress } from "./ComponentAddress";
import type { ConfidentialClaim } from "./ConfidentialClaim";
import type { ConfidentialOutput } from "./ConfidentialOutput";
import type { LogLevel } from "./LogLevel";

export type Instruction =
  | { CallFunction: { template_address: Uint8Array; function: string; args: Array<Arg> } }
  | { CallMethod: { component_address: ComponentAddress; method: string; args: Array<string> } }
  | { PutLastInstructionOutputOnWorkspace: { key: Array<number> } }
  | { EmitLog: { level: LogLevel; message: string } }
  | { ClaimBurn: { claim: ConfidentialClaim } }
  | { ClaimValidatorFees: { epoch: number; validator_public_key: string } }
  | "DropAllProofsInWorkspace"
  | { CreateFreeTestCoins: { revealed_amount: Amount; output: ConfidentialOutput | null } };
