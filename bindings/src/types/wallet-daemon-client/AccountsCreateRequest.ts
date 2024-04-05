// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Amount } from "..\\Amount";
import type { ComponentAccessRules } from "..\\ComponentAccessRules";

export interface AccountsCreateRequest { account_name: string | null, custom_access_rules: ComponentAccessRules | null, max_fee: Amount | null, is_default: boolean, key_id: number | null, }