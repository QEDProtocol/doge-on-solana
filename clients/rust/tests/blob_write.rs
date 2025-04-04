use qed_solana_doge_ibc_v3_program_client::{
    doge_chain_state::QEDDogeChainState,
    helpers::{blob_buffer::BlobBufferHelper, instruction_data::gen_instruction_data_create},
    instructions::CreateBuilder,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program_test::tokio;
use solana_sdk::hash::Hash;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use zerocopy::FromBytes;

use std::time::Instant;
const TIME_LONG: &str = "\x1b[48;5;124m";
const TIME_MEDIUM: &str = "\x1b[48;5;24m";
const TIME_FAST: &str = "\x1B[38;5;230m\x1b[48;5;34m";
fn get_time_color(elapsed_ms: u64) -> &'static str {
    if elapsed_ms > 2000 {
        TIME_LONG
    } else if elapsed_ms > 500 {
        TIME_MEDIUM
    } else {
        TIME_FAST
    }
}
pub struct DebugTimer {
    pub start_time: Instant,
    pub name: String,
}
impl DebugTimer {
    pub fn new(name: &str) -> Self {
        let n = name.to_string();
        Self {
            start_time: Instant::now(),
            name: n,
        }
    }
    pub fn lap(&mut self, event_name: &str) -> u64 {
        let elapsed = self.start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as u64;
        println!(
            "\x1b[96m{}\x1b[0m - \x1b[94m{}\x1b[0m: {} {}ms \x1b[0m",
            self.name,
            event_name,
            get_time_color(elapsed_ms),
            elapsed_ms
        );
        self.start_time = Instant::now();
        elapsed_ms
    }
    pub fn event(&mut self, event_name: String) -> u64 {
        let elapsed = self.start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as u64;
        println!(
            "\x1b[96m{}\x1b[0m - \x1b[94m{}\x1b[0m: {} {}ms \x1b[0m",
            self.name,
            event_name,
            get_time_color(elapsed_ms),
            elapsed_ms
        );
        self.start_time = Instant::now();
        elapsed_ms
    }
    pub fn batch_average(&mut self, event_name: &str, batch_item_type: &str, batch_size: usize) -> (u64, u64) {
        let elapsed = self.start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as u64;
        println!(
            "\x1b[96m{}\x1b[0m - \x1b[94m{} ({}x - {})\x1b[0m: {} {}ms \x1b[0m",
            self.name,
            event_name,
            batch_size,
            batch_item_type,
            get_time_color(elapsed_ms),
            elapsed_ms
        );
        let per_time = ((elapsed_ms as f64) / (batch_size as f64)).floor() as u64;
        println!(
            "\x1b[96m{}\x1b[0m - \x1b[94m({}) Per {}\x1b[0m: {} {}ms \x1b[0m",
            self.name,
            event_name,
            batch_item_type,
            get_time_color(per_time),
            per_time
        );
        self.start_time = Instant::now();
        (elapsed_ms, per_time)
    }
}

const TEST_SER_QED_STATE: [u8; 6414] = hex_literal::hex!("8f9b55001f00fb539151219829669b56e0aecf8df07c311953d3a0ac7ac7d6a7a39a4676b256cf4166bc9ebc2ec7a3b1d2bf2f1ea13e482cb4566f988a502b6a0ed50a6b917064779b43665222bf9ecadb1f185da967352577048753aad84801c58c70a30600c7cac567ad8b001aac7b57d9acec689020b96c5c25684cf4654658111168ec0f15e97ec2d8176be2552a30f896ba490d2c01c49ded695d37d01155494974d76a0626655eb3eea78b5da5d83c6092f9a49453a5b305c9a501bcd3b6694ebfa9aab7b3135c870853cfd2cac56759617b193ad5cc64a78c033d12a79a3e4dad2d349de4357caa3b0ac7b15c9b0000d235d83dcbc9d1e292072666cf2b47d0110676cb545493038f64022df69c955e75528646eba24fcca2a4d2a8f5b52b6a4336d9a31dab136cc9e63d73e5cf707464a918f1cac567d00a6f19162e41cbcde130cee150508fb008f7225ddd6f9e77091511ffc285f29a60f3915610046d41aa336e7cce65e72c701079cb366dab1abbb7950948c1a41676b7e02588137435dacf5eeb3fa57e30fec36651265a6d390ee5bc71ef33885bc5325f57cbc567787d6919e656aec4247d13d42cdb72300a19a3670f703cba72f3ca246db545142df597d60339a8553bd99467e4d77c7f7aab79c829fc1e47c5f145509e5675715d6eb46059e8198746649ae9212204f4e42e22da526b6a3b0fb23613f5c4088b0ed074755ecbc567ec47721919fbbb120929835e9a6e73ee7e885ab70c95f6ee136b13ef33b669249b757d4e92411f875d69f10b56ed8af0e6eab5f66dd0b2961cc05098af2f30e9f7bcd4ac62004f909ce96ab51d8e1ebf3549b822996952596b99aef651d1ffbfc6b5563490cbc56754da661956dbb31de5c3c323fc1d6b15a93a253ec1c5f2887f8d8db3e781a452a0c1691c3430f42418d2087b9bb3d178ffb0f692a665a4a694ec36dc0efabf7d88fa29c8d9e88169140b0b9195c037fc0e83159e887c7f27be430040b8450ff104a9402fb4cbc5677d236519a6fbbf0a985ec29871c510a33fb83f9a6c92d31fe8d6a49448300d4bc43cc42d2f071b0ad6aa66503499fcde270e0a41c63110b2210f83515efa6229eb6117cefebb0b9c9d30c29f14b05ec5f670cbedf1c03de71ebcbc709ada0f2ff1ad787ed7cbc567e91460193f848bede5f02dd5d6a0a7345f2f988ee17e947784dd5c6e7e78f09ee4b6d0388d1744ccb7615bf9eaa1f2bf8e328d77c45c92d7b890bf1e090b0558628d663c9d6a446d5860f8f229062bd4ec3f74d872c8df0087bf91dab0261a4e92d8df6027ccc56710475b195ad150002a66d07ed8a85c40c48908244fb1dbf54fb6302046f953bca83dce52c923958c9162b74c8c09f15e12e30e44f163f15ad8a2da3d0a0f500f972d6025eeaf4b74ee519b563e251658286b13cd6b55bf644f483223c8dab35b76d1ee29bbccc567f6515e192c929a09cebdcf2d4fd51f2b34d81c7aa5ff59221aa7a05660cc8c7746b349f089f7dab4610f1631282173767a45d6ca3806c6796ff4d91e25197a6d9f324e86afe1034698d15040a884b2e822edd69209925e68168ac9fa4e0f44ccc68047c4f6ccc567b89c6f19f7fde52189ae9b7255fe0bf709e6b8a5ce2cf2964ca99a82ec05ead5885ff3e4b543820a8b183206d3acf9511aa4ac4841391ada092aba2bfb2878d67ed7130ae353eef1da2789d930804aeeb47dcdfcb4f81acea52bbf0e548eadae6435e1f2e8cdc567b89c6f190b9bf744348747c96316030199f7a37290486f907c4150c764fe19d306853da6db0141fe962c67761dd2b621de02dc9b398700dfee77caae5365baf59b74915c377799b772552ddc0b883113a915ff251625afe780de0720b7e2071c570536f3c8cec5678998001ab9415dbd2de3f0474cba9f41fe83be2b39972694fce0768cf22732b8b51d43344f6f0a988b573ebf1eb38731cf09ad165b44696737074d325c58f6433b1e6100273655f8fce620460e4c6cccffa37b1f87e569494e145a996b8eeb6a0445be7c1acfc56761cb001ac9b128205aba6eaea3980306554945414484fd00d8449cc784569e4e85a6350e312285d78952ace415f87ecad2fe4a6904e7925bb87aac708766de148c5db2bfbcb514abd707194965637f5c394aaf6f2ba6b67be523d335561227f9df1b169739cfc56728d2001aa0d17b8606037276461b601e3785270685b7705675ed8b7f2f8494d4f637b498bcffb4a17062355ec0dff9a3745189118407fef87257cd9304270a4d4d6e4aa2546ba759e15a394285e08215299a1aa809bc5ae8e1003624d67d0e67bc0b9b2954cfc567a6c7001a12a8e74a7f3b8967e7f5006a73fd86d60479fb89c21f79eb0d78192bbca36b1856219c21044467d5ac2d68e465e4a0052e436a041d8382f0e9f524eac731db822e24d1adaaa51057625e56db7ac8b80e72f49942264a4ef2cae0899083f89b6760cfc56756ba001a10ebe19816e3109e10fe421eddf479136f673178e2e5b1700622d48bc2f6ddd3280777148b5b0fcc2a1c3a1f01f23f4229cf141badb56e77df402d937833f61346cd491b6986e13afdb40d1a836efd6d7c2017082462142e1eac220985743e5caecfc567b3a7001a6a1bad5d27c28662b79c6875454dee978b277d7ca94e842c37832a01414af18a367ce7d7f36dc0ffbd84debfdeea6a2f343a4020e7091a2545ff8c07985d7c6a1efeee1efb61b956a5f6d2688c3ca05763a8a44f1cb7887c36f1e7a9a7e4723dd4cfc5674aad001a07712bfc00a5531af8eb48c09010ec68d3968374c86f0c55c03d17a499786295d934432fba080581262deef8e3570c9d8f39422cf241dda6583da84f3b2aa4d26901feea3346feb083bcf0b12fb0b1275af45fcbd0a41d41adcbb9e736141a12e6cfc56783a7001a69f589457399dad4893bf724c0225c99b2855af0fc56b5ef8137bc6ad0e33d4342a3625c13a630c3e91bc3bcded345d5927d84039960000b5be65ceb57e5bc80ca94a9c5c59d84306e66a52737c3d44a747d2d59d5111c15c47a5f2e0b451c7d55d0c5678d99001ad04a6b14ebee37b7e050d435d989333265bc2c17980acf3294788c16c3ee71e10a45aa84d09a350ae989d7ba12a2deef20344013d4b6bc00d0c8cf6d9d5e81e058754175bd9192c561c86a566e9c737e5e5638d451e56bbec3b0771216edad9d72d0c567e7a8001ad2bac7ccccd0c6ee400a8b65286ca004d89b5d70cc29424472a6eb1749d31a31a9dcdf95ca812fe13999470440381502b97131b7b273b4a99613b7094e582e16f260d112872c133b26aab1238ad14f8bcf00d2b4175e786491ed778023f177a180d0c56775a0001a0d09f3ed3abedc9cdbac525d0450e657ce723cbcde4466735f8992dcc05739059b77210b16c7d28964452c4c03d03ec1da0326cbc45ef1781be4b46b628d852f4f546295bc94322f3eab50a65fb8b4d24c9720dea2d4ba0816e3fa46e3d29761a2d0c5671593001a9d406116ca171fbc36159bd235bfe6904e8e599f69a13bc208de7d88808f72eed7807aca3babc04aa2ee7ad39f48f7d8679de1bdc5a2a4035253400f4597b8e86afdc0597f02efb831cb2cb578d5926377fcaa8727505f79fdb2b7c25d321cc010d1c567ba8b001afcb9e1cf5b61a6b8ee799f2768063bcff08428b05e1c551ad2e961d89e9f58d230fc3e98416bbf9b29912e1c0b12d54c7364c90e07642c7ea643defde4847f42a91770419fdfbe4d15f1a28934a84c38a1fd85e86eee688bbda7e1eda4b1fda658d1c567b399001a94a9e2200b7a86f3a6c069fc3fb48aa5889ea4301c6a990d36b64cc240e71370d0eaf1060c4d38f380552b91bb94c8166c5ce11708595b7376396d87ca111cbc6b7723ad69bd7e2de706697e15ce2fca9019b04f196620e42c3b372c0faf00db91d1c567429c001aecceb16fa0028f4235433ec2aef179117c8fc809eb197f7b2e90e723b8a4fac52bcbc87799036db31bf08c6b8759014c297012b91869bc26f80ec1feb64e91732a5c41550db282cb946aab4fe8dd922faa9f0f281d024b939365d94c0eb49b6ab7d1c567429c001a37f18f2105335ac3ad1916ecb06f0d994f5ac4635b470e197978d1e4a215efc03ca52d374ad546f2bed3fb066e7859b01ea732cd6374b5cb4a319c841d4ddfc8aa05afa5e8607ff0c068423457b1fdc1bbbd8043b9599c59a82e796ab7501ebafad1c5670c97001ade2d7c28188bb22b1c607a56963801e1cea475b3c6c31050dd14e363579188c6ffcbc02f4475c7a54477feceb87f8ac0b8623fd113e40ba706125fcd3bf3a90a448f2e7f99cb93f3a84b4c32756ee39c215f7a92258a6c2742760707a8cbba5ff4d2c5670c97001a2f01f0869b0610124a389adec426618b379b1584c841e4bd9b20bf08d205db610fda773f4c3bd5a6d6b65196205a1ee2f7b1370f0d411819f95d83c338872c49792496e602c5eb87a20f34b14d0293d44db06e07cc00e83efe95d5bed676075704d3c567f2d0001a9e4ce542e5cdafcbc2cd69b4a1ce29272b4d83ab9fec80215683f0eebb880a3b7719c88d931b972b9e1ef5a584bc4e770eb07f80662319bc01cdaf7f76a6560abdf612cc6d207d443f2d1db6fb91361ace12c68f8b43d752d1242148210af2804bd3c56788bf001a909b5500000000000fda773f4c3bd5a6d6b65196205a1ee2f7b1370f0d411819f95d83c338872c497719c88d931b972b9e1ef5a584bc4e770eb07f80662319bc01cdaf7f76a6560a0000000000000000000000000000000000000000000000000000000000000000bb01b51d5e4f01b48607156b57dcae2e24a8fa92f0079990290e7a8820e7afcd68bfaada00ec140023839bc1e6f93814fe672bc122741edecde06cc8aa90aa3cf5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4be4d5f6c63c6759ffa4dcaf28046847b8339879ac8ab205236ae77cce5a259c1a8c847162d9b760302417ba53c3e6381a495c5a019bdaaf0fbc54d2304ba0cb21db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d718302e11a6dff8449d48a331f3d5e741ab2064b0270df136328812d563eebb6f8f5e9c66b165b0ca96cd6cd58dfbb984e8017ea4c20d50e48e129719b3963c2acc78009fdf07fc56a11f122370658a353aaa542ed63e44c4bc15ff4cd105ab33c1ee7470bd345719bafc7501cc9cad078c1b4693a9496d307f9e6f0ee45a55925536d98837f2dd165a55d5eeae91485954472d56f246df256bf3cae19352a123c536d98837f2dd165a55d5eeae91485954472d56f246df256bf3cae19352a123cba1b65cc643bbc5fe317c22ed45da3755d1c5fbe726a5e0575aa715c227952d69efde052aa15429fae05bad4d0b1d7c64da64d03d7a1854a588c2cb8430c0d309efde052aa15429fae05bad4d0b1d7c64da64d03d7a1854a588c2cb8430c0d301fe9ca863644b39b63d64d3d4ebd7b585ba2b0dbc4c279172bef1e271bf9f92cd88ddfeed400a8755596b21942c1497e114c302e6118290f91e6772976041fa1d88ddfeed400a8755596b21942c1497e114c302e6118290f91e6772976041fa135f84d636f46d4d65a178f4abcfdf503777fe5e23810bce9c6535fd1b98a45733e4c6bc0d810831ce3daae97e5efadd9825cc0a84f33e368b6733c0fa25c14f487eb0ddba57e35f6d286673802a4af5975e22506c7cf4c64bb6be5ee11527f2c26846476fd5fc54a5d43385167c95144f2643f533cc85bb9d16b782f8d7db1935ba90e0cd3071e32b6ee074d23c1139ba79a0386645fe29347b27aed309c8f5926846476fd5fc54a5d43385167c95144f2643f533cc85bb9d16b782f8d7db193506d86582d252405b840018792cad2bf1259f1ef5aa5f887e13cb2f0094f51e1d947c38d343dd8d30886f326a51faf1b70535fc6c23a43b3dcdd98121e1999a8506d86582d252405b840018792cad2bf1259f1ef5aa5f887e13cb2f0094f51e1412a135f131eeeb7fb2c33de92b7e376ba6f7e0555ca7045ec0dd098f38a3f96ffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92bffff0ad7e659772f9534c195c815efc4014ef1e1daed4404c06385d11192e92b6cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220c3665a548084e54e0cad64f0532d297aae36195326c36969363ac6f2b813a6036cf04127db05441cd833107a52be852868890e4317e6a02ab47683aa75964220b7d05f875f140027ef5118a2247bbb84ce8f2f0f1123623085daf7960c329f5f30c51b224f266653333680bcb4ffddbbfa3b8c5080f34fef4545a8e77f0345cfb7d05f875f140027ef5118a2247bbb84ce8f2f0f1123623085daf7960c329f5fc502ca16800cc59379e2702fba6b7fd842e57fcc774daf3321ec3b9e007ae2e9df6af5f5bbdb6be9ef8aa618e4bf8073960867171e29676f8b284dea6a08a85edf6af5f5bbdb6be9ef8aa618e4bf8073960867171e29676f8b284dea6a08a85e3387cc1da1cfc6fce0d6a7e854f7c8d95c795e5182d35f9c6a18f57801c64ea1b58d900f5e182e3c50ef74969ea16c7726c549757cc23523c369587da7293784b58d900f5e182e3c50ef74969ea16c7726c549757cc23523c369587da7293784d49a7502ffcfb0340b1d7885688500ca308161a7f96b62df9d083b71fcc8f2bbb51bc6f0696ed58f3321c4e3e6d092f778c06d8407195de73c46c11b368e124dd49a7502ffcfb0340b1d7885688500ca308161a7f96b62df9d083b71fcc8f2bb8fe6b1689256c0d385f42f5bbe2027a22c1996e110ba97c171d3e5948de92beb90da3c3dc1668be41ab4904cfff3687480ae7a0acc38cf40686ab918b1dbcce68fe6b1689256c0d385f42f5bbe2027a22c1996e110ba97c171d3e5948de92bebdb7029f6d7367e7359c56108575e2b795e6d417da71c2f5c300dffdb3cc5c4288d0d63c39ebade8509e0ae3c9c3876fb5fa112be18f905ecacfecb92057603ab8d0d63c39ebade8509e0ae3c9c3876fb5fa112be18f905ecacfecb92057603ab95eec8b2e541cad4e91de38385f2e046619f54496c2382cb6cacd5b98c26f5a422e42935e72d18ed50bfb95513bc3907e3119332ba99a3af7baa16efcd16ca2795eec8b2e541cad4e91de38385f2e046619f54496c2382cb6cacd5b98c26f5a4f53054b85d707776386b3a138ecc51ebb9d6cfe7ac9c66ba845c1c8b9c43eeb8f893e908917775b62bff23294dbbe3a1cd8e6cc1c35b4801887b646a6f81f17ff893e908917775b62bff23294dbbe3a1cd8e6cc1c35b4801887b646a6f81f17fcddba7b592e3133393c16194fac7431abf2f5485ed711db282183c819e08ebaac3c43b402daa4cd85ef78acdcbe967e62ca4dc7faf571f76165cb56df8b595e3cddba7b592e3133393c16194fac7431abf2f5485ed711db282183c819e08ebaa1c2e813e7ff751b5dde7c8fca9d4aec6bf3870eec113ed048fe624b904c39f408a8d7fe3af8caa085a7639a832001457dfb9128a8061142ad0335629ff23ff9c8a8d7fe3af8caa085a7639a832001457dfb9128a8061142ad0335629ff23ff9cfeb3c337d7a51a6fbf00b9e34c52e1c9195c969bd4e7a0bfd51d5c5bed9c1167336a09b89a52e5625a6891f3d8d7913fbefdd4d3e1edd657d62248f89b22e4affeb3c337d7a51a6fbf00b9e34c52e1c9195c969bd4e7a0bfd51d5c5bed9c11678d4753b292691330659cc93f755db421c6c261d3d75b2f8fe9f81c65029b7567e71f0aa83cc32edfbefa9f4d3e0174ca85182eec9f3a09f6a6c0df6377a510d7e71f0aa83cc32edfbefa9f4d3e0174ca85182eec9f3a09f6a6c0df6377a510d70169384e7cbab976a6b610809039e63bfffb5488da6c4b736154b3991a22060031206fa80a50bb6abe29085058f16212212a60eec8f049fecb92d8c8e0a84bc031206fa80a50bb6abe29085058f16212212a60eec8f049fecb92d8c8e0a84bc052db01b3cb568204cbce1856fa6d9d79b8b0abdafe03364536fa97132ed99ae121352bfecbeddde993839f614c3dac0a3ee37543f9b412b16199dc158e23b54421352bfecbeddde993839f614c3dac0a3ee37543f9b412b16199dc158e23b544d2f0fe96e82ec510da38163b2dae724d5e7559e86e719b0384320cd42cc4b9dd619e312724bb6d7c3153ed9de791d764a366b389af13c58bf8a8d90481a46765619e312724bb6d7c3153ed9de791d764a366b389af13c58bf8a8d90481a46765f018c97c5b64cd77ac5ad35b610043a5786d16e67ca7ddd3eab398ca7fb593f57cdd2986268250628d0c10e385c58c6191e6fbe05191bcc04f133f2cea72c1c47cdd2986268250628d0c10e385c58c6191e6fbe05191bcc04f133f2cea72c1c4829c7d08f90c7b21be60f41a759eacea58ad83704e5fb13858b40a8f43524a84848930bd7ba8cac54661072113fb278869e07bb8587f91392933374d017bcbe1848930bd7ba8cac54661072113fb278869e07bb8587f91392933374d017bcbe1dcf05278ececed808ebc6aef917fc51bf71b700827dfcd399ec9499080c3dbe68869ff2c22b28cc10510d9853292803328be4fb0e80495e8bb8d271f5b8896368869ff2c22b28cc10510d9853292803328be4fb0e80495e8bb8d271f5b8896362efa1fa56b9fb76f38509a3b095dd11cbc8b0aef1d7c210b14fd6945b0f73660b5fe28e79f1b850f8658246ce9b6a1e7b49fc06db7143e8fe0b4f2b0c5523a5cb5fe28e79f1b850f8658246ce9b6a1e7b49fc06db7143e8fe0b4f2b0c5523a5cae8e6a04380c47e4c6d0141aac164c13c57f52a5b1823753a2161e8eab7a585e985e929f70af28d0bdd1a90a808f977f597c7c778c489e98d3bd8910d31ac0f7985e929f70af28d0bdd1a90a808f977f597c7c778c489e98d3bd8910d31ac0f7"
);

struct TestContext {
    client: RpcClient,
    payer: Keypair,
    last_blockhash: Hash,
}
impl TestContext {
    async fn setup_account() -> anyhow::Result<Self> {
        let client = RpcClient::new("http://127.0.0.1:8899".to_string());
        let payer = Keypair::from_base58_string(
            "3aQSf1fzHm1ueckkgYz7JPfj6LGJ6nRaNMZ2gdRJ8TkF5xEBBwjatc2QAtKXhNpVPSS5Mxx4w4yLUzwKKHQkozQ5",
        );
        let sig = client
            .request_airdrop(&payer.pubkey(), 100_100_000_000_000)
            .await?;
        client.poll_for_signature_confirmation(&sig, 10).await?;
        let last_blockhash = client.get_latest_blockhash().await?;

        Ok(Self {
            client,
            payer,
            last_blockhash,
        })
    }
}

fn find_pda_ibc(payer: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"qed_doge_ibc", payer.as_ref()],
        &qed_solana_doge_ibc_v3_program_client::ID,
    )
}
async fn run_simple_init_test() -> Result<(), Box<dyn std::error::Error>> {

    let mut timer = DebugTimer::new("buffer_writer");
    let tctx = TestContext::setup_account().await?;
    println!("setup account");
    
    let pp2 = 
QEDDogeChainState::ref_from_bytes(&TEST_SER_QED_STATE)?;    
let create_ix_blob = gen_instruction_data_create(&pp2);

    println!("create_ix_blob: {}", hex::encode(&create_ix_blob));

    let buffer_helper =
        BlobBufferHelper::new(&tctx.client, &tctx.payer, create_ix_blob.len()).await?;

    println!(
        "new_account_signer: {:?}",
        buffer_helper.new_account_signer.pubkey()
    );
    buffer_helper
        .send_write_transactions_and_confirm(&tctx.client, &tctx.payer, &create_ix_blob, 600)
        .await?;

    let account = tctx
        .client
        .get_account_data(&buffer_helper.new_account_signer.pubkey())
        .await?;
    println!("got account_data: {}", hex::encode(account));

    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    let account = tctx
        .client
        .get_account_data(&buffer_helper.new_account_signer.pubkey())
        .await?;
    println!("got account_data: {}", hex::encode(account));

    let qed_doge_ibc_address = find_pda_ibc(&tctx.payer.pubkey()).0;
    let ix = CreateBuilder::new()
        .qed_doge_ibc(qed_doge_ibc_address)
        .authority(tctx.payer.pubkey())
        .payer(tctx.payer.pubkey())
        .init_data(Vec::new())
        .instruction();

    let dummy_ix = Instruction {
        program_id: ix.program_id,
        accounts: [
            ix.accounts,
            vec![AccountMeta::new(
                buffer_helper.new_account_signer.pubkey(),
                false,
            )],
        ]
        .concat(),
        data: Vec::new(),
    };
    let last_blockhash = tctx.client.get_latest_blockhash().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[dummy_ix],
        Some(&tctx.payer.pubkey()),
        &[&tctx.payer],
        last_blockhash,
    );

    println!("created dummy transaction");
    tctx.client.send_and_confirm_transaction(&tx).await?;

    println!("ibc verifier created: {:?}", qed_doge_ibc_address);


    Ok(())
}
#[tokio::test]
async fn create() {
    run_simple_init_test().await.unwrap();
}
