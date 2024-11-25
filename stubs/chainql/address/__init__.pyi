# Copyright 2024 Valery Klachkov
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import enum
from typing import Optional, Self

class Ss58AddressFormat:
    def __init__(self, format) -> None:
        ...
    
    @staticmethod
    def from_name(name: str) -> Self:
        ...
    
    @staticmethod
    def custom(prefix: int) -> Self:
        ...
    
    def prefix(self) -> int:
        """Address prefix used on the network"""
        ...

    def is_reserved(self) -> bool:
        """Network/AddressType is reserved for future use"""
        ...

    def is_custom(self) -> bool:
        """A custom format is one that is not already known"""
        ...

class Ss58AccountFormat(enum.Enum):
    """A known address (sub)format/network ID for SS58"""
    
    BareEd25519 = 3,
    """Bare 32-bit Ed25519 public key."""
    
    BareSecp256K1 = 43,
    """Bare 32-bit ECDSA SECP-256k1 public key."""
    
    BareSr25519 = 1,
    """Bare 32-bit Schnorr/Ristretto (S/R 25519) public key."""
    
    Dico = 53,
    """DICO - <https://dico.io>"""
    
    Ice = 2206,
    """ICE Network - <https://icenetwork.io>"""
    
    Kico = 52,
    """KICO - <https://dico.io>"""
    
    Snow = 2207,
    """SNOW: ICE Canary Network - <https://icenetwork.io>"""
    
    Acala = 10,
    """Acala - <https://acala.network/>"""
    
    Ajuna = 1328,
    """Ajuna Network - <https://ajuna.io>"""
    
    AllfeatNetwork = 440,
    """Allfeat Network - <https://allfeat.network>"""
    
    Altair = 136,
    """Altair - <https://centrifuge.io/>"""
    
    Amplitude = 57,
    """Amplitude chain - <https://pendulumchain.org/>"""
    
    AnalogTimechain = 12850,
    """Analog Timechain - <https://analog.one>"""
    
    Anmol = 92,
    """Anmol Network - <https://anmol.network/>"""
    
    Ares = 34,
    """Ares Protocol - <https://www.aresprotocol.com/>"""
    
    Astar = 5,
    """Astar Network - <https://astar.network>"""
    
    Autonomys = 6094,
    """Autonomys - <https://autonomys.xyz>"""
    
    Aventus = 65,
    """Aventus Mainnet - <https://aventus.io>"""
    
    Bajun = 1337,
    """Bajun Network - <https://ajuna.io>"""
    
    Basilisk = 10041,
    """Basilisk - <https://bsx.fi>"""
    
    Bifrost = 6,
    """Bifrost - <https://bifrost.finance/>"""
    
    Bitgreen = 2106,
    """Bitgreen - <https://bitgreen.org/>"""
    
    Bittensor = 13116,
    """Bittensor - <https://bittensor.com>"""
    
    Calamari = 78,
    """Calamari: Manta Canary Network - <https://manta.network>"""
    
    Centrifuge = 36,
    """Centrifuge Chain - <https://centrifuge.io/>"""
    
    Cere = 54,
    """Cere Network - <https://cere.network>"""
    
    Cess = 11331,
    """CESS - <https://cess.cloud>"""
    
    CessTestnet = 11330,
    """CESS Testnet - <https://cess.cloud>"""
    
    Chainflip = 2112,
    """Chainflip - <https://chainflip.io/>"""
    
    Chainx = 44,
    """ChainX - <https://chainx.org/>"""
    
    CloudwalkMainnet = 2009,
    """CloudWalk Network Mainnet - <https://explorer.mainnet.cloudwalk.io>"""
    
    Clover = 128,
    """Clover Finance - <https://clover.finance>"""
    
    Composable = 50,
    """Composable Finance - <https://composable.finance>"""
    
    Contextfree = 11820,
    """Automata ContextFree - <https://ata.network>"""
    
    Cord = 29,
    """CORD Network - <https://cord.network/>"""
    
    Crust = 66,
    """Crust Network - <https://crust.network>"""
    
    Curio = 777,
    """Curio - <https://parachain.capitaldex.exchange/>"""
    
    Dark = 17,
    """Dark Mainnet"""
    
    Darwinia = 18,
    """Darwinia Network - <https://darwinia.network>"""
    
    Datahighway = 33,
    """DataHighway"""
    
    Dentnet = 9807,
    """DENTNet - <https://www.dentnet.io>"""
    
    DockPosMainnet = 22,
    """Dock Mainnet - <https://dock.io>"""
    
    DorafactoryPolkadot = 129,
    """Dorafactory Polkadot Network - <https://dorafactory.org>"""
    
    Edgeware = 7,
    """Edgeware - <https://edgewa.re>"""
    
    Efinity = 1110,
    """Efinity - <https://efinity.io/>"""
    
    Equilibrium = 68,
    """Equilibrium Network - <https://equilibrium.io>"""
    
    EternalCivilization = 58,
    """Eternal Civilization - <http://www.ysknfr.cn/>"""
    
    Fragnova = 93,
    """Fragnova Network - <https://fragnova.com>"""
    
    Frequency = 90,
    """Frequency - <https://www.frequency.xyz>"""
    
    G1 = 4450,
    """Ğ1 - <https://duniter.org>"""
    
    Geek = 789,
    """GEEK Network - <https://geek.gl>"""
    
    Genshiro = 67,
    """Genshiro Network - <https://genshiro.equilibrium.io>"""
    
    Gm = 7013,
    """GM - <https://gmordie.com>"""
    
    GoldenGate = 8866,
    """Golden Gate - <https://ggxchain.io/>"""
    
    GoldenGateSydney = 8886,
    """Golden Gate Sydney - <https://ggxchain.io/>"""
    
    Goro = 14697,
    """GORO Network - <https://goro.network>"""
    
    Hashed = 9072,
    """Hashed Network - <https://hashed.network>"""
    
    Heiko = 110,
    """Heiko - <https://parallel.fi/>"""
    
    Humanode = 5234,
    """Humanode Network - <https://humanode.io>"""
    
    Hydradx = 63,
    """Hydration - <https://hydration.net>"""
    
    Ibtida = 100,
    """Anmol Network Ibtida Canary network - <https://anmol.network/>"""
    
    Impact = 12155,
    """Impact Protocol Network - <https://impactprotocol.network/>"""
    
    Integritee = 13,
    """Integritee - <https://integritee.network>"""
    
    IntegriteeIncognito = 113,
    """Integritee Incognito - <https://integritee.network>"""
    
    Interlay = 2032,
    """Interlay - <https://interlay.io/>"""
    
    Joystream = 126,
    """Joystream - <https://www.joystream.org>"""
    
    Jupiter = 26,
    """Jupiter - <https://jupiter.patract.io>"""
    
    Kabocha = 27,
    """Kabocha - <https://kabocha.network>"""
    
    Kapex = 2007,
    """Kapex - <https://totemaccounting.com>"""
    
    Karmachain = 21,
    """Karmacoin - <https://karmaco.in>"""
    
    Karura = 8,
    """Karura - <https://karura.network/>"""
    
    Katalchain = 4,
    """Katal Chain"""
    
    Kilt = 38,
    """KILT Spiritnet - <https://kilt.io/>"""
    
    Kintsugi = 2092,
    """Kintsugi - <https://interlay.io/>"""
    
    Krest = 1222,
    """Krest Network - <https://www.peaq.network/>"""
    
    Krigan = 7306,
    """Krigan Network - <https://krigan.network>"""
    
    Kulupu = 16,
    """Kulupu - <https://kulupu.network/>"""
    
    Kusama = 2,
    """Kusama Relay Chain - <https://kusama.network>"""
    
    Laminar = 11,
    """Laminar - <http://laminar.network/>"""
    
    Litentry = 31,
    """Litentry Network - <https://litentry.com/>"""
    
    Litmus = 131,
    """Litmus Network - <https://litentry.com/>"""
    
    Logion = 2021,
    """logion network - <https://logion.network>"""
    
    Luhn = 11486,
    """Luhn Network - <https://luhn.network>"""
    
    Manta = 77,
    """Manta network - <https://manta.network>"""
    
    Mathchain = 39,
    """MathChain mainnet - <https://mathwallet.org>"""
    
    MathchainTestnet = 40,
    """MathChain testnet - <https://mathwallet.org>"""
    
    MetaquityNetwork = 666,
    """Metaquity Network - <https://metaquity.xyz/>"""
    
    Moonbeam = 1284,
    """Moonbeam - <https://moonbeam.network>"""
    
    Moonriver = 1285,
    """Moonriver - <https://moonbeam.network>"""
    
    Moonsama = 2199,
    """Moonsama - <https://moonsama.com>"""
    
    MosaicChain = 14998,
    """Mosaic Chain - <https://mosaicchain.io>"""
    
    Mythos = 29972,
    """Mythos - <https://mythos.foundation>"""
    
    Neatcoin = 48,
    """Neatcoin Mainnet - <https://neatcoin.org>"""
    
    Nftmart = 12191,
    """NFTMart - <https://nftmart.io>"""
    
    Nodle = 37,
    """Nodle Chain - <https://nodle.io/>"""
    
    Oak = 51,
    """OAK Network - <https://oak.tech>"""
    
    OrigintrailParachain = 101,
    """OriginTrail Parachain - <https://parachain.origintrail.io/>"""
    
    P3D = 71,
    """3DP network - <https://3dpass.org>"""
    
    P3Dt = 72,
    """3DP test network - <https://3dpass.org>"""
    
    Parallel = 172,
    """Parallel - <https://parallel.fi/>"""
    
    Peaq = 1221,
    """Peaq Network - <https://www.peaq.network/>"""
    
    Peerplays = 3333,
    """Peerplays - <https://www.peerplays.com/>"""
    
    Pendulum = 56,
    """Pendulum chain - <https://pendulumchain.org/>"""
    
    Phala = 30,
    """Phala Network - <https://phala.network>"""
    
    Picasso = 49,
    """Picasso - <https://picasso.composable.finance>"""
    
    PioneerNetwork = 268,
    """Pioneer Network by Bit.Country - <https://bit.country>"""
    
    Polimec = 41,
    """Polimec Protocol - <https://www.polimec.org/>"""
    
    Polkadex = 88,
    """Polkadex Mainnet - <https://polkadex.trade>"""
    
    Polkadexparachain = 89,
    """Polkadex Parachain - <https://polkadex.trade>"""
    
    Polkadot = 0,
    """Polkadot Relay Chain - <https://polkadot.network>"""
    
    Polkafoundry = 99,
    """PolkaFoundry Network - <https://polkafoundry.com>"""
    
    Polkasmith = 98,
    """PolkaSmith Canary Network - <https://polkafoundry.com>"""
    
    Polymesh = 12,
    """Polymesh - <https://polymath.network/>"""
    
    PontemNetwork = 105,
    """Pontem Network - <https://pontem.network>"""
    
    QuartzMainnet = 255,
    """QUARTZ by UNIQUE - <https://unique.network>"""
    
    Reserved46 = 46,
    """This prefix is reserved."""
    
    Reserved47 = 47,
    """This prefix is reserved."""
    
    Reynolds = 9,
    """Laminar Reynolds Canary - <http://laminar.network/>"""
    
    Robonomics = 32,
    """Robonomics - <https://robonomics.network>"""
    
    SapphireMainnet = 8883,
    """Sapphire by Unique - <https://unique.network>"""
    
    Seals = 1985,
    """Seals Network - <https://seals.app>"""
    
    Shift = 23,
    """ShiftNrg"""
    
    SocialNetwork = 252,
    """Social Network - <https://social.network>"""
    
    Societal = 1516,
    """Societal - <https://www.sctl.xyz>"""
    
    Sora = 69,
    """SORA Network - <https://sora.org>"""
    
    SoraDotPara = 81,
    """SORA Polkadot Parachain - <https://sora.org>"""
    
    SoraKusamaPara = 420,
    """SORA Kusama Parachain - <https://sora.org>"""
    
    Stafi = 20,
    """Stafi - <https://stafi.io>"""
    
    Subsocial = 28,
    """Subsocial"""
    
    SubspaceTestnet = 2254,
    """Subspace testnet - <https://subspace.network>"""
    
    Substrate = 42,
    """Substrate - <https://substrate.io/>"""
    
    Synesthesia = 15,
    """Synesthesia - <https://synesthesia.network/>"""
    
    T3Rn = 9935,
    """t3rn - <https://t3rn.io/>"""
    
    Tangle = 5845,
    """Tangle Network - <https://www.tangle.tools/>"""
    
    Ternoa = 995,
    """Ternoa - <https://www.ternoa.network>"""
    
    Tidefi = 7007,
    """Tidefi - <https://tidefi.com>"""
    
    Tinker = 117,
    """Tinker - <https://invarch.network>"""
    
    Totem = 14,
    """Totem - <https://totemaccounting.com>"""
    
    Uniarts = 45,
    """UniArts Network - <https://uniarts.me>"""
    
    UniqueMainnet = 7391,
    """Unique Network - <https://unique.network>"""
    
    Vara = 137,
    """Vara Network - <https://vara.network/>"""
    
    Vln = 35,
    """Valiu Liquidity Network - <https://valiu.com/>"""
    
    VowChain = 2024,
    """Enigmatic Smile - <https://www.vow.foundation/>"""
    
    Watr = 19,
    """Watr Protocol - <https://www.watr.org>"""
    
    Xcavate = 8888,
    """Xcavate Protocol - <https://xcavate.io/>"""
    
    Xxnetwork = 55,
    """xx network - <https://xx.network>"""
    
    Zeitgeist = 73,
    """Zeitgeist - <https://zeitgeist.pm>"""
    
    Zero = 24,
    """ZERO - <https://zero.io>"""
    
    ZeroAlphaville = 25,
    """ZERO Alphaville - <https://zero.io>"""
    

    @staticmethod
    def from_format(format: Ss58AddressFormat) -> Self:
        ...
    
    @staticmethod
    def from_name(name: str) -> Self:
        ...

def ss58_encode(raw: bytes, format: Optional[Ss58AddressFormat]) -> str:
    """Encode bytes to SS58 string"""
    ...

def ss58_decode(ss58: str) -> bytes:
    """Parse SS58 address to bytes"""
    ...

class SignatureSchema(enum.Enum):
    Ed25519 = enum.auto()
    Sr25519 = enum.auto()
    Ecdsa = enum.auto()
    Ethereum = enum.auto()

def address_seed(scheme: SignatureSchema, suri: str, format: Optional[Ss58AddressFormat] = None) -> str:
    ...

def public_bytes_seed(scheme: SignatureSchema, suri: str) -> bytes:
    ...
