# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: s2s.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


import paymentrequest_pb2 as paymentrequest__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='s2s.proto',
  package='models',
  syntax='proto3',
  serialized_options=None,
  serialized_pb=_b('\n\ts2s.proto\x12\x06models\x1a\x14paymentrequest.proto\"\xc4\x01\n\x0eInvoiceRequest\x12\x0f\n\x07network\x18\x01 \x01(\t\x12\x0e\n\x06\x61mount\x18\x02 \x01(\x04\x12\x0c\n\x04time\x18\x03 \x01(\x04\x12\x0f\n\x07\x65xpires\x18\x04 \x01(\x04\x12\x10\n\x08req_memo\x18\x05 \x01(\t\x12\x15\n\rmerchant_data\x18\x06 \x01(\x0c\x12\x10\n\x08\x61\x63k_memo\x18\x07 \x01(\t\x12\x10\n\x08tokenize\x18\x08 \x01(\x08\x12\x0f\n\x07tx_data\x18\t \x01(\x0c\x12\x14\n\x0c\x63\x61llback_url\x18\n \x01(\t\"V\n\x0fInvoiceResponse\x12\x12\n\npayment_id\x18\x01 \x01(\t\x12/\n\x0fpayment_request\x18\x02 \x01(\x0b\x32\x16.models.PaymentRequest\"N\n\x0f\x43\x61llbackPayload\x12\x12\n\npayment_id\x18\x01 \x01(\t\x12\'\n\x0bpayment_ack\x18\x02 \x01(\x0b\x32\x12.models.PaymentACKb\x06proto3')
  ,
  dependencies=[paymentrequest__pb2.DESCRIPTOR,])




_INVOICEREQUEST = _descriptor.Descriptor(
  name='InvoiceRequest',
  full_name='models.InvoiceRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='network', full_name='models.InvoiceRequest.network', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='amount', full_name='models.InvoiceRequest.amount', index=1,
      number=2, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='time', full_name='models.InvoiceRequest.time', index=2,
      number=3, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='expires', full_name='models.InvoiceRequest.expires', index=3,
      number=4, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='req_memo', full_name='models.InvoiceRequest.req_memo', index=4,
      number=5, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='merchant_data', full_name='models.InvoiceRequest.merchant_data', index=5,
      number=6, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=_b(""),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='ack_memo', full_name='models.InvoiceRequest.ack_memo', index=6,
      number=7, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='tokenize', full_name='models.InvoiceRequest.tokenize', index=7,
      number=8, type=8, cpp_type=7, label=1,
      has_default_value=False, default_value=False,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='tx_data', full_name='models.InvoiceRequest.tx_data', index=8,
      number=9, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=_b(""),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='callback_url', full_name='models.InvoiceRequest.callback_url', index=9,
      number=10, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=44,
  serialized_end=240,
)


_INVOICERESPONSE = _descriptor.Descriptor(
  name='InvoiceResponse',
  full_name='models.InvoiceResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='payment_id', full_name='models.InvoiceResponse.payment_id', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='payment_request', full_name='models.InvoiceResponse.payment_request', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=242,
  serialized_end=328,
)


_CALLBACKPAYLOAD = _descriptor.Descriptor(
  name='CallbackPayload',
  full_name='models.CallbackPayload',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='payment_id', full_name='models.CallbackPayload.payment_id', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='payment_ack', full_name='models.CallbackPayload.payment_ack', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=330,
  serialized_end=408,
)

_INVOICERESPONSE.fields_by_name['payment_request'].message_type = paymentrequest__pb2._PAYMENTREQUEST
_CALLBACKPAYLOAD.fields_by_name['payment_ack'].message_type = paymentrequest__pb2._PAYMENTACK
DESCRIPTOR.message_types_by_name['InvoiceRequest'] = _INVOICEREQUEST
DESCRIPTOR.message_types_by_name['InvoiceResponse'] = _INVOICERESPONSE
DESCRIPTOR.message_types_by_name['CallbackPayload'] = _CALLBACKPAYLOAD
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

InvoiceRequest = _reflection.GeneratedProtocolMessageType('InvoiceRequest', (_message.Message,), dict(
  DESCRIPTOR = _INVOICEREQUEST,
  __module__ = 's2s_pb2'
  # @@protoc_insertion_point(class_scope:models.InvoiceRequest)
  ))
_sym_db.RegisterMessage(InvoiceRequest)

InvoiceResponse = _reflection.GeneratedProtocolMessageType('InvoiceResponse', (_message.Message,), dict(
  DESCRIPTOR = _INVOICERESPONSE,
  __module__ = 's2s_pb2'
  # @@protoc_insertion_point(class_scope:models.InvoiceResponse)
  ))
_sym_db.RegisterMessage(InvoiceResponse)

CallbackPayload = _reflection.GeneratedProtocolMessageType('CallbackPayload', (_message.Message,), dict(
  DESCRIPTOR = _CALLBACKPAYLOAD,
  __module__ = 's2s_pb2'
  # @@protoc_insertion_point(class_scope:models.CallbackPayload)
  ))
_sym_db.RegisterMessage(CallbackPayload)


# @@protoc_insertion_point(module_scope)
