import chainql

print("ENABLED LOGS")

chainql.enable_logs()
print(chainql.chain("ws://localhost:9699/relay-assethubA/")["latest"])

print("DISABLED LOGS")

chainql.disable_logs()
print(chainql.chain("ws://localhost:9699/relay-assethubA/")['latest']['System']['Account']['0x1cea013c4584f20c4cdce347688b9cef6d85938c464867ee5309b9bbdeb3844b']['consumers'])