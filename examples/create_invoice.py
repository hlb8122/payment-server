from s2s_pb2 import *
from paymentrequest_pb2 import *
import requests
from time import time

time = int(time())
invoice_params = InvoiceRequest(network="mainnet", amount=5, time=time, expires=time + 10, tokenize=True)
raw_invoice_params = invoice_params.SerializeToString()

response = requests.post("http://127.0.0.1:8081/invoice", data=raw_invoice_params)
print(response.content)
invoice_response = InvoiceResponse.FromString(response.content)
print(invoice_response)