// Based on `ss58_registry`.

use pyo3::prelude::*;

use crate::{ss58::Ss58AddressFormat, to_value_error};
use ss58_registry as ss58;

/// A known address (sub)format/network ID for SS58.
#[pyclass(eq, eq_int, str)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Ss58AddressFormatRegistry {
    /// Bare 32-bit Ed25519 public key.
    BareEd25519Account = 3u16,
    /// Bare 32-bit ECDSA SECP-256k1 public key.
    BareSecp256K1Account = 43u16,
    /// Bare 32-bit Schnorr/Ristretto (S/R 25519) public key.
    BareSr25519Account = 1u16,
    /// DICO - <https://dico.io>
    DicoAccount = 53u16,
    /// ICE Network - <https://icenetwork.io>
    IceAccount = 2206u16,
    /// KICO - <https://dico.io>
    KicoAccount = 52u16,
    /// SNOW: ICE Canary Network - <https://icenetwork.io>
    SnowAccount = 2207u16,
    /// Acala - <https://acala.network/>
    AcalaAccount = 10u16,
    /// Ajuna Network - <https://ajuna.io>
    AjunaAccount = 1328u16,
    /// Allfeat Network - <https://allfeat.network>
    AllfeatNetworkAccount = 440u16,
    /// Altair - <https://centrifuge.io/>
    AltairAccount = 136u16,
    /// Amplitude chain - <https://pendulumchain.org/>
    AmplitudeAccount = 57u16,
    /// Analog Timechain - <https://analog.one>
    AnalogTimechainAccount = 12850u16,
    /// Anmol Network - <https://anmol.network/>
    AnmolAccount = 92u16,
    /// Ares Protocol - <https://www.aresprotocol.com/>
    AresAccount = 34u16,
    /// Astar Network - <https://astar.network>
    AstarAccount = 5u16,
    /// Autonomys - <https://autonomys.xyz>
    AutonomysAccount = 6094u16,
    /// Aventus Mainnet - <https://aventus.io>
    AventusAccount = 65u16,
    /// Bajun Network - <https://ajuna.io>
    BajunAccount = 1337u16,
    /// Basilisk - <https://bsx.fi>
    BasiliskAccount = 10041u16,
    /// Bifrost - <https://bifrost.finance/>
    BifrostAccount = 6u16,
    /// Bitgreen - <https://bitgreen.org/>
    BitgreenAccount = 2106u16,
    /// Bittensor - <https://bittensor.com>
    BittensorAccount = 13116u16,
    /// Calamari: Manta Canary Network - <https://manta.network>
    CalamariAccount = 78u16,
    /// Centrifuge Chain - <https://centrifuge.io/>
    CentrifugeAccount = 36u16,
    /// Cere Network - <https://cere.network>
    CereAccount = 54u16,
    /// CESS - <https://cess.cloud>
    CessAccount = 11331u16,
    /// CESS Testnet - <https://cess.cloud>
    CessTestnetAccount = 11330u16,
    /// Chainflip - <https://chainflip.io/>
    ChainflipAccount = 2112u16,
    /// ChainX - <https://chainx.org/>
    ChainxAccount = 44u16,
    /// CloudWalk Network Mainnet - <https://explorer.mainnet.cloudwalk.io>
    CloudwalkMainnetAccount = 2009u16,
    /// Clover Finance - <https://clover.finance>
    CloverAccount = 128u16,
    /// Composable Finance - <https://composable.finance>
    ComposableAccount = 50u16,
    /// Automata ContextFree - <https://ata.network>
    ContextfreeAccount = 11820u16,
    /// CORD Network - <https://cord.network/>
    CordAccount = 29u16,
    /// Crust Network - <https://crust.network>
    CrustAccount = 66u16,
    /// Curio - <https://parachain.capitaldex.exchange/>
    CurioAccount = 777u16,
    /// Dark Mainnet
    DarkAccount = 17u16,
    /// Darwinia Network - <https://darwinia.network>
    DarwiniaAccount = 18u16,
    /// DataHighway
    DatahighwayAccount = 33u16,
    /// DENTNet - <https://www.dentnet.io>
    DentnetAccount = 9807u16,
    /// Dock Mainnet - <https://dock.io>
    DockPosMainnetAccount = 22u16,
    /// Dorafactory Polkadot Network - <https://dorafactory.org>
    DorafactoryPolkadotAccount = 129u16,
    /// Edgeware - <https://edgewa.re>
    EdgewareAccount = 7u16,
    /// Efinity - <https://efinity.io/>
    EfinityAccount = 1110u16,
    /// Equilibrium Network - <https://equilibrium.io>
    EquilibriumAccount = 68u16,
    /// Eternal Civilization - <http://www.ysknfr.cn/>
    EternalCivilizationAccount = 58u16,
    /// Fragnova Network - <https://fragnova.com>
    FragnovaAccount = 93u16,
    /// Frequency - <https://www.frequency.xyz>
    FrequencyAccount = 90u16,
    /// Ğ1 - <https://duniter.org>
    G1Account = 4450u16,
    /// GEEK Network - <https://geek.gl>
    GeekAccount = 789u16,
    /// Genshiro Network - <https://genshiro.equilibrium.io>
    GenshiroAccount = 67u16,
    /// GM - <https://gmordie.com>
    GmAccount = 7013u16,
    /// Golden Gate - <https://ggxchain.io/>
    GoldenGateAccount = 8866u16,
    /// Golden Gate Sydney - <https://ggxchain.io/>
    GoldenGateSydneyAccount = 8886u16,
    /// GORO Network - <https://goro.network>
    GoroAccount = 14697u16,
    /// Hashed Network - <https://hashed.network>
    HashedAccount = 9072u16,
    /// Heiko - <https://parallel.fi/>
    HeikoAccount = 110u16,
    /// Humanode Network - <https://humanode.io>
    HumanodeAccount = 5234u16,
    /// Hydration - <https://hydration.net>
    HydradxAccount = 63u16,
    /// Anmol Network Ibtida Canary network - <https://anmol.network/>
    IbtidaAccount = 100u16,
    /// Impact Protocol Network - <https://impactprotocol.network/>
    ImpactAccount = 12155u16,
    /// Integritee - <https://integritee.network>
    IntegriteeAccount = 13u16,
    /// Integritee Incognito - <https://integritee.network>
    IntegriteeIncognitoAccount = 113u16,
    /// Interlay - <https://interlay.io/>
    InterlayAccount = 2032u16,
    /// Joystream - <https://www.joystream.org>
    JoystreamAccount = 126u16,
    /// Jupiter - <https://jupiter.patract.io>
    JupiterAccount = 26u16,
    /// Kabocha - <https://kabocha.network>
    KabochaAccount = 27u16,
    /// Kapex - <https://totemaccounting.com>
    KapexAccount = 2007u16,
    /// Karmacoin - <https://karmaco.in>
    KarmachainAccount = 21u16,
    /// Karura - <https://karura.network/>
    KaruraAccount = 8u16,
    /// Katal Chain
    KatalchainAccount = 4u16,
    /// KILT Spiritnet - <https://kilt.io/>
    KiltAccount = 38u16,
    /// Kintsugi - <https://interlay.io/>
    KintsugiAccount = 2092u16,
    /// Krest Network - <https://www.peaq.network/>
    KrestAccount = 1222u16,
    /// Krigan Network - <https://krigan.network>
    KriganAccount = 7306u16,
    /// Kulupu - <https://kulupu.network/>
    KulupuAccount = 16u16,
    /// Kusama Relay Chain - <https://kusama.network>
    KusamaAccount = 2u16,
    /// Laminar - <http://laminar.network/>
    LaminarAccount = 11u16,
    /// Litentry Network - <https://litentry.com/>
    LitentryAccount = 31u16,
    /// Litmus Network - <https://litentry.com/>
    LitmusAccount = 131u16,
    /// logion network - <https://logion.network>
    LogionAccount = 2021u16,
    /// Luhn Network - <https://luhn.network>
    LuhnAccount = 11486u16,
    /// Manta network - <https://manta.network>
    MantaAccount = 77u16,
    /// MathChain mainnet - <https://mathwallet.org>
    MathchainAccount = 39u16,
    /// MathChain testnet - <https://mathwallet.org>
    MathchainTestnetAccount = 40u16,
    /// Metaquity Network - <https://metaquity.xyz/>
    MetaquityNetworkAccount = 666u16,
    /// Moonbeam - <https://moonbeam.network>
    MoonbeamAccount = 1284u16,
    /// Moonriver - <https://moonbeam.network>
    MoonriverAccount = 1285u16,
    /// Moonsama - <https://moonsama.com>
    MoonsamaAccount = 2199u16,
    /// Mosaic Chain - <https://mosaicchain.io>
    MosaicChainAccount = 14998u16,
    /// Mythos - <https://mythos.foundation>
    MythosAccount = 29972u16,
    /// Neatcoin Mainnet - <https://neatcoin.org>
    NeatcoinAccount = 48u16,
    /// NFTMart - <https://nftmart.io>
    NftmartAccount = 12191u16,
    /// Nodle Chain - <https://nodle.io/>
    NodleAccount = 37u16,
    /// OAK Network - <https://oak.tech>
    OakAccount = 51u16,
    /// OriginTrail Parachain - <https://parachain.origintrail.io/>
    OrigintrailParachainAccount = 101u16,
    /// 3DP network - <https://3dpass.org>
    P3DAccount = 71u16,
    /// 3DP test network - <https://3dpass.org>
    P3DtAccount = 72u16,
    /// Parallel - <https://parallel.fi/>
    ParallelAccount = 172u16,
    /// Peaq Network - <https://www.peaq.network/>
    PeaqAccount = 1221u16,
    /// Peerplays - <https://www.peerplays.com/>
    PeerplaysAccount = 3333u16,
    /// Pendulum chain - <https://pendulumchain.org/>
    PendulumAccount = 56u16,
    /// Phala Network - <https://phala.network>
    PhalaAccount = 30u16,
    /// Picasso - <https://picasso.composable.finance>
    PicassoAccount = 49u16,
    /// Pioneer Network by Bit.Country - <https://bit.country>
    PioneerNetworkAccount = 268u16,
    /// Polimec Protocol - <https://www.polimec.org/>
    PolimecAccount = 41u16,
    /// Polkadex Mainnet - <https://polkadex.trade>
    PolkadexAccount = 88u16,
    /// Polkadex Parachain - <https://polkadex.trade>
    PolkadexparachainAccount = 89u16,
    /// Polkadot Relay Chain - <https://polkadot.network>
    PolkadotAccount = 0u16,
    /// PolkaFoundry Network - <https://polkafoundry.com>
    PolkafoundryAccount = 99u16,
    /// PolkaSmith Canary Network - <https://polkafoundry.com>
    PolkasmithAccount = 98u16,
    /// Polymesh - <https://polymath.network/>
    PolymeshAccount = 12u16,
    /// Pontem Network - <https://pontem.network>
    PontemNetworkAccount = 105u16,
    /// QUARTZ by UNIQUE - <https://unique.network>
    QuartzMainnetAccount = 255u16,
    /// This prefix is reserved.
    Reserved46Account = 46u16,
    /// This prefix is reserved.
    Reserved47Account = 47u16,
    /// Laminar Reynolds Canary - <http://laminar.network/>
    ReynoldsAccount = 9u16,
    /// Robonomics - <https://robonomics.network>
    RobonomicsAccount = 32u16,
    /// Sapphire by Unique - <https://unique.network>
    SapphireMainnetAccount = 8883u16,
    /// Seals Network - <https://seals.app>
    SealsAccount = 1985u16,
    /// ShiftNrg
    ShiftAccount = 23u16,
    /// Social Network - <https://social.network>
    SocialNetworkAccount = 252u16,
    /// Societal - <https://www.sctl.xyz>
    SocietalAccount = 1516u16,
    /// SORA Network - <https://sora.org>
    SoraAccount = 69u16,
    /// SORA Polkadot Parachain - <https://sora.org>
    SoraDotParaAccount = 81u16,
    /// SORA Kusama Parachain - <https://sora.org>
    SoraKusamaParaAccount = 420u16,
    /// Stafi - <https://stafi.io>
    StafiAccount = 20u16,
    /// Subsocial
    SubsocialAccount = 28u16,
    /// Subspace testnet - <https://subspace.network>
    SubspaceTestnetAccount = 2254u16,
    /// Substrate - <https://substrate.io/>
    SubstrateAccount = 42u16,
    /// Synesthesia - <https://synesthesia.network/>
    SynesthesiaAccount = 15u16,
    /// t3rn - <https://t3rn.io/>
    T3RnAccount = 9935u16,
    /// Tangle Network - <https://www.tangle.tools/>
    TangleAccount = 5845u16,
    /// Ternoa - <https://www.ternoa.network>
    TernoaAccount = 995u16,
    /// Tidefi - <https://tidefi.com>
    TidefiAccount = 7007u16,
    /// Tinker - <https://invarch.network>
    TinkerAccount = 117u16,
    /// Totem - <https://totemaccounting.com>
    TotemAccount = 14u16,
    /// UniArts Network - <https://uniarts.me>
    UniartsAccount = 45u16,
    /// Unique Network - <https://unique.network>
    UniqueMainnetAccount = 7391u16,
    /// Vara Network - <https://vara.network/>
    VaraAccount = 137u16,
    /// Valiu Liquidity Network - <https://valiu.com/>
    VlnAccount = 35u16,
    /// Enigmatic Smile - <https://www.vow.foundation/>
    VowChainAccount = 2024u16,
    /// Watr Protocol - <https://www.watr.org>
    WatrAccount = 19u16,
    /// Xcavate Protocol - <https://xcavate.io/>
    XcavateAccount = 8888u16,
    /// xx network - <https://xx.network>
    XxnetworkAccount = 55u16,
    /// Zeitgeist - <https://zeitgeist.pm>
    ZeitgeistAccount = 73u16,
    /// ZERO - <https://zero.io>
    ZeroAccount = 24u16,
    /// ZERO Alphaville - <https://zero.io>
    ZeroAlphavilleAccount = 25u16,
}

impl From<Ss58AddressFormatRegistry> for ss58::Ss58AddressFormatRegistry {
    #[rustfmt::skip]
    fn from(value: Ss58AddressFormatRegistry) -> Self {
        match value {
            Ss58AddressFormatRegistry::BareEd25519Account => ss58::Ss58AddressFormatRegistry::BareEd25519Account,
            Ss58AddressFormatRegistry::BareSecp256K1Account => ss58::Ss58AddressFormatRegistry::BareSecp256K1Account,
            Ss58AddressFormatRegistry::BareSr25519Account => ss58::Ss58AddressFormatRegistry::BareSr25519Account,
            Ss58AddressFormatRegistry::DicoAccount => ss58::Ss58AddressFormatRegistry::DicoAccount,
            Ss58AddressFormatRegistry::IceAccount => ss58::Ss58AddressFormatRegistry::IceAccount,
            Ss58AddressFormatRegistry::KicoAccount => ss58::Ss58AddressFormatRegistry::KicoAccount,
            Ss58AddressFormatRegistry::SnowAccount => ss58::Ss58AddressFormatRegistry::SnowAccount,
            Ss58AddressFormatRegistry::AcalaAccount => ss58::Ss58AddressFormatRegistry::AcalaAccount,
            Ss58AddressFormatRegistry::AjunaAccount => ss58::Ss58AddressFormatRegistry::AjunaAccount,
            Ss58AddressFormatRegistry::AllfeatNetworkAccount => ss58::Ss58AddressFormatRegistry::AllfeatNetworkAccount,
            Ss58AddressFormatRegistry::AltairAccount => ss58::Ss58AddressFormatRegistry::AltairAccount,
            Ss58AddressFormatRegistry::AmplitudeAccount => ss58::Ss58AddressFormatRegistry::AmplitudeAccount,
            Ss58AddressFormatRegistry::AnalogTimechainAccount => ss58::Ss58AddressFormatRegistry::AnalogTimechainAccount,
            Ss58AddressFormatRegistry::AnmolAccount => ss58::Ss58AddressFormatRegistry::AnmolAccount,
            Ss58AddressFormatRegistry::AresAccount => ss58::Ss58AddressFormatRegistry::AresAccount,
            Ss58AddressFormatRegistry::AstarAccount => ss58::Ss58AddressFormatRegistry::AstarAccount,
            Ss58AddressFormatRegistry::AutonomysAccount => ss58::Ss58AddressFormatRegistry::AutonomysAccount,
            Ss58AddressFormatRegistry::AventusAccount => ss58::Ss58AddressFormatRegistry::AventusAccount,
            Ss58AddressFormatRegistry::BajunAccount => ss58::Ss58AddressFormatRegistry::BajunAccount,
            Ss58AddressFormatRegistry::BasiliskAccount => ss58::Ss58AddressFormatRegistry::BasiliskAccount,
            Ss58AddressFormatRegistry::BifrostAccount => ss58::Ss58AddressFormatRegistry::BifrostAccount,
            Ss58AddressFormatRegistry::BitgreenAccount => ss58::Ss58AddressFormatRegistry::BitgreenAccount,
            Ss58AddressFormatRegistry::BittensorAccount => ss58::Ss58AddressFormatRegistry::BittensorAccount,
            Ss58AddressFormatRegistry::CalamariAccount => ss58::Ss58AddressFormatRegistry::CalamariAccount,
            Ss58AddressFormatRegistry::CentrifugeAccount => ss58::Ss58AddressFormatRegistry::CentrifugeAccount,
            Ss58AddressFormatRegistry::CereAccount => ss58::Ss58AddressFormatRegistry::CereAccount,
            Ss58AddressFormatRegistry::CessAccount => ss58::Ss58AddressFormatRegistry::CessAccount,
            Ss58AddressFormatRegistry::CessTestnetAccount => ss58::Ss58AddressFormatRegistry::CessTestnetAccount,
            Ss58AddressFormatRegistry::ChainflipAccount => ss58::Ss58AddressFormatRegistry::ChainflipAccount,
            Ss58AddressFormatRegistry::ChainxAccount => ss58::Ss58AddressFormatRegistry::ChainxAccount,
            Ss58AddressFormatRegistry::CloudwalkMainnetAccount => ss58::Ss58AddressFormatRegistry::CloudwalkMainnetAccount,
            Ss58AddressFormatRegistry::CloverAccount => ss58::Ss58AddressFormatRegistry::CloverAccount,
            Ss58AddressFormatRegistry::ComposableAccount => ss58::Ss58AddressFormatRegistry::ComposableAccount,
            Ss58AddressFormatRegistry::ContextfreeAccount => ss58::Ss58AddressFormatRegistry::ContextfreeAccount,
            Ss58AddressFormatRegistry::CordAccount => ss58::Ss58AddressFormatRegistry::CordAccount,
            Ss58AddressFormatRegistry::CrustAccount => ss58::Ss58AddressFormatRegistry::CrustAccount,
            Ss58AddressFormatRegistry::CurioAccount => ss58::Ss58AddressFormatRegistry::CurioAccount,
            Ss58AddressFormatRegistry::DarkAccount => ss58::Ss58AddressFormatRegistry::DarkAccount,
            Ss58AddressFormatRegistry::DarwiniaAccount => ss58::Ss58AddressFormatRegistry::DarwiniaAccount,
            Ss58AddressFormatRegistry::DatahighwayAccount => ss58::Ss58AddressFormatRegistry::DatahighwayAccount,
            Ss58AddressFormatRegistry::DentnetAccount => ss58::Ss58AddressFormatRegistry::DentnetAccount,
            Ss58AddressFormatRegistry::DockPosMainnetAccount => ss58::Ss58AddressFormatRegistry::DockPosMainnetAccount,
            Ss58AddressFormatRegistry::DorafactoryPolkadotAccount => ss58::Ss58AddressFormatRegistry::DorafactoryPolkadotAccount,
            Ss58AddressFormatRegistry::EdgewareAccount => ss58::Ss58AddressFormatRegistry::EdgewareAccount,
            Ss58AddressFormatRegistry::EfinityAccount => ss58::Ss58AddressFormatRegistry::EfinityAccount,
            Ss58AddressFormatRegistry::EquilibriumAccount => ss58::Ss58AddressFormatRegistry::EquilibriumAccount,
            Ss58AddressFormatRegistry::EternalCivilizationAccount => ss58::Ss58AddressFormatRegistry::EternalCivilizationAccount,
            Ss58AddressFormatRegistry::FragnovaAccount => ss58::Ss58AddressFormatRegistry::FragnovaAccount,
            Ss58AddressFormatRegistry::FrequencyAccount => ss58::Ss58AddressFormatRegistry::FrequencyAccount,
            Ss58AddressFormatRegistry::G1Account => ss58::Ss58AddressFormatRegistry::G1Account,
            Ss58AddressFormatRegistry::GeekAccount => ss58::Ss58AddressFormatRegistry::GeekAccount,
            Ss58AddressFormatRegistry::GenshiroAccount => ss58::Ss58AddressFormatRegistry::GenshiroAccount,
            Ss58AddressFormatRegistry::GmAccount => ss58::Ss58AddressFormatRegistry::GmAccount,
            Ss58AddressFormatRegistry::GoldenGateAccount => ss58::Ss58AddressFormatRegistry::GoldenGateAccount,
            Ss58AddressFormatRegistry::GoldenGateSydneyAccount => ss58::Ss58AddressFormatRegistry::GoldenGateSydneyAccount,
            Ss58AddressFormatRegistry::GoroAccount => ss58::Ss58AddressFormatRegistry::GoroAccount,
            Ss58AddressFormatRegistry::HashedAccount => ss58::Ss58AddressFormatRegistry::HashedAccount,
            Ss58AddressFormatRegistry::HeikoAccount => ss58::Ss58AddressFormatRegistry::HeikoAccount,
            Ss58AddressFormatRegistry::HumanodeAccount => ss58::Ss58AddressFormatRegistry::HumanodeAccount,
            Ss58AddressFormatRegistry::HydradxAccount => ss58::Ss58AddressFormatRegistry::HydradxAccount,
            Ss58AddressFormatRegistry::IbtidaAccount => ss58::Ss58AddressFormatRegistry::IbtidaAccount,
            Ss58AddressFormatRegistry::ImpactAccount => ss58::Ss58AddressFormatRegistry::ImpactAccount,
            Ss58AddressFormatRegistry::IntegriteeAccount => ss58::Ss58AddressFormatRegistry::IntegriteeAccount,
            Ss58AddressFormatRegistry::IntegriteeIncognitoAccount => ss58::Ss58AddressFormatRegistry::IntegriteeIncognitoAccount,
            Ss58AddressFormatRegistry::InterlayAccount => ss58::Ss58AddressFormatRegistry::InterlayAccount,
            Ss58AddressFormatRegistry::JoystreamAccount => ss58::Ss58AddressFormatRegistry::JoystreamAccount,
            Ss58AddressFormatRegistry::JupiterAccount => ss58::Ss58AddressFormatRegistry::JupiterAccount,
            Ss58AddressFormatRegistry::KabochaAccount => ss58::Ss58AddressFormatRegistry::KabochaAccount,
            Ss58AddressFormatRegistry::KapexAccount => ss58::Ss58AddressFormatRegistry::KapexAccount,
            Ss58AddressFormatRegistry::KarmachainAccount => ss58::Ss58AddressFormatRegistry::KarmachainAccount,
            Ss58AddressFormatRegistry::KaruraAccount => ss58::Ss58AddressFormatRegistry::KaruraAccount,
            Ss58AddressFormatRegistry::KatalchainAccount => ss58::Ss58AddressFormatRegistry::KatalchainAccount,
            Ss58AddressFormatRegistry::KiltAccount => ss58::Ss58AddressFormatRegistry::KiltAccount,
            Ss58AddressFormatRegistry::KintsugiAccount => ss58::Ss58AddressFormatRegistry::KintsugiAccount,
            Ss58AddressFormatRegistry::KrestAccount => ss58::Ss58AddressFormatRegistry::KrestAccount,
            Ss58AddressFormatRegistry::KriganAccount => ss58::Ss58AddressFormatRegistry::KriganAccount,
            Ss58AddressFormatRegistry::KulupuAccount => ss58::Ss58AddressFormatRegistry::KulupuAccount,
            Ss58AddressFormatRegistry::KusamaAccount => ss58::Ss58AddressFormatRegistry::KusamaAccount,
            Ss58AddressFormatRegistry::LaminarAccount => ss58::Ss58AddressFormatRegistry::LaminarAccount,
            Ss58AddressFormatRegistry::LitentryAccount => ss58::Ss58AddressFormatRegistry::LitentryAccount,
            Ss58AddressFormatRegistry::LitmusAccount => ss58::Ss58AddressFormatRegistry::LitmusAccount,
            Ss58AddressFormatRegistry::LogionAccount => ss58::Ss58AddressFormatRegistry::LogionAccount,
            Ss58AddressFormatRegistry::LuhnAccount => ss58::Ss58AddressFormatRegistry::LuhnAccount,
            Ss58AddressFormatRegistry::MantaAccount => ss58::Ss58AddressFormatRegistry::MantaAccount,
            Ss58AddressFormatRegistry::MathchainAccount => ss58::Ss58AddressFormatRegistry::MathchainAccount,
            Ss58AddressFormatRegistry::MathchainTestnetAccount => ss58::Ss58AddressFormatRegistry::MathchainTestnetAccount,
            Ss58AddressFormatRegistry::MetaquityNetworkAccount => ss58::Ss58AddressFormatRegistry::MetaquityNetworkAccount,
            Ss58AddressFormatRegistry::MoonbeamAccount => ss58::Ss58AddressFormatRegistry::MoonbeamAccount,
            Ss58AddressFormatRegistry::MoonriverAccount => ss58::Ss58AddressFormatRegistry::MoonriverAccount,
            Ss58AddressFormatRegistry::MoonsamaAccount => ss58::Ss58AddressFormatRegistry::MoonsamaAccount,
            Ss58AddressFormatRegistry::MosaicChainAccount => ss58::Ss58AddressFormatRegistry::MosaicChainAccount,
            Ss58AddressFormatRegistry::MythosAccount => ss58::Ss58AddressFormatRegistry::MythosAccount,
            Ss58AddressFormatRegistry::NeatcoinAccount => ss58::Ss58AddressFormatRegistry::NeatcoinAccount,
            Ss58AddressFormatRegistry::NftmartAccount => ss58::Ss58AddressFormatRegistry::NftmartAccount,
            Ss58AddressFormatRegistry::NodleAccount => ss58::Ss58AddressFormatRegistry::NodleAccount,
            Ss58AddressFormatRegistry::OakAccount => ss58::Ss58AddressFormatRegistry::OakAccount,
            Ss58AddressFormatRegistry::OrigintrailParachainAccount => ss58::Ss58AddressFormatRegistry::OrigintrailParachainAccount,
            Ss58AddressFormatRegistry::P3DAccount => ss58::Ss58AddressFormatRegistry::P3DAccount,
            Ss58AddressFormatRegistry::P3DtAccount => ss58::Ss58AddressFormatRegistry::P3DtAccount,
            Ss58AddressFormatRegistry::ParallelAccount => ss58::Ss58AddressFormatRegistry::ParallelAccount,
            Ss58AddressFormatRegistry::PeaqAccount => ss58::Ss58AddressFormatRegistry::PeaqAccount,
            Ss58AddressFormatRegistry::PeerplaysAccount => ss58::Ss58AddressFormatRegistry::PeerplaysAccount,
            Ss58AddressFormatRegistry::PendulumAccount => ss58::Ss58AddressFormatRegistry::PendulumAccount,
            Ss58AddressFormatRegistry::PhalaAccount => ss58::Ss58AddressFormatRegistry::PhalaAccount,
            Ss58AddressFormatRegistry::PicassoAccount => ss58::Ss58AddressFormatRegistry::PicassoAccount,
            Ss58AddressFormatRegistry::PioneerNetworkAccount => ss58::Ss58AddressFormatRegistry::PioneerNetworkAccount,
            Ss58AddressFormatRegistry::PolimecAccount => ss58::Ss58AddressFormatRegistry::PolimecAccount,
            Ss58AddressFormatRegistry::PolkadexAccount => ss58::Ss58AddressFormatRegistry::PolkadexAccount,
            Ss58AddressFormatRegistry::PolkadexparachainAccount => ss58::Ss58AddressFormatRegistry::PolkadexparachainAccount,
            Ss58AddressFormatRegistry::PolkadotAccount => ss58::Ss58AddressFormatRegistry::PolkadotAccount,
            Ss58AddressFormatRegistry::PolkafoundryAccount => ss58::Ss58AddressFormatRegistry::PolkafoundryAccount,
            Ss58AddressFormatRegistry::PolkasmithAccount => ss58::Ss58AddressFormatRegistry::PolkasmithAccount,
            Ss58AddressFormatRegistry::PolymeshAccount => ss58::Ss58AddressFormatRegistry::PolymeshAccount,
            Ss58AddressFormatRegistry::PontemNetworkAccount => ss58::Ss58AddressFormatRegistry::PontemNetworkAccount,
            Ss58AddressFormatRegistry::QuartzMainnetAccount => ss58::Ss58AddressFormatRegistry::QuartzMainnetAccount,
            Ss58AddressFormatRegistry::Reserved46Account => ss58::Ss58AddressFormatRegistry::Reserved46Account,
            Ss58AddressFormatRegistry::Reserved47Account => ss58::Ss58AddressFormatRegistry::Reserved47Account,
            Ss58AddressFormatRegistry::ReynoldsAccount => ss58::Ss58AddressFormatRegistry::ReynoldsAccount,
            Ss58AddressFormatRegistry::RobonomicsAccount => ss58::Ss58AddressFormatRegistry::RobonomicsAccount,
            Ss58AddressFormatRegistry::SapphireMainnetAccount => ss58::Ss58AddressFormatRegistry::SapphireMainnetAccount,
            Ss58AddressFormatRegistry::SealsAccount => ss58::Ss58AddressFormatRegistry::SealsAccount,
            Ss58AddressFormatRegistry::ShiftAccount => ss58::Ss58AddressFormatRegistry::ShiftAccount,
            Ss58AddressFormatRegistry::SocialNetworkAccount => ss58::Ss58AddressFormatRegistry::SocialNetworkAccount,
            Ss58AddressFormatRegistry::SocietalAccount => ss58::Ss58AddressFormatRegistry::SocietalAccount,
            Ss58AddressFormatRegistry::SoraAccount => ss58::Ss58AddressFormatRegistry::SoraAccount,
            Ss58AddressFormatRegistry::SoraDotParaAccount => ss58::Ss58AddressFormatRegistry::SoraDotParaAccount,
            Ss58AddressFormatRegistry::SoraKusamaParaAccount => ss58::Ss58AddressFormatRegistry::SoraKusamaParaAccount,
            Ss58AddressFormatRegistry::StafiAccount => ss58::Ss58AddressFormatRegistry::StafiAccount,
            Ss58AddressFormatRegistry::SubsocialAccount => ss58::Ss58AddressFormatRegistry::SubsocialAccount,
            Ss58AddressFormatRegistry::SubspaceTestnetAccount => ss58::Ss58AddressFormatRegistry::SubspaceTestnetAccount,
            Ss58AddressFormatRegistry::SubstrateAccount => ss58::Ss58AddressFormatRegistry::SubstrateAccount,
            Ss58AddressFormatRegistry::SynesthesiaAccount => ss58::Ss58AddressFormatRegistry::SynesthesiaAccount,
            Ss58AddressFormatRegistry::T3RnAccount => ss58::Ss58AddressFormatRegistry::T3RnAccount,
            Ss58AddressFormatRegistry::TangleAccount => ss58::Ss58AddressFormatRegistry::TangleAccount,
            Ss58AddressFormatRegistry::TernoaAccount => ss58::Ss58AddressFormatRegistry::TernoaAccount,
            Ss58AddressFormatRegistry::TidefiAccount => ss58::Ss58AddressFormatRegistry::TidefiAccount,
            Ss58AddressFormatRegistry::TinkerAccount => ss58::Ss58AddressFormatRegistry::TinkerAccount,
            Ss58AddressFormatRegistry::TotemAccount => ss58::Ss58AddressFormatRegistry::TotemAccount,
            Ss58AddressFormatRegistry::UniartsAccount => ss58::Ss58AddressFormatRegistry::UniartsAccount,
            Ss58AddressFormatRegistry::UniqueMainnetAccount => ss58::Ss58AddressFormatRegistry::UniqueMainnetAccount,
            Ss58AddressFormatRegistry::VaraAccount => ss58::Ss58AddressFormatRegistry::VaraAccount,
            Ss58AddressFormatRegistry::VlnAccount => ss58::Ss58AddressFormatRegistry::VlnAccount,
            Ss58AddressFormatRegistry::VowChainAccount => ss58::Ss58AddressFormatRegistry::VowChainAccount,
            Ss58AddressFormatRegistry::WatrAccount => ss58::Ss58AddressFormatRegistry::WatrAccount,
            Ss58AddressFormatRegistry::XcavateAccount => ss58::Ss58AddressFormatRegistry::XcavateAccount,
            Ss58AddressFormatRegistry::XxnetworkAccount => ss58::Ss58AddressFormatRegistry::XxnetworkAccount,
            Ss58AddressFormatRegistry::ZeitgeistAccount => ss58::Ss58AddressFormatRegistry::ZeitgeistAccount,
            Ss58AddressFormatRegistry::ZeroAccount => ss58::Ss58AddressFormatRegistry::ZeroAccount,
            Ss58AddressFormatRegistry::ZeroAlphavilleAccount => ss58::Ss58AddressFormatRegistry::ZeroAlphavilleAccount,
        }
    }
}

impl From<ss58::Ss58AddressFormatRegistry> for Ss58AddressFormatRegistry {
    #[rustfmt::skip]
    fn from(value: ss58::Ss58AddressFormatRegistry) -> Self {
        match value {
            ss58::Ss58AddressFormatRegistry::BareEd25519Account => Ss58AddressFormatRegistry::BareEd25519Account,
            ss58::Ss58AddressFormatRegistry::BareSecp256K1Account => Ss58AddressFormatRegistry::BareSecp256K1Account,
            ss58::Ss58AddressFormatRegistry::BareSr25519Account => Ss58AddressFormatRegistry::BareSr25519Account,
            ss58::Ss58AddressFormatRegistry::DicoAccount => Ss58AddressFormatRegistry::DicoAccount,
            ss58::Ss58AddressFormatRegistry::IceAccount => Ss58AddressFormatRegistry::IceAccount,
            ss58::Ss58AddressFormatRegistry::KicoAccount => Ss58AddressFormatRegistry::KicoAccount,
            ss58::Ss58AddressFormatRegistry::SnowAccount => Ss58AddressFormatRegistry::SnowAccount,
            ss58::Ss58AddressFormatRegistry::AcalaAccount => Ss58AddressFormatRegistry::AcalaAccount,
            ss58::Ss58AddressFormatRegistry::AjunaAccount => Ss58AddressFormatRegistry::AjunaAccount,
            ss58::Ss58AddressFormatRegistry::AllfeatNetworkAccount => Ss58AddressFormatRegistry::AllfeatNetworkAccount,
            ss58::Ss58AddressFormatRegistry::AltairAccount => Ss58AddressFormatRegistry::AltairAccount,
            ss58::Ss58AddressFormatRegistry::AmplitudeAccount => Ss58AddressFormatRegistry::AmplitudeAccount,
            ss58::Ss58AddressFormatRegistry::AnalogTimechainAccount => Ss58AddressFormatRegistry::AnalogTimechainAccount,
            ss58::Ss58AddressFormatRegistry::AnmolAccount => Ss58AddressFormatRegistry::AnmolAccount,
            ss58::Ss58AddressFormatRegistry::AresAccount => Ss58AddressFormatRegistry::AresAccount,
            ss58::Ss58AddressFormatRegistry::AstarAccount => Ss58AddressFormatRegistry::AstarAccount,
            ss58::Ss58AddressFormatRegistry::AutonomysAccount => Ss58AddressFormatRegistry::AutonomysAccount,
            ss58::Ss58AddressFormatRegistry::AventusAccount => Ss58AddressFormatRegistry::AventusAccount,
            ss58::Ss58AddressFormatRegistry::BajunAccount => Ss58AddressFormatRegistry::BajunAccount,
            ss58::Ss58AddressFormatRegistry::BasiliskAccount => Ss58AddressFormatRegistry::BasiliskAccount,
            ss58::Ss58AddressFormatRegistry::BifrostAccount => Ss58AddressFormatRegistry::BifrostAccount,
            ss58::Ss58AddressFormatRegistry::BitgreenAccount => Ss58AddressFormatRegistry::BitgreenAccount,
            ss58::Ss58AddressFormatRegistry::BittensorAccount => Ss58AddressFormatRegistry::BittensorAccount,
            ss58::Ss58AddressFormatRegistry::CalamariAccount => Ss58AddressFormatRegistry::CalamariAccount,
            ss58::Ss58AddressFormatRegistry::CentrifugeAccount => Ss58AddressFormatRegistry::CentrifugeAccount,
            ss58::Ss58AddressFormatRegistry::CereAccount => Ss58AddressFormatRegistry::CereAccount,
            ss58::Ss58AddressFormatRegistry::CessAccount => Ss58AddressFormatRegistry::CessAccount,
            ss58::Ss58AddressFormatRegistry::CessTestnetAccount => Ss58AddressFormatRegistry::CessTestnetAccount,
            ss58::Ss58AddressFormatRegistry::ChainflipAccount => Ss58AddressFormatRegistry::ChainflipAccount,
            ss58::Ss58AddressFormatRegistry::ChainxAccount => Ss58AddressFormatRegistry::ChainxAccount,
            ss58::Ss58AddressFormatRegistry::CloudwalkMainnetAccount => Ss58AddressFormatRegistry::CloudwalkMainnetAccount,
            ss58::Ss58AddressFormatRegistry::CloverAccount => Ss58AddressFormatRegistry::CloverAccount,
            ss58::Ss58AddressFormatRegistry::ComposableAccount => Ss58AddressFormatRegistry::ComposableAccount,
            ss58::Ss58AddressFormatRegistry::ContextfreeAccount => Ss58AddressFormatRegistry::ContextfreeAccount,
            ss58::Ss58AddressFormatRegistry::CordAccount => Ss58AddressFormatRegistry::CordAccount,
            ss58::Ss58AddressFormatRegistry::CrustAccount => Ss58AddressFormatRegistry::CrustAccount,
            ss58::Ss58AddressFormatRegistry::CurioAccount => Ss58AddressFormatRegistry::CurioAccount,
            ss58::Ss58AddressFormatRegistry::DarkAccount => Ss58AddressFormatRegistry::DarkAccount,
            ss58::Ss58AddressFormatRegistry::DarwiniaAccount => Ss58AddressFormatRegistry::DarwiniaAccount,
            ss58::Ss58AddressFormatRegistry::DatahighwayAccount => Ss58AddressFormatRegistry::DatahighwayAccount,
            ss58::Ss58AddressFormatRegistry::DentnetAccount => Ss58AddressFormatRegistry::DentnetAccount,
            ss58::Ss58AddressFormatRegistry::DockPosMainnetAccount => Ss58AddressFormatRegistry::DockPosMainnetAccount,
            ss58::Ss58AddressFormatRegistry::DorafactoryPolkadotAccount => Ss58AddressFormatRegistry::DorafactoryPolkadotAccount,
            ss58::Ss58AddressFormatRegistry::EdgewareAccount => Ss58AddressFormatRegistry::EdgewareAccount,
            ss58::Ss58AddressFormatRegistry::EfinityAccount => Ss58AddressFormatRegistry::EfinityAccount,
            ss58::Ss58AddressFormatRegistry::EquilibriumAccount => Ss58AddressFormatRegistry::EquilibriumAccount,
            ss58::Ss58AddressFormatRegistry::EternalCivilizationAccount => Ss58AddressFormatRegistry::EternalCivilizationAccount,
            ss58::Ss58AddressFormatRegistry::FragnovaAccount => Ss58AddressFormatRegistry::FragnovaAccount,
            ss58::Ss58AddressFormatRegistry::FrequencyAccount => Ss58AddressFormatRegistry::FrequencyAccount,
            ss58::Ss58AddressFormatRegistry::G1Account => Ss58AddressFormatRegistry::G1Account,
            ss58::Ss58AddressFormatRegistry::GeekAccount => Ss58AddressFormatRegistry::GeekAccount,
            ss58::Ss58AddressFormatRegistry::GenshiroAccount => Ss58AddressFormatRegistry::GenshiroAccount,
            ss58::Ss58AddressFormatRegistry::GmAccount => Ss58AddressFormatRegistry::GmAccount,
            ss58::Ss58AddressFormatRegistry::GoldenGateAccount => Ss58AddressFormatRegistry::GoldenGateAccount,
            ss58::Ss58AddressFormatRegistry::GoldenGateSydneyAccount => Ss58AddressFormatRegistry::GoldenGateSydneyAccount,
            ss58::Ss58AddressFormatRegistry::GoroAccount => Ss58AddressFormatRegistry::GoroAccount,
            ss58::Ss58AddressFormatRegistry::HashedAccount => Ss58AddressFormatRegistry::HashedAccount,
            ss58::Ss58AddressFormatRegistry::HeikoAccount => Ss58AddressFormatRegistry::HeikoAccount,
            ss58::Ss58AddressFormatRegistry::HumanodeAccount => Ss58AddressFormatRegistry::HumanodeAccount,
            ss58::Ss58AddressFormatRegistry::HydradxAccount => Ss58AddressFormatRegistry::HydradxAccount,
            ss58::Ss58AddressFormatRegistry::IbtidaAccount => Ss58AddressFormatRegistry::IbtidaAccount,
            ss58::Ss58AddressFormatRegistry::ImpactAccount => Ss58AddressFormatRegistry::ImpactAccount,
            ss58::Ss58AddressFormatRegistry::IntegriteeAccount => Ss58AddressFormatRegistry::IntegriteeAccount,
            ss58::Ss58AddressFormatRegistry::IntegriteeIncognitoAccount => Ss58AddressFormatRegistry::IntegriteeIncognitoAccount,
            ss58::Ss58AddressFormatRegistry::InterlayAccount => Ss58AddressFormatRegistry::InterlayAccount,
            ss58::Ss58AddressFormatRegistry::JoystreamAccount => Ss58AddressFormatRegistry::JoystreamAccount,
            ss58::Ss58AddressFormatRegistry::JupiterAccount => Ss58AddressFormatRegistry::JupiterAccount,
            ss58::Ss58AddressFormatRegistry::KabochaAccount => Ss58AddressFormatRegistry::KabochaAccount,
            ss58::Ss58AddressFormatRegistry::KapexAccount => Ss58AddressFormatRegistry::KapexAccount,
            ss58::Ss58AddressFormatRegistry::KarmachainAccount => Ss58AddressFormatRegistry::KarmachainAccount,
            ss58::Ss58AddressFormatRegistry::KaruraAccount => Ss58AddressFormatRegistry::KaruraAccount,
            ss58::Ss58AddressFormatRegistry::KatalchainAccount => Ss58AddressFormatRegistry::KatalchainAccount,
            ss58::Ss58AddressFormatRegistry::KiltAccount => Ss58AddressFormatRegistry::KiltAccount,
            ss58::Ss58AddressFormatRegistry::KintsugiAccount => Ss58AddressFormatRegistry::KintsugiAccount,
            ss58::Ss58AddressFormatRegistry::KrestAccount => Ss58AddressFormatRegistry::KrestAccount,
            ss58::Ss58AddressFormatRegistry::KriganAccount => Ss58AddressFormatRegistry::KriganAccount,
            ss58::Ss58AddressFormatRegistry::KulupuAccount => Ss58AddressFormatRegistry::KulupuAccount,
            ss58::Ss58AddressFormatRegistry::KusamaAccount => Ss58AddressFormatRegistry::KusamaAccount,
            ss58::Ss58AddressFormatRegistry::LaminarAccount => Ss58AddressFormatRegistry::LaminarAccount,
            ss58::Ss58AddressFormatRegistry::LitentryAccount => Ss58AddressFormatRegistry::LitentryAccount,
            ss58::Ss58AddressFormatRegistry::LitmusAccount => Ss58AddressFormatRegistry::LitmusAccount,
            ss58::Ss58AddressFormatRegistry::LogionAccount => Ss58AddressFormatRegistry::LogionAccount,
            ss58::Ss58AddressFormatRegistry::LuhnAccount => Ss58AddressFormatRegistry::LuhnAccount,
            ss58::Ss58AddressFormatRegistry::MantaAccount => Ss58AddressFormatRegistry::MantaAccount,
            ss58::Ss58AddressFormatRegistry::MathchainAccount => Ss58AddressFormatRegistry::MathchainAccount,
            ss58::Ss58AddressFormatRegistry::MathchainTestnetAccount => Ss58AddressFormatRegistry::MathchainTestnetAccount,
            ss58::Ss58AddressFormatRegistry::MetaquityNetworkAccount => Ss58AddressFormatRegistry::MetaquityNetworkAccount,
            ss58::Ss58AddressFormatRegistry::MoonbeamAccount => Ss58AddressFormatRegistry::MoonbeamAccount,
            ss58::Ss58AddressFormatRegistry::MoonriverAccount => Ss58AddressFormatRegistry::MoonriverAccount,
            ss58::Ss58AddressFormatRegistry::MoonsamaAccount => Ss58AddressFormatRegistry::MoonsamaAccount,
            ss58::Ss58AddressFormatRegistry::MosaicChainAccount => Ss58AddressFormatRegistry::MosaicChainAccount,
            ss58::Ss58AddressFormatRegistry::MythosAccount => Ss58AddressFormatRegistry::MythosAccount,
            ss58::Ss58AddressFormatRegistry::NeatcoinAccount => Ss58AddressFormatRegistry::NeatcoinAccount,
            ss58::Ss58AddressFormatRegistry::NftmartAccount => Ss58AddressFormatRegistry::NftmartAccount,
            ss58::Ss58AddressFormatRegistry::NodleAccount => Ss58AddressFormatRegistry::NodleAccount,
            ss58::Ss58AddressFormatRegistry::OakAccount => Ss58AddressFormatRegistry::OakAccount,
            ss58::Ss58AddressFormatRegistry::OrigintrailParachainAccount => Ss58AddressFormatRegistry::OrigintrailParachainAccount,
            ss58::Ss58AddressFormatRegistry::P3DAccount => Ss58AddressFormatRegistry::P3DAccount,
            ss58::Ss58AddressFormatRegistry::P3DtAccount => Ss58AddressFormatRegistry::P3DtAccount,
            ss58::Ss58AddressFormatRegistry::ParallelAccount => Ss58AddressFormatRegistry::ParallelAccount,
            ss58::Ss58AddressFormatRegistry::PeaqAccount => Ss58AddressFormatRegistry::PeaqAccount,
            ss58::Ss58AddressFormatRegistry::PeerplaysAccount => Ss58AddressFormatRegistry::PeerplaysAccount,
            ss58::Ss58AddressFormatRegistry::PendulumAccount => Ss58AddressFormatRegistry::PendulumAccount,
            ss58::Ss58AddressFormatRegistry::PhalaAccount => Ss58AddressFormatRegistry::PhalaAccount,
            ss58::Ss58AddressFormatRegistry::PicassoAccount => Ss58AddressFormatRegistry::PicassoAccount,
            ss58::Ss58AddressFormatRegistry::PioneerNetworkAccount => Ss58AddressFormatRegistry::PioneerNetworkAccount,
            ss58::Ss58AddressFormatRegistry::PolimecAccount => Ss58AddressFormatRegistry::PolimecAccount,
            ss58::Ss58AddressFormatRegistry::PolkadexAccount => Ss58AddressFormatRegistry::PolkadexAccount,
            ss58::Ss58AddressFormatRegistry::PolkadexparachainAccount => Ss58AddressFormatRegistry::PolkadexparachainAccount,
            ss58::Ss58AddressFormatRegistry::PolkadotAccount => Ss58AddressFormatRegistry::PolkadotAccount,
            ss58::Ss58AddressFormatRegistry::PolkafoundryAccount => Ss58AddressFormatRegistry::PolkafoundryAccount,
            ss58::Ss58AddressFormatRegistry::PolkasmithAccount => Ss58AddressFormatRegistry::PolkasmithAccount,
            ss58::Ss58AddressFormatRegistry::PolymeshAccount => Ss58AddressFormatRegistry::PolymeshAccount,
            ss58::Ss58AddressFormatRegistry::PontemNetworkAccount => Ss58AddressFormatRegistry::PontemNetworkAccount,
            ss58::Ss58AddressFormatRegistry::QuartzMainnetAccount => Ss58AddressFormatRegistry::QuartzMainnetAccount,
            ss58::Ss58AddressFormatRegistry::Reserved46Account => Ss58AddressFormatRegistry::Reserved46Account,
            ss58::Ss58AddressFormatRegistry::Reserved47Account => Ss58AddressFormatRegistry::Reserved47Account,
            ss58::Ss58AddressFormatRegistry::ReynoldsAccount => Ss58AddressFormatRegistry::ReynoldsAccount,
            ss58::Ss58AddressFormatRegistry::RobonomicsAccount => Ss58AddressFormatRegistry::RobonomicsAccount,
            ss58::Ss58AddressFormatRegistry::SapphireMainnetAccount => Ss58AddressFormatRegistry::SapphireMainnetAccount,
            ss58::Ss58AddressFormatRegistry::SealsAccount => Ss58AddressFormatRegistry::SealsAccount,
            ss58::Ss58AddressFormatRegistry::ShiftAccount => Ss58AddressFormatRegistry::ShiftAccount,
            ss58::Ss58AddressFormatRegistry::SocialNetworkAccount => Ss58AddressFormatRegistry::SocialNetworkAccount,
            ss58::Ss58AddressFormatRegistry::SocietalAccount => Ss58AddressFormatRegistry::SocietalAccount,
            ss58::Ss58AddressFormatRegistry::SoraAccount => Ss58AddressFormatRegistry::SoraAccount,
            ss58::Ss58AddressFormatRegistry::SoraDotParaAccount => Ss58AddressFormatRegistry::SoraDotParaAccount,
            ss58::Ss58AddressFormatRegistry::SoraKusamaParaAccount => Ss58AddressFormatRegistry::SoraKusamaParaAccount,
            ss58::Ss58AddressFormatRegistry::StafiAccount => Ss58AddressFormatRegistry::StafiAccount,
            ss58::Ss58AddressFormatRegistry::SubsocialAccount => Ss58AddressFormatRegistry::SubsocialAccount,
            ss58::Ss58AddressFormatRegistry::SubspaceTestnetAccount => Ss58AddressFormatRegistry::SubspaceTestnetAccount,
            ss58::Ss58AddressFormatRegistry::SubstrateAccount => Ss58AddressFormatRegistry::SubstrateAccount,
            ss58::Ss58AddressFormatRegistry::SynesthesiaAccount => Ss58AddressFormatRegistry::SynesthesiaAccount,
            ss58::Ss58AddressFormatRegistry::T3RnAccount => Ss58AddressFormatRegistry::T3RnAccount,
            ss58::Ss58AddressFormatRegistry::TangleAccount => Ss58AddressFormatRegistry::TangleAccount,
            ss58::Ss58AddressFormatRegistry::TernoaAccount => Ss58AddressFormatRegistry::TernoaAccount,
            ss58::Ss58AddressFormatRegistry::TidefiAccount => Ss58AddressFormatRegistry::TidefiAccount,
            ss58::Ss58AddressFormatRegistry::TinkerAccount => Ss58AddressFormatRegistry::TinkerAccount,
            ss58::Ss58AddressFormatRegistry::TotemAccount => Ss58AddressFormatRegistry::TotemAccount,
            ss58::Ss58AddressFormatRegistry::UniartsAccount => Ss58AddressFormatRegistry::UniartsAccount,
            ss58::Ss58AddressFormatRegistry::UniqueMainnetAccount => Ss58AddressFormatRegistry::UniqueMainnetAccount,
            ss58::Ss58AddressFormatRegistry::VaraAccount => Ss58AddressFormatRegistry::VaraAccount,
            ss58::Ss58AddressFormatRegistry::VlnAccount => Ss58AddressFormatRegistry::VlnAccount,
            ss58::Ss58AddressFormatRegistry::VowChainAccount => Ss58AddressFormatRegistry::VowChainAccount,
            ss58::Ss58AddressFormatRegistry::WatrAccount => Ss58AddressFormatRegistry::WatrAccount,
            ss58::Ss58AddressFormatRegistry::XcavateAccount => Ss58AddressFormatRegistry::XcavateAccount,
            ss58::Ss58AddressFormatRegistry::XxnetworkAccount => Ss58AddressFormatRegistry::XxnetworkAccount,
            ss58::Ss58AddressFormatRegistry::ZeitgeistAccount => Ss58AddressFormatRegistry::ZeitgeistAccount,
            ss58::Ss58AddressFormatRegistry::ZeroAccount => Ss58AddressFormatRegistry::ZeroAccount,
            ss58::Ss58AddressFormatRegistry::ZeroAlphavilleAccount => Ss58AddressFormatRegistry::ZeroAlphavilleAccount,
            _ => unreachable!()
        }
    }
}

#[pymethods]
impl Ss58AddressFormatRegistry {
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

impl std::fmt::Display for Ss58AddressFormatRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ss58::Ss58AddressFormatRegistry::from(*self).fmt(f)
    }
}
