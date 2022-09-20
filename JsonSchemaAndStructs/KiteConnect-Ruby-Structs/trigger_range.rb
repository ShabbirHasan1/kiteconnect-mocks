# This code may look unusually verbose for Ruby (and it is), but
# it performs some subtle and complex validation of JSON data.
#
# To parse this JSON, add 'dry-struct' and 'dry-types' gems, then do:
#
#   trigger_range = TriggerRange.from_json! "{…}"
#   puts trigger_range.data&["…"].instrument_token.even?
#
# If from_json! succeeds, the value returned matches the schema.

require 'json'
require 'dry-types'
require 'dry-struct'

module Types
  include Dry::Types.module

  Int    = Strict::Int
  Hash   = Strict::Hash
  String = Strict::String
  Double = Strict::Float | Strict::Int
end

class Datum < Dry::Struct
  attribute :instrument_token, Types::Int.optional
  attribute :lower,            Types::Double.optional
  attribute :upper,            Types::Double.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      instrument_token: d["instrument_token"],
      lower:            d["lower"],
      upper:            d["upper"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "instrument_token" => @instrument_token,
      "lower"            => @lower,
      "upper"            => @upper,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end

class TriggerRange < Dry::Struct
  attribute :data,   Types::Hash.meta(of: Datum).optional
  attribute :status, Types::String.optional

  def self.from_dynamic!(d)
    d = Types::Hash[d]
    new(
      data:   Types::Hash.optional[d["data"]]&.map { |k, v| [k, Datum.from_dynamic!(v)] }&.to_h,
      status: d["status"],
    )
  end

  def self.from_json!(json)
    from_dynamic!(JSON.parse(json))
  end

  def to_dynamic
    {
      "data"   => @data&.map { |k, v| [k, v.to_dynamic] }.to_h,
      "status" => @status,
    }
  end

  def to_json(options = nil)
    JSON.generate(to_dynamic, options)
  end
end
