# This code may look unusually verbose for Ruby (and it is), but
# it performs some subtle and complex validation of JSON data.
#
# To parse this JSON, add 'dry-struct' and 'dry-types' gems, then do:
#
#   mf_order_response = MFOrderResponse.from_json! "{…}"
#   puts mf_order_response.definitions.mf_order_response.required.first
#
# If from_json! succeeds, the value returned matches the schema.

require 'json'
require 'dry-types'
require 'dry-struct'

module Types
  include Dry::Types.module

  Bool   = Strict::Bool
  Hash   = Strict::Hash
  String = Strict::String
end

class OrderID < Dry::Struct
  attribute :order_id_format, Types::String
  attribute :order_id_type,   Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      order_id_format: d.fetch("format"),
      order_id_type:   d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "format" => @order_id_format,
      "type"   => @order_id_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class DataProperties < Dry::Struct
  attribute :order_id, OrderID

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      order_id: OrderID.from_dynamic!(d.fetch("order_id")),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "order_id" => @order_id.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class DataClass < Dry::Struct
  attribute :additional_properties, Types::Bool
  attribute :properties,            DataProperties
  attribute :required,              Types.Array(Types::String)
  attribute :title,                 Types::String
  attribute :data_type,             Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      additional_properties: d.fetch("additionalProperties"),
      properties:            DataProperties.from_dynamic!(d.fetch("properties")),
      required:              d.fetch("required"),
      title:                 d.fetch("title"),
      data_type:             d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "additionalProperties" => @additional_properties,
      "properties"           => @properties.to_dynamic,
      "required"             => @required,
      "title"                => @title,
      "type"                 => @data_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class PropertiesData < Dry::Struct
  attribute :ref, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      ref: d.fetch("$ref"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "$ref" => @ref,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Status < Dry::Struct
  attribute :status_type, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      status_type: d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "type" => @status_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class MFOrderResponseProperties < Dry::Struct
  attribute :data,   PropertiesData
  attribute :status, Status

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:   PropertiesData.from_dynamic!(d.fetch("data")),
      status: Status.from_dynamic!(d.fetch("status")),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"   => @data.to_dynamic,
      "status" => @status.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class MFOrderResponseClass < Dry::Struct
  attribute :additional_properties,        Types::Bool
  attribute :properties,                   MFOrderResponseProperties
  attribute :required,                     Types.Array(Types::String)
  attribute :title,                        Types::String
  attribute :mf_order_response_class_type, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      additional_properties:        d.fetch("additionalProperties"),
      properties:                   MFOrderResponseProperties.from_dynamic!(d.fetch("properties")),
      required:                     d.fetch("required"),
      title:                        d.fetch("title"),
      mf_order_response_class_type: d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "additionalProperties" => @additional_properties,
      "properties"           => @properties.to_dynamic,
      "required"             => @required,
      "title"                => @title,
      "type"                 => @mf_order_response_class_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Definitions < Dry::Struct
  attribute :data,              DataClass
  attribute :mf_order_response, MFOrderResponseClass

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:              DataClass.from_dynamic!(d.fetch("Data")),
      mf_order_response: MFOrderResponseClass.from_dynamic!(d.fetch("MFOrderResponse")),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "Data"            => @data.to_dynamic,
      "MFOrderResponse" => @mf_order_response.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class MFOrderResponse < Dry::Struct
  attribute :ref,         Types::String
  attribute :schema,      Types::String
  attribute :definitions, Definitions

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      ref:         d.fetch("$ref"),
      schema:      d.fetch("$schema"),
      definitions: Definitions.from_dynamic!(d.fetch("definitions")),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "$ref"        => @ref,
      "$schema"     => @schema,
      "definitions" => @definitions.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end
