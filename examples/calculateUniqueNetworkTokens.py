from functools import reduce
from pprint import pprint
import chainql
import operator
import sys

if len(sys.argv) != 2:
  print("Usage: unique.py [url]")
  sys.exit(1)


chainql.enable_logs()

state = chainql.Chain(url=sys.argv[1]).latest()

print("Loading...")

allCollections = list(state.Common.CollectionById._preloadKeys.values())
collectionsCount = len(allCollections) 

nftCollectionsCount = sum(c.mode == 'NFT' for c in allCollections)
rftCollectionsCount = sum(c.mode == 'ReFungible' for c in allCollections)
fungibleCollectionsCount = sum(c.mode != 'ReFungible' and c.mode != 'NFT' for c in allCollections)
nftMinted = reduce(operator.add, state.Nonfungible.TokensMinted._preloadKeys.values(), 0)
nftBurned = reduce(operator.add, state.Nonfungible.TokensBurnt._preloadKeys.values(), 0)
rftMinted = reduce(operator.add, state.Refungible.TokensMinted._preloadKeys.values(), 0)
rftBurned = reduce(operator.add, state.Refungible.TokensBurnt._preloadKeys.values(), 0)

print({
  'total': {
    'aliveCollections': collectionsCount,
    'createdCollections': state.Common.CreatedCollectionCount,
    'deletedCollections': state.Common.DestroyedCollectionCount,
  },
  'nonfungible': {
    'collections': nftCollectionsCount,
    'tokens': {
      'alive': nftMinted - nftBurned,
      'minted': nftMinted,
      'burned': nftBurned
    }    
  },
  'fungible': {
    'collections': fungibleCollectionsCount
  },
  'refungible': {
    'collections': rftCollectionsCount,
    'tokens': {
      'alive': rftMinted - rftBurned,
      'minted': rftMinted,
      'burned': rftBurned
    }
  }
})