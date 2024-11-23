import chainql

print("TEST LATEST, ACCESS TO some field")

print(chainql.Chain("ws://localhost:9699/relay-assethubA/").latest()['System']['Account']['0x1cea013c4584f20c4cdce347688b9cef6d85938c464867ee5309b9bbdeb3844b']['consumers'])

print("TEST BLOCK, DO THE SAME")

print(chainql.Chain("ws://localhost:9699/relay-assethubA/").block(300)['System']['Account']['0x1cea013c4584f20c4cdce347688b9cef6d85938c464867ee5309b9bbdeb3844b']['consumers'])