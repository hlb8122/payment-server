import bitcoin
import requests
from s2s_pb2 import *
from paymentrequest_pb2 import *
from time import time
from bitcoinrpc.authproxy import AuthServiceProxy, JSONRPCException
from bitcoin.core.key import CECKey
from bitcoin.wallet import P2PKHBitcoinAddress
from bitcoin.core import CMutableTransaction, CMutableTxIn
from decimal import Decimal

BASE_URL = "http://0.0.0.0:8080"
bitcoin.SelectParams("regtest")

# Init Bitcoin RPC
rpc_user = "username"
rpc_password = "password"
rpc_connection = AuthServiceProxy(
    "http://%s:%s@127.0.0.1:18443" % (rpc_user, rpc_password))

time = int(time())
invoice_params = InvoiceRequest(network="regnet", amount=5, time=time, expires=time + 10, tokenize=True,
                                ack_memo="Thanks for your custom!", merchant_data=b"http://localhost:1234/keys/example-key")
raw_invoice_params = invoice_params.SerializeToString()

# Get payment request
print("Sending invoice request...")
response = requests.post(
    "http://127.0.0.1:8081/invoice", data=raw_invoice_params)
invoice_response = InvoiceResponse.FromString(response.content)
payment_request = invoice_response.payment_request
print("Received PaymentRequest:")
print(payment_request)
input("Send payment?")

# Deserialize invoice
payment_details_raw = payment_request.serialized_payment_details
payment_details = PaymentDetails.FromString(payment_details_raw)

# Payment amount
price = Decimal(payment_details.outputs[0].amount) / 1_00_000_000

# Collect inputs
fee = Decimal(5) / 10_000_000
utxos = rpc_connection.listunspent()
inputs = []
input_value = Decimal(0)
for utxo in utxos:
    if input_value < price + fee:
        inputs.append({
            "txid": utxo["txid"],
            "vout": utxo["vout"]
        })
        input_value += utxo["amount"]
    else:
        break

# Create outputs
my_addr = utxo["address"]
change = input_value - price - fee
p2pkh = payment_details.outputs[0].script
# op_return = payment_details.outputs[1].script[2:].hex()
payment_addr = str(P2PKHBitcoinAddress.from_scriptPubKey(p2pkh))
outputs = [
    {
        payment_addr: price  # Payment output
    },
    # {
    #     "data": op_return
    # },
    {
        my_addr: change  # Change output
    }
]

# Create tx
raw_tx_unsigned = rpc_connection.createrawtransaction(inputs, outputs)
signed_raw_tx = bytes.fromhex(
    rpc_connection.signrawtransactionwithwallet(raw_tx_unsigned)["hex"])

# Construct payment message
payment = Payment(merchant_data=payment_details.merchant_data,
                  transactions=[signed_raw_tx])
payment_raw = payment.SerializeToString()

# Send payment
payment_url = payment_details.payment_url
headers = {
    "Content-Type": "application/bitcoincash-payment",
    "Accept": "application/bitcoincash-paymentack"
}
response = requests.post(url=payment_url, data=payment_raw,
                         headers=headers, allow_redirects=False)
payment_ack = PaymentACK.FromString(response.content)
print("Received PaymentAck:")
print(payment_ack)
print("Redirect URL:")
print(response.headers["Location"])
print("Bearer Auth:")
print(response.headers["authorization"])
