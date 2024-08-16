// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Automatically generated by:
//   cargo run -p risc0-circuit-bigint -F make_control_ids --bin make_control_ids > risc0/circuit/bigint/src/control_id.rs

use risc0_zkp::{core::digest::Digest, digest};

pub const RSA_256_X1_CONTROL_ID: Digest =
    digest!("b64d9520da37fe3fb06b5254065d140238bd165a7b41bf51ef3fc25433e98503");
pub const RSA_256_X1_CONTROL_ROOT: Digest =
    digest!("39b42c22d723a56dda3ab641aecbf92442a7ff4415fe800b814a3d086a159e2b");

pub const RSA_256_X2_CONTROL_ID: Digest =
    digest!("c9b17f4d82070833e6d6023304b4111ef1e48767f204d66e705eff2074c28655");
pub const RSA_256_X2_CONTROL_ROOT: Digest =
    digest!("32d34e64a3d70b09ed889d63d9df1c4ffa468b1a150fc31246bc994c73e43b14");

pub const RSA_3072_X15_CONTROL_ID: Digest =
    digest!("5b8c6338762a361b58cdf418544cb321a5b11961e6907e05759a30130f892801");
pub const RSA_3072_X15_CONTROL_ROOT: Digest =
    digest!("a5568767aa62fd111843d721c213910cdad0f03386db8269301f0009efd86041");

pub const NONDET_INV_TEST_8_CONTROL_ID: Digest =
    digest!("06b2d40f2642aa45bf36d376f8620e6cdc4e6b574924f31a1f229e6b9345422f");
pub const NONDET_INV_TEST_8_CONTROL_ROOT: Digest =
    digest!("9161051265033a043167c53126a2c052e9022d69ba54256bdd75260bfca99e37");

pub const CONST_ADD_TEST_8_CONTROL_ID: Digest =
    digest!("6a8afb2846b4af55dd68cb5b58a059493fbe0b365a21322fcd39b5308ee40917");
pub const CONST_ADD_TEST_8_CONTROL_ROOT: Digest =
    digest!("1fca6f6d47c86268fe193a123eae674f3b56e55fe454d7743583f1492e8d3865");

pub const CONST_ADD_ALT_TEST_16_CONTROL_ID: Digest =
    digest!("97a96a16ce3b5d0f84cbb4488f18a359d626ac0b74124413f679634df19b0567");
pub const CONST_ADD_ALT_TEST_16_CONTROL_ROOT: Digest =
    digest!("4bd8e046e365a53ba83f1e5860c9634379fd941863542135837faa302be8e706");

pub const CONST_MUL_TEST_8_CONTROL_ID: Digest =
    digest!("82aee8246da8193e045a901c078ee9752d742d35613f8f3e05ac1c18c6454e70");
pub const CONST_MUL_TEST_8_CONTROL_ROOT: Digest =
    digest!("28ac5116566e2b1f312d204cfd6d972e2d70a51a75d9f93b6dda101e2b513649");

pub const ADD_TEST_8_CONTROL_ID: Digest =
    digest!("7a5e693d29efca181c14524283d3c64a2e39d15b1e003744c985d46f62ccb554");
pub const ADD_TEST_8_CONTROL_ROOT: Digest =
    digest!("b6666525d30389428d32e651c6742647a1d6de69b6639e30d7c30c616c879855");

pub const ADD_TEST_16_CONTROL_ID: Digest =
    digest!("0949613353166c5c842b5a0b3fbc355f16a39b03d534652a2fbf870fa134c02a");
pub const ADD_TEST_16_CONTROL_ROOT: Digest =
    digest!("87b5c15b6f4e166af7ef4f30a9bb9319cfe07e11258a0f6c5f7e6d5f3f872c0e");

pub const ADD_TEST_128_CONTROL_ID: Digest =
    digest!("79b36c33fbb721755a0a3f1336e909712e2c6c4c4e2fab33ecfaf32d8ed32754");
pub const ADD_TEST_128_CONTROL_ROOT: Digest =
    digest!("0047b84a1718a954d33fda38e74627490ac1c60ab71f125c559d76754fe72e22");

pub const CONST_ONE_TEST_8_CONTROL_ID: Digest =
    digest!("1f8fc4087a0e1d4c2348c16cc0ce5e721776ad1803b3eb1754851a2e4e81f002");
pub const CONST_ONE_TEST_8_CONTROL_ROOT: Digest =
    digest!("a067705e9fc51c2b5128843c7f2c9a10691dbe271d40fd3f3d56cf00930e7775");

pub const CONST_TWOBYTE_TEST_16_CONTROL_ID: Digest =
    digest!("ee9b1730160a75140d0d470ab20f353eff375b346647012ec8733d41e821a733");
pub const CONST_TWOBYTE_TEST_16_CONTROL_ROOT: Digest =
    digest!("9a258702faa4b75a2e7fb96a254a0e14b65619562d0b5a0aa4c80917b6cdb62b");

pub const SUB_TEST_8_CONTROL_ID: Digest =
    digest!("91afae323ffc1f227c0d731ca06ec75890875a0b8fd1211cf4e4486df2b9813d");
pub const SUB_TEST_8_CONTROL_ROOT: Digest =
    digest!("ba12ed3af526b04e5248501219fa09271cd6a705ab0ce132c329ac7777f84b02");

pub const SUB_TEST_128_CONTROL_ID: Digest =
    digest!("60e1270202c0d9453c421a2cd02c773ed8efbe2fe38c4b479729546ed8e56c51");
pub const SUB_TEST_128_CONTROL_ROOT: Digest =
    digest!("57d56a4322dadb0b86a13670e4cc114550d0480c0f143423dd1fcc1632c3bd0d");

pub const MUL_TEST_8_CONTROL_ID: Digest =
    digest!("b84be75d8dc2e6270149bd07cc3fad38d8f84226400cc85dc7b3ab72b1aa6207");
pub const MUL_TEST_8_CONTROL_ROOT: Digest =
    digest!("08be6355563b4b47ade35c1391e89a450e5bf3264f41f03abf532e746864274b");

pub const MUL_TEST_128_CONTROL_ID: Digest =
    digest!("80b4823af8eaaa1c1a844f551a4ff055e49bfc589ed8d701069b64637694f770");
pub const MUL_TEST_128_CONTROL_ROOT: Digest =
    digest!("a3c5f2432fbcf66e70d9c1770ae8bd124c435b471c124e3f249b054ed14a0249");

pub const REDUCE_TEST_8_CONTROL_ID: Digest =
    digest!("9a32102c3c7772299d47435a0c079012c41573215b678454210a045615cd0f5e");
pub const REDUCE_TEST_8_CONTROL_ROOT: Digest =
    digest!("1a23e116f3e13f02e0c0325cb37ea75979ee7947bfd8940ca798350ad5ef273c");

pub const REDUCE_TEST_128_CONTROL_ID: Digest =
    digest!("d9803b4d5d64c238de13315770ed3b372332622fedd05a0950fa5b376b368c19");
pub const REDUCE_TEST_128_CONTROL_ROOT: Digest =
    digest!("9ea6c9016ca0036732577e6e4feabf43e418b800a9bb8d6d0ed5e04c47ef2757");
