// Based on `ss58_registry`.

use pyo3::prelude::*;

use crate::{ss58::Ss58AddressFormat, to_value_error};
use ss58_registry as ss58;

/// A known address (sub)format/network ID for SS58.
#[pyclass(eq, eq_int, str)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Ss58AccountFormat {
    /// Bare 32-bit Ed25519 public key.
    BareEd25519 = 3u16,
    /// Bare 32-bit ECDSA SECP-256k1 public key.
    BareSecp256K1 = 43u16,
    /// Bare 32-bit Schnorr/Ristretto (S/R 25519) public key.
    BareSr25519 = 1u16,
    /// DICO - <https://dico.io>
    Dico = 53u16,
    /// ICE Network - <https://icenetwork.io>
    Ice = 2206u16,
    /// KICO - <https://dico.io>
    Kico = 52u16,
    /// SNOW: ICE Canary Network - <https://icenetwork.io>
    Snow = 2207u16,
    /// Acala - <https://acala.network/>
    Acala = 10u16,
    /// Ajuna Network - <https://ajuna.io>
    Ajuna = 1328u16,
    /// Allfeat Network - <https://allfeat.network>
    AllfeatNetwork = 440u16,
    /// Altair - <https://centrifuge.io/>
    Altair = 136u16,
    /// Amplitude chain - <https://pendulumchain.org/>
    Amplitude = 57u16,
    /// Analog Timechain - <https://analog.one>
    AnalogTimechain = 12850u16,
    /// Anmol Network - <https://anmol.network/>
    Anmol = 92u16,
    /// Ares Protocol - <https://www.aresprotocol.com/>
    Ares = 34u16,
    /// Astar Network - <https://astar.network>
    Astar = 5u16,
    /// Autonomys - <https://autonomys.xyz>
    Autonomys = 6094u16,
    /// Aventus Mainnet - <https://aventus.io>
    Aventus = 65u16,
    /// Bajun Network - <https://ajuna.io>
    Bajun = 1337u16,
    /// Basilisk - <https://bsx.fi>
    Basilisk = 10041u16,
    /// Bifrost - <https://bifrost.finance/>
    Bifrost = 6u16,
    /// Bitgreen - <https://bitgreen.org/>
    Bitgreen = 2106u16,
    /// Bittensor - <https://bittensor.com>
    Bittensor = 13116u16,
    /// Calamari: Manta Canary Network - <https://manta.network>
    Calamari = 78u16,
    /// Centrifuge Chain - <https://centrifuge.io/>
    Centrifuge = 36u16,
    /// Cere Network - <https://cere.network>
    Cere = 54u16,
    /// CESS - <https://cess.cloud>
    Cess = 11331u16,
    /// CESS Testnet - <https://cess.cloud>
    CessTestnet = 11330u16,
    /// Chainflip - <https://chainflip.io/>
    Chainflip = 2112u16,
    /// ChainX - <https://chainx.org/>
    Chainx = 44u16,
    /// CloudWalk Network Mainnet - <https://explorer.mainnet.cloudwalk.io>
    CloudwalkMainnet = 2009u16,
    /// Clover Finance - <https://clover.finance>
    Clover = 128u16,
    /// Composable Finance - <https://composable.finance>
    Composable = 50u16,
    /// Automata ContextFree - <https://ata.network>
    Contextfree = 11820u16,
    /// CORD Network - <https://cord.network/>
    Cord = 29u16,
    /// Crust Network - <https://crust.network>
    Crust = 66u16,
    /// Curio - <https://parachain.capitaldex.exchange/>
    Curio = 777u16,
    /// Dark Mainnet
    Dark = 17u16,
    /// Darwinia Network - <https://darwinia.network>
    Darwinia = 18u16,
    /// DataHighway
    Datahighway = 33u16,
    /// DENTNet - <https://www.dentnet.io>
    Dentnet = 9807u16,
    /// Dock Mainnet - <https://dock.io>
    DockPosMainnet = 22u16,
    /// Dorafactory Polkadot Network - <https://dorafactory.org>
    DorafactoryPolkadot = 129u16,
    /// Edgeware - <https://edgewa.re>
    Edgeware = 7u16,
    /// Efinity - <https://efinity.io/>
    Efinity = 1110u16,
    /// Equilibrium Network - <https://equilibrium.io>
    Equilibrium = 68u16,
    /// Eternal Civilization - <http://www.ysknfr.cn/>
    EternalCivilization = 58u16,
    /// Fragnova Network - <https://fragnova.com>
    Fragnova = 93u16,
    /// Frequency - <https://www.frequency.xyz>
    Frequency = 90u16,
    /// Ğ1 - <https://duniter.org>
    G1 = 4450u16,
    /// GEEK Network - <https://geek.gl>
    Geek = 789u16,
    /// Genshiro Network - <https://genshiro.equilibrium.io>
    Genshiro = 67u16,
    /// GM - <https://gmordie.com>
    Gm = 7013u16,
    /// Golden Gate - <https://ggxchain.io/>
    GoldenGate = 8866u16,
    /// Golden Gate Sydney - <https://ggxchain.io/>
    GoldenGateSydney = 8886u16,
    /// GORO Network - <https://goro.network>
    Goro = 14697u16,
    /// Hashed Network - <https://hashed.network>
    Hashed = 9072u16,
    /// Heiko - <https://parallel.fi/>
    Heiko = 110u16,
    /// Humanode Network - <https://humanode.io>
    Humanode = 5234u16,
    /// Hydration - <https://hydration.net>
    Hydradx = 63u16,
    /// Anmol Network Ibtida Canary network - <https://anmol.network/>
    Ibtida = 100u16,
    /// Impact Protocol Network - <https://impactprotocol.network/>
    Impact = 12155u16,
    /// Integritee - <https://integritee.network>
    Integritee = 13u16,
    /// Integritee Incognito - <https://integritee.network>
    IntegriteeIncognito = 113u16,
    /// Interlay - <https://interlay.io/>
    Interlay = 2032u16,
    /// Joystream - <https://www.joystream.org>
    Joystream = 126u16,
    /// Jupiter - <https://jupiter.patract.io>
    Jupiter = 26u16,
    /// Kabocha - <https://kabocha.network>
    Kabocha = 27u16,
    /// Kapex - <https://totemaccounting.com>
    Kapex = 2007u16,
    /// Karmacoin - <https://karmaco.in>
    Karmachain = 21u16,
    /// Karura - <https://karura.network/>
    Karura = 8u16,
    /// Katal Chain
    Katalchain = 4u16,
    /// KILT Spiritnet - <https://kilt.io/>
    Kilt = 38u16,
    /// Kintsugi - <https://interlay.io/>
    Kintsugi = 2092u16,
    /// Krest Network - <https://www.peaq.network/>
    Krest = 1222u16,
    /// Krigan Network - <https://krigan.network>
    Krigan = 7306u16,
    /// Kulupu - <https://kulupu.network/>
    Kulupu = 16u16,
    /// Kusama Relay Chain - <https://kusama.network>
    Kusama = 2u16,
    /// Laminar - <http://laminar.network/>
    Laminar = 11u16,
    /// Litentry Network - <https://litentry.com/>
    Litentry = 31u16,
    /// Litmus Network - <https://litentry.com/>
    Litmus = 131u16,
    /// logion network - <https://logion.network>
    Logion = 2021u16,
    /// Luhn Network - <https://luhn.network>
    Luhn = 11486u16,
    /// Manta network - <https://manta.network>
    Manta = 77u16,
    /// MathChain mainnet - <https://mathwallet.org>
    Mathchain = 39u16,
    /// MathChain testnet - <https://mathwallet.org>
    MathchainTestnet = 40u16,
    /// Metaquity Network - <https://metaquity.xyz/>
    MetaquityNetwork = 666u16,
    /// Moonbeam - <https://moonbeam.network>
    Moonbeam = 1284u16,
    /// Moonriver - <https://moonbeam.network>
    Moonriver = 1285u16,
    /// Moonsama - <https://moonsama.com>
    Moonsama = 2199u16,
    /// Mosaic Chain - <https://mosaicchain.io>
    MosaicChain = 14998u16,
    /// Mythos - <https://mythos.foundation>
    Mythos = 29972u16,
    /// Neatcoin Mainnet - <https://neatcoin.org>
    Neatcoin = 48u16,
    /// NFTMart - <https://nftmart.io>
    Nftmart = 12191u16,
    /// Nodle Chain - <https://nodle.io/>
    Nodle = 37u16,
    /// OAK Network - <https://oak.tech>
    Oak = 51u16,
    /// OriginTrail Parachain - <https://parachain.origintrail.io/>
    OrigintrailParachain = 101u16,
    /// 3DP network - <https://3dpass.org>
    P3D = 71u16,
    /// 3DP test network - <https://3dpass.org>
    P3Dt = 72u16,
    /// Parallel - <https://parallel.fi/>
    Parallel = 172u16,
    /// Peaq Network - <https://www.peaq.network/>
    Peaq = 1221u16,
    /// Peerplays - <https://www.peerplays.com/>
    Peerplays = 3333u16,
    /// Pendulum chain - <https://pendulumchain.org/>
    Pendulum = 56u16,
    /// Phala Network - <https://phala.network>
    Phala = 30u16,
    /// Picasso - <https://picasso.composable.finance>
    Picasso = 49u16,
    /// Pioneer Network by Bit.Country - <https://bit.country>
    PioneerNetwork = 268u16,
    /// Polimec Protocol - <https://www.polimec.org/>
    Polimec = 41u16,
    /// Polkadex Mainnet - <https://polkadex.trade>
    Polkadex = 88u16,
    /// Polkadex Parachain - <https://polkadex.trade>
    Polkadexparachain = 89u16,
    /// Polkadot Relay Chain - <https://polkadot.network>
    Polkadot = 0u16,
    /// PolkaFoundry Network - <https://polkafoundry.com>
    Polkafoundry = 99u16,
    /// PolkaSmith Canary Network - <https://polkafoundry.com>
    Polkasmith = 98u16,
    /// Polymesh - <https://polymath.network/>
    Polymesh = 12u16,
    /// Pontem Network - <https://pontem.network>
    PontemNetwork = 105u16,
    /// QUARTZ by UNIQUE - <https://unique.network>
    QuartzMainnet = 255u16,
    /// This prefix is reserved.
    Reserved46 = 46u16,
    /// This prefix is reserved.
    Reserved47 = 47u16,
    /// Laminar Reynolds Canary - <http://laminar.network/>
    Reynolds = 9u16,
    /// Robonomics - <https://robonomics.network>
    Robonomics = 32u16,
    /// Sapphire by Unique - <https://unique.network>
    SapphireMainnet = 8883u16,
    /// Seals Network - <https://seals.app>
    Seals = 1985u16,
    /// ShiftNrg
    Shift = 23u16,
    /// Social Network - <https://social.network>
    SocialNetwork = 252u16,
    /// Societal - <https://www.sctl.xyz>
    Societal = 1516u16,
    /// SORA Network - <https://sora.org>
    Sora = 69u16,
    /// SORA Polkadot Parachain - <https://sora.org>
    SoraDotPara = 81u16,
    /// SORA Kusama Parachain - <https://sora.org>
    SoraKusamaPara = 420u16,
    /// Stafi - <https://stafi.io>
    Stafi = 20u16,
    /// Subsocial
    Subsocial = 28u16,
    /// Subspace testnet - <https://subspace.network>
    SubspaceTestnet = 2254u16,
    /// Substrate - <https://substrate.io/>
    Substrate = 42u16,
    /// Synesthesia - <https://synesthesia.network/>
    Synesthesia = 15u16,
    /// t3rn - <https://t3rn.io/>
    T3Rn = 9935u16,
    /// Tangle Network - <https://www.tangle.tools/>
    Tangle = 5845u16,
    /// Ternoa - <https://www.ternoa.network>
    Ternoa = 995u16,
    /// Tidefi - <https://tidefi.com>
    Tidefi = 7007u16,
    /// Tinker - <https://invarch.network>
    Tinker = 117u16,
    /// Totem - <https://totemaccounting.com>
    Totem = 14u16,
    /// UniArts Network - <https://uniarts.me>
    Uniarts = 45u16,
    /// Unique Network - <https://unique.network>
    UniqueMainnet = 7391u16,
    /// Vara Network - <https://vara.network/>
    Vara = 137u16,
    /// Valiu Liquidity Network - <https://valiu.com/>
    Vln = 35u16,
    /// Enigmatic Smile - <https://www.vow.foundation/>
    VowChain = 2024u16,
    /// Watr Protocol - <https://www.watr.org>
    Watr = 19u16,
    /// Xcavate Protocol - <https://xcavate.io/>
    Xcavate = 8888u16,
    /// xx network - <https://xx.network>
    Xxnetwork = 55u16,
    /// Zeitgeist - <https://zeitgeist.pm>
    Zeitgeist = 73u16,
    /// ZERO - <https://zero.io>
    Zero = 24u16,
    /// ZERO Alphaville - <https://zero.io>
    ZeroAlphaville = 25u16,
}

impl From<Ss58AccountFormat> for ss58::Ss58AddressFormatRegistry {
    #[rustfmt::skip]
    fn from(value: Ss58AccountFormat) -> Self {
        match value {
            Ss58AccountFormat::BareEd25519 => ss58::Ss58AddressFormatRegistry::BareEd25519Account,
            Ss58AccountFormat::BareSecp256K1 => ss58::Ss58AddressFormatRegistry::BareSecp256K1Account,
            Ss58AccountFormat::BareSr25519 => ss58::Ss58AddressFormatRegistry::BareSr25519Account,
            Ss58AccountFormat::Dico => ss58::Ss58AddressFormatRegistry::DicoAccount,
            Ss58AccountFormat::Ice => ss58::Ss58AddressFormatRegistry::IceAccount,
            Ss58AccountFormat::Kico => ss58::Ss58AddressFormatRegistry::KicoAccount,
            Ss58AccountFormat::Snow => ss58::Ss58AddressFormatRegistry::SnowAccount,
            Ss58AccountFormat::Acala => ss58::Ss58AddressFormatRegistry::AcalaAccount,
            Ss58AccountFormat::Ajuna => ss58::Ss58AddressFormatRegistry::AjunaAccount,
            Ss58AccountFormat::AllfeatNetwork => ss58::Ss58AddressFormatRegistry::AllfeatNetworkAccount,
            Ss58AccountFormat::Altair => ss58::Ss58AddressFormatRegistry::AltairAccount,
            Ss58AccountFormat::Amplitude => ss58::Ss58AddressFormatRegistry::AmplitudeAccount,
            Ss58AccountFormat::AnalogTimechain => ss58::Ss58AddressFormatRegistry::AnalogTimechainAccount,
            Ss58AccountFormat::Anmol => ss58::Ss58AddressFormatRegistry::AnmolAccount,
            Ss58AccountFormat::Ares => ss58::Ss58AddressFormatRegistry::AresAccount,
            Ss58AccountFormat::Astar => ss58::Ss58AddressFormatRegistry::AstarAccount,
            Ss58AccountFormat::Autonomys => ss58::Ss58AddressFormatRegistry::AutonomysAccount,
            Ss58AccountFormat::Aventus => ss58::Ss58AddressFormatRegistry::AventusAccount,
            Ss58AccountFormat::Bajun => ss58::Ss58AddressFormatRegistry::BajunAccount,
            Ss58AccountFormat::Basilisk => ss58::Ss58AddressFormatRegistry::BasiliskAccount,
            Ss58AccountFormat::Bifrost => ss58::Ss58AddressFormatRegistry::BifrostAccount,
            Ss58AccountFormat::Bitgreen => ss58::Ss58AddressFormatRegistry::BitgreenAccount,
            Ss58AccountFormat::Bittensor => ss58::Ss58AddressFormatRegistry::BittensorAccount,
            Ss58AccountFormat::Calamari => ss58::Ss58AddressFormatRegistry::CalamariAccount,
            Ss58AccountFormat::Centrifuge => ss58::Ss58AddressFormatRegistry::CentrifugeAccount,
            Ss58AccountFormat::Cere => ss58::Ss58AddressFormatRegistry::CereAccount,
            Ss58AccountFormat::Cess => ss58::Ss58AddressFormatRegistry::CessAccount,
            Ss58AccountFormat::CessTestnet => ss58::Ss58AddressFormatRegistry::CessTestnetAccount,
            Ss58AccountFormat::Chainflip => ss58::Ss58AddressFormatRegistry::ChainflipAccount,
            Ss58AccountFormat::Chainx => ss58::Ss58AddressFormatRegistry::ChainxAccount,
            Ss58AccountFormat::CloudwalkMainnet => ss58::Ss58AddressFormatRegistry::CloudwalkMainnetAccount,
            Ss58AccountFormat::Clover => ss58::Ss58AddressFormatRegistry::CloverAccount,
            Ss58AccountFormat::Composable => ss58::Ss58AddressFormatRegistry::ComposableAccount,
            Ss58AccountFormat::Contextfree => ss58::Ss58AddressFormatRegistry::ContextfreeAccount,
            Ss58AccountFormat::Cord => ss58::Ss58AddressFormatRegistry::CordAccount,
            Ss58AccountFormat::Crust => ss58::Ss58AddressFormatRegistry::CrustAccount,
            Ss58AccountFormat::Curio => ss58::Ss58AddressFormatRegistry::CurioAccount,
            Ss58AccountFormat::Dark => ss58::Ss58AddressFormatRegistry::DarkAccount,
            Ss58AccountFormat::Darwinia => ss58::Ss58AddressFormatRegistry::DarwiniaAccount,
            Ss58AccountFormat::Datahighway => ss58::Ss58AddressFormatRegistry::DatahighwayAccount,
            Ss58AccountFormat::Dentnet => ss58::Ss58AddressFormatRegistry::DentnetAccount,
            Ss58AccountFormat::DockPosMainnet => ss58::Ss58AddressFormatRegistry::DockPosMainnetAccount,
            Ss58AccountFormat::DorafactoryPolkadot => ss58::Ss58AddressFormatRegistry::DorafactoryPolkadotAccount,
            Ss58AccountFormat::Edgeware => ss58::Ss58AddressFormatRegistry::EdgewareAccount,
            Ss58AccountFormat::Efinity => ss58::Ss58AddressFormatRegistry::EfinityAccount,
            Ss58AccountFormat::Equilibrium => ss58::Ss58AddressFormatRegistry::EquilibriumAccount,
            Ss58AccountFormat::EternalCivilization => ss58::Ss58AddressFormatRegistry::EternalCivilizationAccount,
            Ss58AccountFormat::Fragnova => ss58::Ss58AddressFormatRegistry::FragnovaAccount,
            Ss58AccountFormat::Frequency => ss58::Ss58AddressFormatRegistry::FrequencyAccount,
            Ss58AccountFormat::G1 => ss58::Ss58AddressFormatRegistry::G1Account,
            Ss58AccountFormat::Geek => ss58::Ss58AddressFormatRegistry::GeekAccount,
            Ss58AccountFormat::Genshiro => ss58::Ss58AddressFormatRegistry::GenshiroAccount,
            Ss58AccountFormat::Gm => ss58::Ss58AddressFormatRegistry::GmAccount,
            Ss58AccountFormat::GoldenGate => ss58::Ss58AddressFormatRegistry::GoldenGateAccount,
            Ss58AccountFormat::GoldenGateSydney => ss58::Ss58AddressFormatRegistry::GoldenGateSydneyAccount,
            Ss58AccountFormat::Goro => ss58::Ss58AddressFormatRegistry::GoroAccount,
            Ss58AccountFormat::Hashed => ss58::Ss58AddressFormatRegistry::HashedAccount,
            Ss58AccountFormat::Heiko => ss58::Ss58AddressFormatRegistry::HeikoAccount,
            Ss58AccountFormat::Humanode => ss58::Ss58AddressFormatRegistry::HumanodeAccount,
            Ss58AccountFormat::Hydradx => ss58::Ss58AddressFormatRegistry::HydradxAccount,
            Ss58AccountFormat::Ibtida => ss58::Ss58AddressFormatRegistry::IbtidaAccount,
            Ss58AccountFormat::Impact => ss58::Ss58AddressFormatRegistry::ImpactAccount,
            Ss58AccountFormat::Integritee => ss58::Ss58AddressFormatRegistry::IntegriteeAccount,
            Ss58AccountFormat::IntegriteeIncognito => ss58::Ss58AddressFormatRegistry::IntegriteeIncognitoAccount,
            Ss58AccountFormat::Interlay => ss58::Ss58AddressFormatRegistry::InterlayAccount,
            Ss58AccountFormat::Joystream => ss58::Ss58AddressFormatRegistry::JoystreamAccount,
            Ss58AccountFormat::Jupiter => ss58::Ss58AddressFormatRegistry::JupiterAccount,
            Ss58AccountFormat::Kabocha => ss58::Ss58AddressFormatRegistry::KabochaAccount,
            Ss58AccountFormat::Kapex => ss58::Ss58AddressFormatRegistry::KapexAccount,
            Ss58AccountFormat::Karmachain => ss58::Ss58AddressFormatRegistry::KarmachainAccount,
            Ss58AccountFormat::Karura => ss58::Ss58AddressFormatRegistry::KaruraAccount,
            Ss58AccountFormat::Katalchain => ss58::Ss58AddressFormatRegistry::KatalchainAccount,
            Ss58AccountFormat::Kilt => ss58::Ss58AddressFormatRegistry::KiltAccount,
            Ss58AccountFormat::Kintsugi => ss58::Ss58AddressFormatRegistry::KintsugiAccount,
            Ss58AccountFormat::Krest => ss58::Ss58AddressFormatRegistry::KrestAccount,
            Ss58AccountFormat::Krigan => ss58::Ss58AddressFormatRegistry::KriganAccount,
            Ss58AccountFormat::Kulupu => ss58::Ss58AddressFormatRegistry::KulupuAccount,
            Ss58AccountFormat::Kusama => ss58::Ss58AddressFormatRegistry::KusamaAccount,
            Ss58AccountFormat::Laminar => ss58::Ss58AddressFormatRegistry::LaminarAccount,
            Ss58AccountFormat::Litentry => ss58::Ss58AddressFormatRegistry::LitentryAccount,
            Ss58AccountFormat::Litmus => ss58::Ss58AddressFormatRegistry::LitmusAccount,
            Ss58AccountFormat::Logion => ss58::Ss58AddressFormatRegistry::LogionAccount,
            Ss58AccountFormat::Luhn => ss58::Ss58AddressFormatRegistry::LuhnAccount,
            Ss58AccountFormat::Manta => ss58::Ss58AddressFormatRegistry::MantaAccount,
            Ss58AccountFormat::Mathchain => ss58::Ss58AddressFormatRegistry::MathchainAccount,
            Ss58AccountFormat::MathchainTestnet => ss58::Ss58AddressFormatRegistry::MathchainTestnetAccount,
            Ss58AccountFormat::MetaquityNetwork => ss58::Ss58AddressFormatRegistry::MetaquityNetworkAccount,
            Ss58AccountFormat::Moonbeam => ss58::Ss58AddressFormatRegistry::MoonbeamAccount,
            Ss58AccountFormat::Moonriver => ss58::Ss58AddressFormatRegistry::MoonriverAccount,
            Ss58AccountFormat::Moonsama => ss58::Ss58AddressFormatRegistry::MoonsamaAccount,
            Ss58AccountFormat::MosaicChain => ss58::Ss58AddressFormatRegistry::MosaicChainAccount,
            Ss58AccountFormat::Mythos => ss58::Ss58AddressFormatRegistry::MythosAccount,
            Ss58AccountFormat::Neatcoin => ss58::Ss58AddressFormatRegistry::NeatcoinAccount,
            Ss58AccountFormat::Nftmart => ss58::Ss58AddressFormatRegistry::NftmartAccount,
            Ss58AccountFormat::Nodle => ss58::Ss58AddressFormatRegistry::NodleAccount,
            Ss58AccountFormat::Oak => ss58::Ss58AddressFormatRegistry::OakAccount,
            Ss58AccountFormat::OrigintrailParachain => ss58::Ss58AddressFormatRegistry::OrigintrailParachainAccount,
            Ss58AccountFormat::P3D => ss58::Ss58AddressFormatRegistry::P3DAccount,
            Ss58AccountFormat::P3Dt => ss58::Ss58AddressFormatRegistry::P3DtAccount,
            Ss58AccountFormat::Parallel => ss58::Ss58AddressFormatRegistry::ParallelAccount,
            Ss58AccountFormat::Peaq => ss58::Ss58AddressFormatRegistry::PeaqAccount,
            Ss58AccountFormat::Peerplays => ss58::Ss58AddressFormatRegistry::PeerplaysAccount,
            Ss58AccountFormat::Pendulum => ss58::Ss58AddressFormatRegistry::PendulumAccount,
            Ss58AccountFormat::Phala => ss58::Ss58AddressFormatRegistry::PhalaAccount,
            Ss58AccountFormat::Picasso => ss58::Ss58AddressFormatRegistry::PicassoAccount,
            Ss58AccountFormat::PioneerNetwork => ss58::Ss58AddressFormatRegistry::PioneerNetworkAccount,
            Ss58AccountFormat::Polimec => ss58::Ss58AddressFormatRegistry::PolimecAccount,
            Ss58AccountFormat::Polkadex => ss58::Ss58AddressFormatRegistry::PolkadexAccount,
            Ss58AccountFormat::Polkadexparachain => ss58::Ss58AddressFormatRegistry::PolkadexparachainAccount,
            Ss58AccountFormat::Polkadot => ss58::Ss58AddressFormatRegistry::PolkadotAccount,
            Ss58AccountFormat::Polkafoundry => ss58::Ss58AddressFormatRegistry::PolkafoundryAccount,
            Ss58AccountFormat::Polkasmith => ss58::Ss58AddressFormatRegistry::PolkasmithAccount,
            Ss58AccountFormat::Polymesh => ss58::Ss58AddressFormatRegistry::PolymeshAccount,
            Ss58AccountFormat::PontemNetwork => ss58::Ss58AddressFormatRegistry::PontemNetworkAccount,
            Ss58AccountFormat::QuartzMainnet => ss58::Ss58AddressFormatRegistry::QuartzMainnetAccount,
            Ss58AccountFormat::Reserved46 => ss58::Ss58AddressFormatRegistry::Reserved46Account,
            Ss58AccountFormat::Reserved47 => ss58::Ss58AddressFormatRegistry::Reserved47Account,
            Ss58AccountFormat::Reynolds => ss58::Ss58AddressFormatRegistry::ReynoldsAccount,
            Ss58AccountFormat::Robonomics => ss58::Ss58AddressFormatRegistry::RobonomicsAccount,
            Ss58AccountFormat::SapphireMainnet => ss58::Ss58AddressFormatRegistry::SapphireMainnetAccount,
            Ss58AccountFormat::Seals => ss58::Ss58AddressFormatRegistry::SealsAccount,
            Ss58AccountFormat::Shift => ss58::Ss58AddressFormatRegistry::ShiftAccount,
            Ss58AccountFormat::SocialNetwork => ss58::Ss58AddressFormatRegistry::SocialNetworkAccount,
            Ss58AccountFormat::Societal => ss58::Ss58AddressFormatRegistry::SocietalAccount,
            Ss58AccountFormat::Sora => ss58::Ss58AddressFormatRegistry::SoraAccount,
            Ss58AccountFormat::SoraDotPara => ss58::Ss58AddressFormatRegistry::SoraDotParaAccount,
            Ss58AccountFormat::SoraKusamaPara => ss58::Ss58AddressFormatRegistry::SoraKusamaParaAccount,
            Ss58AccountFormat::Stafi => ss58::Ss58AddressFormatRegistry::StafiAccount,
            Ss58AccountFormat::Subsocial => ss58::Ss58AddressFormatRegistry::SubsocialAccount,
            Ss58AccountFormat::SubspaceTestnet => ss58::Ss58AddressFormatRegistry::SubspaceTestnetAccount,
            Ss58AccountFormat::Substrate => ss58::Ss58AddressFormatRegistry::SubstrateAccount,
            Ss58AccountFormat::Synesthesia => ss58::Ss58AddressFormatRegistry::SynesthesiaAccount,
            Ss58AccountFormat::T3Rn => ss58::Ss58AddressFormatRegistry::T3RnAccount,
            Ss58AccountFormat::Tangle => ss58::Ss58AddressFormatRegistry::TangleAccount,
            Ss58AccountFormat::Ternoa => ss58::Ss58AddressFormatRegistry::TernoaAccount,
            Ss58AccountFormat::Tidefi => ss58::Ss58AddressFormatRegistry::TidefiAccount,
            Ss58AccountFormat::Tinker => ss58::Ss58AddressFormatRegistry::TinkerAccount,
            Ss58AccountFormat::Totem => ss58::Ss58AddressFormatRegistry::TotemAccount,
            Ss58AccountFormat::Uniarts => ss58::Ss58AddressFormatRegistry::UniartsAccount,
            Ss58AccountFormat::UniqueMainnet => ss58::Ss58AddressFormatRegistry::UniqueMainnetAccount,
            Ss58AccountFormat::Vara => ss58::Ss58AddressFormatRegistry::VaraAccount,
            Ss58AccountFormat::Vln => ss58::Ss58AddressFormatRegistry::VlnAccount,
            Ss58AccountFormat::VowChain => ss58::Ss58AddressFormatRegistry::VowChainAccount,
            Ss58AccountFormat::Watr => ss58::Ss58AddressFormatRegistry::WatrAccount,
            Ss58AccountFormat::Xcavate => ss58::Ss58AddressFormatRegistry::XcavateAccount,
            Ss58AccountFormat::Xxnetwork => ss58::Ss58AddressFormatRegistry::XxnetworkAccount,
            Ss58AccountFormat::Zeitgeist => ss58::Ss58AddressFormatRegistry::ZeitgeistAccount,
            Ss58AccountFormat::Zero => ss58::Ss58AddressFormatRegistry::ZeroAccount,
            Ss58AccountFormat::ZeroAlphaville => ss58::Ss58AddressFormatRegistry::ZeroAlphavilleAccount,
        }
    }
}

impl From<ss58::Ss58AddressFormatRegistry> for Ss58AccountFormat {
    #[rustfmt::skip]
    fn from(value: ss58::Ss58AddressFormatRegistry) -> Self {
        match value {
            ss58::Ss58AddressFormatRegistry::BareEd25519Account => Ss58AccountFormat::BareEd25519,
            ss58::Ss58AddressFormatRegistry::BareSecp256K1Account => Ss58AccountFormat::BareSecp256K1,
            ss58::Ss58AddressFormatRegistry::BareSr25519Account => Ss58AccountFormat::BareSr25519,
            ss58::Ss58AddressFormatRegistry::DicoAccount => Ss58AccountFormat::Dico,
            ss58::Ss58AddressFormatRegistry::IceAccount => Ss58AccountFormat::Ice,
            ss58::Ss58AddressFormatRegistry::KicoAccount => Ss58AccountFormat::Kico,
            ss58::Ss58AddressFormatRegistry::SnowAccount => Ss58AccountFormat::Snow,
            ss58::Ss58AddressFormatRegistry::AcalaAccount => Ss58AccountFormat::Acala,
            ss58::Ss58AddressFormatRegistry::AjunaAccount => Ss58AccountFormat::Ajuna,
            ss58::Ss58AddressFormatRegistry::AllfeatNetworkAccount => Ss58AccountFormat::AllfeatNetwork,
            ss58::Ss58AddressFormatRegistry::AltairAccount => Ss58AccountFormat::Altair,
            ss58::Ss58AddressFormatRegistry::AmplitudeAccount => Ss58AccountFormat::Amplitude,
            ss58::Ss58AddressFormatRegistry::AnalogTimechainAccount => Ss58AccountFormat::AnalogTimechain,
            ss58::Ss58AddressFormatRegistry::AnmolAccount => Ss58AccountFormat::Anmol,
            ss58::Ss58AddressFormatRegistry::AresAccount => Ss58AccountFormat::Ares,
            ss58::Ss58AddressFormatRegistry::AstarAccount => Ss58AccountFormat::Astar,
            ss58::Ss58AddressFormatRegistry::AutonomysAccount => Ss58AccountFormat::Autonomys,
            ss58::Ss58AddressFormatRegistry::AventusAccount => Ss58AccountFormat::Aventus,
            ss58::Ss58AddressFormatRegistry::BajunAccount => Ss58AccountFormat::Bajun,
            ss58::Ss58AddressFormatRegistry::BasiliskAccount => Ss58AccountFormat::Basilisk,
            ss58::Ss58AddressFormatRegistry::BifrostAccount => Ss58AccountFormat::Bifrost,
            ss58::Ss58AddressFormatRegistry::BitgreenAccount => Ss58AccountFormat::Bitgreen,
            ss58::Ss58AddressFormatRegistry::BittensorAccount => Ss58AccountFormat::Bittensor,
            ss58::Ss58AddressFormatRegistry::CalamariAccount => Ss58AccountFormat::Calamari,
            ss58::Ss58AddressFormatRegistry::CentrifugeAccount => Ss58AccountFormat::Centrifuge,
            ss58::Ss58AddressFormatRegistry::CereAccount => Ss58AccountFormat::Cere,
            ss58::Ss58AddressFormatRegistry::CessAccount => Ss58AccountFormat::Cess,
            ss58::Ss58AddressFormatRegistry::CessTestnetAccount => Ss58AccountFormat::CessTestnet,
            ss58::Ss58AddressFormatRegistry::ChainflipAccount => Ss58AccountFormat::Chainflip,
            ss58::Ss58AddressFormatRegistry::ChainxAccount => Ss58AccountFormat::Chainx,
            ss58::Ss58AddressFormatRegistry::CloudwalkMainnetAccount => Ss58AccountFormat::CloudwalkMainnet,
            ss58::Ss58AddressFormatRegistry::CloverAccount => Ss58AccountFormat::Clover,
            ss58::Ss58AddressFormatRegistry::ComposableAccount => Ss58AccountFormat::Composable,
            ss58::Ss58AddressFormatRegistry::ContextfreeAccount => Ss58AccountFormat::Contextfree,
            ss58::Ss58AddressFormatRegistry::CordAccount => Ss58AccountFormat::Cord,
            ss58::Ss58AddressFormatRegistry::CrustAccount => Ss58AccountFormat::Crust,
            ss58::Ss58AddressFormatRegistry::CurioAccount => Ss58AccountFormat::Curio,
            ss58::Ss58AddressFormatRegistry::DarkAccount => Ss58AccountFormat::Dark,
            ss58::Ss58AddressFormatRegistry::DarwiniaAccount => Ss58AccountFormat::Darwinia,
            ss58::Ss58AddressFormatRegistry::DatahighwayAccount => Ss58AccountFormat::Datahighway,
            ss58::Ss58AddressFormatRegistry::DentnetAccount => Ss58AccountFormat::Dentnet,
            ss58::Ss58AddressFormatRegistry::DockPosMainnetAccount => Ss58AccountFormat::DockPosMainnet,
            ss58::Ss58AddressFormatRegistry::DorafactoryPolkadotAccount => Ss58AccountFormat::DorafactoryPolkadot,
            ss58::Ss58AddressFormatRegistry::EdgewareAccount => Ss58AccountFormat::Edgeware,
            ss58::Ss58AddressFormatRegistry::EfinityAccount => Ss58AccountFormat::Efinity,
            ss58::Ss58AddressFormatRegistry::EquilibriumAccount => Ss58AccountFormat::Equilibrium,
            ss58::Ss58AddressFormatRegistry::EternalCivilizationAccount => Ss58AccountFormat::EternalCivilization,
            ss58::Ss58AddressFormatRegistry::FragnovaAccount => Ss58AccountFormat::Fragnova,
            ss58::Ss58AddressFormatRegistry::FrequencyAccount => Ss58AccountFormat::Frequency,
            ss58::Ss58AddressFormatRegistry::G1Account => Ss58AccountFormat::G1,
            ss58::Ss58AddressFormatRegistry::GeekAccount => Ss58AccountFormat::Geek,
            ss58::Ss58AddressFormatRegistry::GenshiroAccount => Ss58AccountFormat::Genshiro,
            ss58::Ss58AddressFormatRegistry::GmAccount => Ss58AccountFormat::Gm,
            ss58::Ss58AddressFormatRegistry::GoldenGateAccount => Ss58AccountFormat::GoldenGate,
            ss58::Ss58AddressFormatRegistry::GoldenGateSydneyAccount => Ss58AccountFormat::GoldenGateSydney,
            ss58::Ss58AddressFormatRegistry::GoroAccount => Ss58AccountFormat::Goro,
            ss58::Ss58AddressFormatRegistry::HashedAccount => Ss58AccountFormat::Hashed,
            ss58::Ss58AddressFormatRegistry::HeikoAccount => Ss58AccountFormat::Heiko,
            ss58::Ss58AddressFormatRegistry::HumanodeAccount => Ss58AccountFormat::Humanode,
            ss58::Ss58AddressFormatRegistry::HydradxAccount => Ss58AccountFormat::Hydradx,
            ss58::Ss58AddressFormatRegistry::IbtidaAccount => Ss58AccountFormat::Ibtida,
            ss58::Ss58AddressFormatRegistry::ImpactAccount => Ss58AccountFormat::Impact,
            ss58::Ss58AddressFormatRegistry::IntegriteeAccount => Ss58AccountFormat::Integritee,
            ss58::Ss58AddressFormatRegistry::IntegriteeIncognitoAccount => Ss58AccountFormat::IntegriteeIncognito,
            ss58::Ss58AddressFormatRegistry::InterlayAccount => Ss58AccountFormat::Interlay,
            ss58::Ss58AddressFormatRegistry::JoystreamAccount => Ss58AccountFormat::Joystream,
            ss58::Ss58AddressFormatRegistry::JupiterAccount => Ss58AccountFormat::Jupiter,
            ss58::Ss58AddressFormatRegistry::KabochaAccount => Ss58AccountFormat::Kabocha,
            ss58::Ss58AddressFormatRegistry::KapexAccount => Ss58AccountFormat::Kapex,
            ss58::Ss58AddressFormatRegistry::KarmachainAccount => Ss58AccountFormat::Karmachain,
            ss58::Ss58AddressFormatRegistry::KaruraAccount => Ss58AccountFormat::Karura,
            ss58::Ss58AddressFormatRegistry::KatalchainAccount => Ss58AccountFormat::Katalchain,
            ss58::Ss58AddressFormatRegistry::KiltAccount => Ss58AccountFormat::Kilt,
            ss58::Ss58AddressFormatRegistry::KintsugiAccount => Ss58AccountFormat::Kintsugi,
            ss58::Ss58AddressFormatRegistry::KrestAccount => Ss58AccountFormat::Krest,
            ss58::Ss58AddressFormatRegistry::KriganAccount => Ss58AccountFormat::Krigan,
            ss58::Ss58AddressFormatRegistry::KulupuAccount => Ss58AccountFormat::Kulupu,
            ss58::Ss58AddressFormatRegistry::KusamaAccount => Ss58AccountFormat::Kusama,
            ss58::Ss58AddressFormatRegistry::LaminarAccount => Ss58AccountFormat::Laminar,
            ss58::Ss58AddressFormatRegistry::LitentryAccount => Ss58AccountFormat::Litentry,
            ss58::Ss58AddressFormatRegistry::LitmusAccount => Ss58AccountFormat::Litmus,
            ss58::Ss58AddressFormatRegistry::LogionAccount => Ss58AccountFormat::Logion,
            ss58::Ss58AddressFormatRegistry::LuhnAccount => Ss58AccountFormat::Luhn,
            ss58::Ss58AddressFormatRegistry::MantaAccount => Ss58AccountFormat::Manta,
            ss58::Ss58AddressFormatRegistry::MathchainAccount => Ss58AccountFormat::Mathchain,
            ss58::Ss58AddressFormatRegistry::MathchainTestnetAccount => Ss58AccountFormat::MathchainTestnet,
            ss58::Ss58AddressFormatRegistry::MetaquityNetworkAccount => Ss58AccountFormat::MetaquityNetwork,
            ss58::Ss58AddressFormatRegistry::MoonbeamAccount => Ss58AccountFormat::Moonbeam,
            ss58::Ss58AddressFormatRegistry::MoonriverAccount => Ss58AccountFormat::Moonriver,
            ss58::Ss58AddressFormatRegistry::MoonsamaAccount => Ss58AccountFormat::Moonsama,
            ss58::Ss58AddressFormatRegistry::MosaicChainAccount => Ss58AccountFormat::MosaicChain,
            ss58::Ss58AddressFormatRegistry::MythosAccount => Ss58AccountFormat::Mythos,
            ss58::Ss58AddressFormatRegistry::NeatcoinAccount => Ss58AccountFormat::Neatcoin,
            ss58::Ss58AddressFormatRegistry::NftmartAccount => Ss58AccountFormat::Nftmart,
            ss58::Ss58AddressFormatRegistry::NodleAccount => Ss58AccountFormat::Nodle,
            ss58::Ss58AddressFormatRegistry::OakAccount => Ss58AccountFormat::Oak,
            ss58::Ss58AddressFormatRegistry::OrigintrailParachainAccount => Ss58AccountFormat::OrigintrailParachain,
            ss58::Ss58AddressFormatRegistry::P3DAccount => Ss58AccountFormat::P3D,
            ss58::Ss58AddressFormatRegistry::P3DtAccount => Ss58AccountFormat::P3Dt,
            ss58::Ss58AddressFormatRegistry::ParallelAccount => Ss58AccountFormat::Parallel,
            ss58::Ss58AddressFormatRegistry::PeaqAccount => Ss58AccountFormat::Peaq,
            ss58::Ss58AddressFormatRegistry::PeerplaysAccount => Ss58AccountFormat::Peerplays,
            ss58::Ss58AddressFormatRegistry::PendulumAccount => Ss58AccountFormat::Pendulum,
            ss58::Ss58AddressFormatRegistry::PhalaAccount => Ss58AccountFormat::Phala,
            ss58::Ss58AddressFormatRegistry::PicassoAccount => Ss58AccountFormat::Picasso,
            ss58::Ss58AddressFormatRegistry::PioneerNetworkAccount => Ss58AccountFormat::PioneerNetwork,
            ss58::Ss58AddressFormatRegistry::PolimecAccount => Ss58AccountFormat::Polimec,
            ss58::Ss58AddressFormatRegistry::PolkadexAccount => Ss58AccountFormat::Polkadex,
            ss58::Ss58AddressFormatRegistry::PolkadexparachainAccount => Ss58AccountFormat::Polkadexparachain,
            ss58::Ss58AddressFormatRegistry::PolkadotAccount => Ss58AccountFormat::Polkadot,
            ss58::Ss58AddressFormatRegistry::PolkafoundryAccount => Ss58AccountFormat::Polkafoundry,
            ss58::Ss58AddressFormatRegistry::PolkasmithAccount => Ss58AccountFormat::Polkasmith,
            ss58::Ss58AddressFormatRegistry::PolymeshAccount => Ss58AccountFormat::Polymesh,
            ss58::Ss58AddressFormatRegistry::PontemNetworkAccount => Ss58AccountFormat::PontemNetwork,
            ss58::Ss58AddressFormatRegistry::QuartzMainnetAccount => Ss58AccountFormat::QuartzMainnet,
            ss58::Ss58AddressFormatRegistry::Reserved46Account => Ss58AccountFormat::Reserved46,
            ss58::Ss58AddressFormatRegistry::Reserved47Account => Ss58AccountFormat::Reserved47,
            ss58::Ss58AddressFormatRegistry::ReynoldsAccount => Ss58AccountFormat::Reynolds,
            ss58::Ss58AddressFormatRegistry::RobonomicsAccount => Ss58AccountFormat::Robonomics,
            ss58::Ss58AddressFormatRegistry::SapphireMainnetAccount => Ss58AccountFormat::SapphireMainnet,
            ss58::Ss58AddressFormatRegistry::SealsAccount => Ss58AccountFormat::Seals,
            ss58::Ss58AddressFormatRegistry::ShiftAccount => Ss58AccountFormat::Shift,
            ss58::Ss58AddressFormatRegistry::SocialNetworkAccount => Ss58AccountFormat::SocialNetwork,
            ss58::Ss58AddressFormatRegistry::SocietalAccount => Ss58AccountFormat::Societal,
            ss58::Ss58AddressFormatRegistry::SoraAccount => Ss58AccountFormat::Sora,
            ss58::Ss58AddressFormatRegistry::SoraDotParaAccount => Ss58AccountFormat::SoraDotPara,
            ss58::Ss58AddressFormatRegistry::SoraKusamaParaAccount => Ss58AccountFormat::SoraKusamaPara,
            ss58::Ss58AddressFormatRegistry::StafiAccount => Ss58AccountFormat::Stafi,
            ss58::Ss58AddressFormatRegistry::SubsocialAccount => Ss58AccountFormat::Subsocial,
            ss58::Ss58AddressFormatRegistry::SubspaceTestnetAccount => Ss58AccountFormat::SubspaceTestnet,
            ss58::Ss58AddressFormatRegistry::SubstrateAccount => Ss58AccountFormat::Substrate,
            ss58::Ss58AddressFormatRegistry::SynesthesiaAccount => Ss58AccountFormat::Synesthesia,
            ss58::Ss58AddressFormatRegistry::T3RnAccount => Ss58AccountFormat::T3Rn,
            ss58::Ss58AddressFormatRegistry::TangleAccount => Ss58AccountFormat::Tangle,
            ss58::Ss58AddressFormatRegistry::TernoaAccount => Ss58AccountFormat::Ternoa,
            ss58::Ss58AddressFormatRegistry::TidefiAccount => Ss58AccountFormat::Tidefi,
            ss58::Ss58AddressFormatRegistry::TinkerAccount => Ss58AccountFormat::Tinker,
            ss58::Ss58AddressFormatRegistry::TotemAccount => Ss58AccountFormat::Totem,
            ss58::Ss58AddressFormatRegistry::UniartsAccount => Ss58AccountFormat::Uniarts,
            ss58::Ss58AddressFormatRegistry::UniqueMainnetAccount => Ss58AccountFormat::UniqueMainnet,
            ss58::Ss58AddressFormatRegistry::VaraAccount => Ss58AccountFormat::Vara,
            ss58::Ss58AddressFormatRegistry::VlnAccount => Ss58AccountFormat::Vln,
            ss58::Ss58AddressFormatRegistry::VowChainAccount => Ss58AccountFormat::VowChain,
            ss58::Ss58AddressFormatRegistry::WatrAccount => Ss58AccountFormat::Watr,
            ss58::Ss58AddressFormatRegistry::XcavateAccount => Ss58AccountFormat::Xcavate,
            ss58::Ss58AddressFormatRegistry::XxnetworkAccount => Ss58AccountFormat::Xxnetwork,
            ss58::Ss58AddressFormatRegistry::ZeitgeistAccount => Ss58AccountFormat::Zeitgeist,
            ss58::Ss58AddressFormatRegistry::ZeroAccount => Ss58AccountFormat::Zero,
            ss58::Ss58AddressFormatRegistry::ZeroAlphavilleAccount => Ss58AccountFormat::ZeroAlphaville,
            _ => unreachable!()
        }
    }
}

#[pymethods]
impl Ss58AccountFormat {
    #[staticmethod]
    pub fn from_format(format: Ss58AddressFormat) -> PyResult<Self> {
        ss58::Ss58AddressFormatRegistry::try_from(format.0)
            .map(Into::into)
            .map_err(to_value_error)
    }

    #[staticmethod]
    pub fn from_name(name: &str) -> PyResult<Self> {
        let registry = ss58::Ss58AddressFormatRegistry::try_from(name).map_err(to_value_error)?;
        Ok(registry.into())
    }
}

impl std::fmt::Display for Ss58AccountFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ss58::Ss58AddressFormatRegistry::from(*self).fmt(f)
    }
}
