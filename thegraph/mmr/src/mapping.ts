import { BigInt, Bytes, json } from "@graphprotocol/graph-ts"
import { mmr, Test } from "../generated/mmr/mmr"
import { NodeEntity } from "../generated/schema"
import { ethereum } from '@graphprotocol/graph-ts'
import { blake2b } from './blake2b'

// ropsten
//const beginBlock: u64 = 11788600;
// mainnet
const beginBlock: u64 = 14288600;

export function handleTest(event: Test): void {
}

function hash(block: ethereum.Block): Bytes {
    return block.hash;
}

function toU64(dec: BigInt): u64 {
    let decimalString = dec.toString();
    return json.toU64(decimalString);
}

export function handleBlock(block: ethereum.Block): void {
    let blocknumber = toU64(block.number);
    if (blocknumber < beginBlock) {
        return;
    }
    if (blocknumber == beginBlock) {
        init();
    }
    let block_position = leaf_index_to_pos(blocknumber);
    let record = new NodeEntity(block_position.toString());

    record.position = block_position.toString();
    record.hash = hash(block);
    record.save();

    checkPeaks(block_position);
}

function checkPeaks(block_position: u64): void {
    let height = 0;
    let pos = block_position;

    while (pos_height_in_tree(pos + 1) > height) {
        pos += 1;
        let left_pos = pos - parent_offset(height);
        let  right_pos = left_pos + sibling_offset(height);
        let left_elem = NodeEntity.load(left_pos.toString());
        let right_elem = NodeEntity.load(right_pos.toString());
        let record = new NodeEntity(pos.toString());

        record.position = pos.toString();
        record.hash = merge(left_elem.hash, right_elem.hash);
        record.save();
        height += 1;
    }
}

function saveRecord(position: u64, hash: Bytes): void {
    let record = new NodeEntity(position.toString());
    record.position = position.toString();
    record.hash = hash;
    record.save();
}

// we can select a checkpoint from subgraph/mmr/checkpoint.json to initialize this peaks.
// ropsten
/*
function init(): void {
    saveRecord(16777214, Bytes.fromHexString("0x36f3d834cbe12a5a20b063c432b88f5506bdce03b93fa3aa035a5d82fd50177c") as Bytes);
    saveRecord(20971517, Bytes.fromHexString("0xb10a06336827182396eabf37e835e57252cee94fd4b493787a76a9869026a65e") as Bytes);
    saveRecord(23068668, Bytes.fromHexString("0x2af9dc26e368b42906981abdb6fb59a68eb77817d02f964af755cd9c52a148c0") as Bytes);
    saveRecord(23330811, Bytes.fromHexString("0x589234c63f4dc85db523bc98910f794c9441593857cfd7249f87b15111d5444a") as Bytes);
    saveRecord(23461882, Bytes.fromHexString("0x76a3a4b315bbce5e677ca8a67d58454cc593c46b6758099268b7790d8e0ea1ba") as Bytes);
    saveRecord(23527417, Bytes.fromHexString("0xf255e727496437a0fadcf150f1dbdba2c8b384c4e4029bd3d7293d3dc5c3f6a0") as Bytes);
    saveRecord(23560184, Bytes.fromHexString("0x0b621c8c07027edadb30e93b019a111e24849a962c9d3d0ac55a18a9c148443a") as Bytes);
    saveRecord(23576567, Bytes.fromHexString("0xa71a0c812345ab35242a9839304f6de811ed010a35622154734b75003f5f469c") as Bytes);
    saveRecord(23577078, Bytes.fromHexString("0x4ce7ab13d9947207378203750e7fa7857c6edbbb85454077333d761de7aa6aa9") as Bytes);
    saveRecord(23577141, Bytes.fromHexString("0x738dab777b875aeab4656388cb4b0082b22bdd86ffcd2a99da2adc9cb0b7945e") as Bytes);
    saveRecord(23577172, Bytes.fromHexString("0x485be458c572b9393a1f68ce36f9bef792b277dfa22a868cdec8de6d2179989a") as Bytes);
    saveRecord(23577187, Bytes.fromHexString("0x87083f853f4c4a7c3615c166d630e1c6d46e85aebc0852744a05cd04cb26b900") as Bytes);
}
*/
// mainnet
function init(): void {
    saveRecord(16777214, Bytes.fromHexString("0x8ab738686d2bd7f0f0340365ee5c23bdef99c571d1662607108903d646d0b332") as Bytes);
    saveRecord(25165821, Bytes.fromHexString("0xc0ab7f73f1a062b9255c2daf7d7ab8ab73f60e49fc1a76a5d733aa11c8121029") as Bytes);
    saveRecord(27262972, Bytes.fromHexString("0x32e963402351f21b6037f0870c3968eaed971c6498265a6de918ac7b0063ec06") as Bytes);
    saveRecord(28311547, Bytes.fromHexString("0x8635a38d3810479760330ba6383a94d613716ea0748144bbf57082416961e768") as Bytes);
    saveRecord(28573690, Bytes.fromHexString("0x969109a4eb91d9a15a53c00e2f59084bc86f5831483efddf711e31a8c7f9cac8") as Bytes);
    saveRecord(28575737, Bytes.fromHexString("0xa9fa2096ccc970409f1405ffdda050607b3a440cb5e0bbd751ba63e171aba252") as Bytes);
    saveRecord(28576760, Bytes.fromHexString("0xd5e9c63688124bad6eca40b522f6bc7051e37789f088437fc908f5b197462b0a") as Bytes);
    saveRecord(28577015, Bytes.fromHexString("0x0a6597cc714e1684d31d4a0086c91d572e464fdb4104a4eae2fc4b51b59e8573") as Bytes);
    saveRecord(28577142, Bytes.fromHexString("0xb1c9470a6869f2462ed2c703b8d9b617e9d9fe7a97af3929e751a2c77850df14") as Bytes);
    saveRecord(28577173, Bytes.fromHexString("0x4d99a90b98432aeb87ff169cd2160654cb21172918d99168605dfd35560bcc44") as Bytes);
    saveRecord(28577188, Bytes.fromHexString("0xd4a62a7fe579348076e2395ff7c7b7c257fc2a69bdc2c818eed86396e4801547") as Bytes);
}

/* ---------------------------------------helper fns-------------------------------------- */
function merge(left: Bytes, right: Bytes): Bytes {
    //let res = concatTypedArrays(left, right);
    let res = new Uint8Array(left.length + right.length);
    for (let i = 0; i < left.length; i++) {
        res[i] = left[i];
        res[i + left.length] = right[i];
    }
    return blake2b(res) as Bytes;
}

function leaf_index_to_pos(index: u64): u64 {
  // mmr_size - H - 1, H is the height(intervals) of last peak
  return leaf_index_to_mmr_size(index) - trailing_zeros(index + 1) - 1;
}

function leaf_index_to_mmr_size(index: u64): u64 {
  // leaf index start with 0
  let leaves_count = index + 1;

  // the peak count(k) is actually the count of 1 in leaves count's binary representation
  let peak_count = count_ones(leaves_count);

  return 2 * leaves_count - peak_count;
}

function count_ones(dec: u64): u64 {
    let ones = 0;
    for (let i = 0; i < 64; i++) {
        if ((dec & (1 << i)) > 0) {
            ones += 1;
        }
   }
    return ones;
}

function trailing_zeros(dec: u64): u64 {
    let zeros = 0;
    for (let i = 0; i < 64; i++) {
        if ((dec & (1 << i)) == 0) {
            zeros += 1;
        } else {
            break;
        }
    }
    return zeros;
}

function leading_zeros(dec: u64): i32 {
    let zeros = 0;

    for (let i = 63; i >= 0; i--) {
        if ((dec & (1 << i)) == 0) {
            zeros += 1;
        } else {
            break;
        }
    }

    return zeros;
}

function all_ones(dec: u64): boolean {
    let bit_length = 64 - leading_zeros(dec);
    return ((1 << bit_length) - 1) == dec;
}

function jump_left(pos: u64): u64 {
  let bit_length = 64 - leading_zeros(pos);
  let most_significant_bits = 1 << (bit_length - 1);

  return pos - (most_significant_bits - 1);
}

function pos_height_in_tree(pos: u64): i32 {
  pos += 1;

  while (!all_ones(pos)) {
    pos = jump_left(pos);
  }

  return 64 - leading_zeros(pos) - 1;
}

function parent_offset(height: u64): u64 {
  return 2 << height;
}

function sibling_offset(height: u64): u64 {
  return (2 << height) - 1;
}

