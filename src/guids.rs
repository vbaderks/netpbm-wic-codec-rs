// Copyright (c) Victor Derks.
// SPDX-License-Identifier: MIT

use windows::core::GUID;

// {4DB4F1DE-8B5D-4E8A-9B83-B5164A4F0206}
pub const NETPBM_DECODER_ID: GUID = GUID {
    data1: 0x4db4f1de,
    data2: 0x8b5d,
    data3: 0x4e8a,
    data4: [0x9b, 0x83, 0xb5, 0x16, 0x4a, 0x4f, 0x2, 0x6],
};

// {70ab66f5-cd48-43a1-aa29-10131b7f4ff1}
pub const CONTAINER_FORMAT_NETPBM_ID: GUID = GUID {
    data1: 0x70ab66f5,
    data2: 0xcd48,
    data3: 0x43a1,
    data4: [0xaa, 0x29, 0x10, 0x13, 0x1b, 0x7f, 0x4f, 0xf1],
};

// {8adbe21c-a720-424e-b238-45ad1052b98c}
pub const VENDOR_VICTOR_ID: GUID = GUID {
    data1: 0x8adbe21c,
    data2: 0xa720,
    data3: 0x424e,
    data4: [0xb2, 0x38, 0x45, 0xad, 0x10, 0x52, 0xb9, 0x8c],
};

// {72A984E2-345A-4227-AA13-B4F1278EB5CE}
pub const PROPERTY_STORE_CLASS_ID: GUID = GUID {
    data1: 0x72a984e2,
    data2: 0x345a,
    data3: 0x4227,
    data4: [0xaa, 0x13, 0xb4, 0xf1, 0x27, 0x8e, 0xb5, 0xce],
};

