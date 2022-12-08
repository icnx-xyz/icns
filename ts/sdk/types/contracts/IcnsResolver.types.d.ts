/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
export interface InstantiateMsg {
    name_address: string;
}
export type ExecuteMsg = {
    set_record: {
        adr36_info: Adr36Info;
        bech32_prefix: string;
        name: string;
    };
} | {
    set_primary: {
        bech32_address: string;
        name: string;
    };
} | {
    remove_record: {
        bech32_address: string;
        name: string;
    };
};
export type Name = "cosmos" | "ethereum";
export type Binary = string;
export type Uint128 = string;
export interface Adr36Info {
    address_hash: Name;
    pub_key: Binary;
    signature: Binary;
    signature_salt: Uint128;
    signer_bech32_address: string;
}
export type QueryMsg = {
    config: {};
} | {
    addresses: {
        name: string;
    };
} | {
    address: {
        bech32_prefix: string;
        name: string;
    };
} | {
    names: {
        address: string;
    };
} | {
    icns_names: {
        address: string;
    };
} | {
    primary_name: {
        address: string;
    };
} | {
    admin: {};
} | {
    address_by_icns: {
        icns: string;
    };
};
export interface MigrateMsg {
}
export interface AddressResponse {
    address: string;
}
export interface AddressByIcnsResponse {
    bech32_address: string;
}
export interface AddressesResponse {
    addresses: [string, string][];
}
export interface AdminResponse {
    admins: string[];
}
export type Addr = string;
export interface Config {
    name_address: Addr;
}
export interface IcnsNamesResponse {
    names: string[];
    primary_name: string;
}
export interface NamesResponse {
    names: string[];
    primary_name: string;
}
export interface PrimaryNameResponse {
    name: string;
}
//# sourceMappingURL=IcnsResolver.types.d.ts.map