// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Amount } from "..\\Amount";
import type { ComponentAddressOrName } from "./ComponentAddressOrName";
import type { Epoch } from "..\\Epoch";

export interface ClaimValidatorFeesRequest { account: ComponentAddressOrName | null, max_fee: Amount | null, validator_public_key: string, claim_fees_public_key: string, epoch: Epoch, dry_run: boolean, }