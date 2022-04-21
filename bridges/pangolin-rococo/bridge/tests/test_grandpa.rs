use client_pangolin_parachain::types::runtime_types::sp_runtime::generic::header::Header as FinalityTarget;
use subquery_s2s::types::NeedRelayBlock;

#[test]
fn test_finality_target() {
    let json = r#"
        {
          "id": "1891870-5",
          "blockNumber": 1891870,
          "blockHash": "0xd6e8dd92c6e9c15f0155488f5397c67d3a8716a1087024954eb31cd99a9b653b",
          "type": "on-demand",
          "origin": "bridge-pangoro",
          "laneId": "roli",
          "messageNonce": 223,
          "parentHash": "0x1becb476d9df47fff31c6cc2491c5cafcf53186fba5b99523fd08c9976a09ef0",
          "stateRoot": "0xd50d0c72edd0fa59328dad221e3d57e44abf3dec05d28e4779d1a6da3ec3089e",
          "extrinsicsRoot": "0xba501248b5b7d145b314a3e32e075f7c403fed505869b7d73e4ec75a5437250b",
          "digest": "0x140642414245340203000000cf4363100000000004424545468404dccdfb8e98567a2ca9dcd87af3d5ea1fe4ceb4c2b561b302d85b3cdc0817336b0466726f6e090101f1a89fa73dcaf1741c593df45ca911ef95bd0fb162b51cd155ad83b5ff49b6b604c8c1b34236ed0d5dad607cb57d7d0cdc73c867238a816aa3ed3ed0786da5349900904d4d5252f265cc1b40bb4e7abc4985360aa4657c5b28413ce9462fd1a9d7e360a5ab7845054241424501017e02ac7f26dd10944e183b955cf4927cc2c694545d8a8b8c6cee56017df1176ae04d11a0a405dec2ea1560453a062801df68a3a750da3022dcead068edd40b8a",
          "timestamp": "2022-04-11T02:47:54"
        }
    "#;
    let next_block: NeedRelayBlock = serde_json::from_str(json).unwrap();

    let raw_digest = next_block.digest;
    let digest = codec::Decode::decode(&mut raw_digest.as_slice()).unwrap();
    let _finality_target: FinalityTarget<u32, u32> = FinalityTarget {
        parent_hash: sp_core::H256(next_block.parent_hash),
        number: next_block.block_number,
        state_root: sp_core::H256(next_block.state_root),
        extrinsics_root: sp_core::H256(next_block.extrinsics_root),
        digest,
        __subxt_unused_type_params: Default::default(),
    };
}
