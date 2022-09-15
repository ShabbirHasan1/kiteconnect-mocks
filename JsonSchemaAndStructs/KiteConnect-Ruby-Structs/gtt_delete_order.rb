# This code may look unusually verbose for Ruby (and it is), but
# it performs some subtle and complex validation of JSON data.
#
# To parse this JSON, add 'dry-struct' and 'dry-types' gems, then do:
#
#   gtt_delete_order = GttDeleteOrder.from_json! "{…}"
#   puts gtt_delete_order.definitions.gtt_delete_order.required.first
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

class TriggerID < Dry::Struct
  attribute :trigger_id_type, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      trigger_id_type: d.fetch("type"),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "type" => @trigger_id_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class DataProperties < Dry::Struct
  attribute :trigger_id, TriggerID

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      trigger_id: TriggerID.from_dynamic!(d.fetch("trigger_id")),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "trigger_id" => @trigger_id.to_dynamic,
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

class GttDeleteOrderProperties < Dry::Struct
  attribute :data,   PropertiesData
  attribute :status, TriggerID

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:   PropertiesData.from_dynamic!(d.fetch("data")),
      status: TriggerID.from_dynamic!(d.fetch("status")),
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

class GttDeleteOrderClass < Dry::Struct
  attribute :additional_properties,       Types::Bool
  attribute :properties,                  GttDeleteOrderProperties
  attribute :required,                    Types.Array(Types::String)
  attribute :title,                       Types::String
  attribute :gtt_delete_order_class_type, Types::String

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      additional_properties:       d.fetch("additionalProperties"),
      properties:                  GttDeleteOrderProperties.from_dynamic!(d.fetch("properties")),
      required:                    d.fetch("required"),
      title:                       d.fetch("title"),
      gtt_delete_order_class_type: d.fetch("type"),
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
      "type"                 => @gtt_delete_order_class_type,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class Definitions < Dry::Struct
  attribute :data,             DataClass
  attribute :gtt_delete_order, GttDeleteOrderClass

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:             DataClass.from_dynamic!(d.fetch("Data")),
      gtt_delete_order: GttDeleteOrderClass.from_dynamic!(d.fetch("GttDeleteOrder")),
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "Data"           => @data.to_dynamic,
      "GttDeleteOrder" => @gtt_delete_order.to_dynamic,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class GttDeleteOrder < Dry::Struct
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
