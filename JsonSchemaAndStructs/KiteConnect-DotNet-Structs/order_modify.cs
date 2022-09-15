// <auto-generated />
//
// To parse this JSON data, add NuGet 'Newtonsoft.Json' then do:
//
//    using QuickType;
//
//    var orderModify = OrderModify.FromJson(jsonString);

namespace QuickType
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class OrderModify
    {
        [JsonProperty("$ref")]
        public string Ref { get; set; }

        [JsonProperty("$schema")]
        public Uri Schema { get; set; }

        [JsonProperty("definitions")]
        public Definitions Definitions { get; set; }
    }

    public partial class Definitions
    {
        [JsonProperty("Data")]
        public Data Data { get; set; }

        [JsonProperty("OrderModify")]
        public OrderModifyClass OrderModify { get; set; }
    }

    public partial class Data
    {
        [JsonProperty("additionalProperties")]
        public bool AdditionalProperties { get; set; }

        [JsonProperty("properties")]
        public DataProperties Properties { get; set; }

        [JsonProperty("required")]
        public string[] DataRequired { get; set; }

        [JsonProperty("title")]
        public string Title { get; set; }

        [JsonProperty("type")]
        public string Type { get; set; }
    }

    public partial class DataProperties
    {
        [JsonProperty("order_id")]
        public OrderId OrderId { get; set; }
    }

    public partial class OrderId
    {
        [JsonProperty("type")]
        public string Type { get; set; }
    }

    public partial class OrderModifyClass
    {
        [JsonProperty("additionalProperties")]
        public bool AdditionalProperties { get; set; }

        [JsonProperty("properties")]
        public OrderModifyProperties Properties { get; set; }

        [JsonProperty("required")]
        public string[] OrderModifyClassRequired { get; set; }

        [JsonProperty("title")]
        public string Title { get; set; }

        [JsonProperty("type")]
        public string Type { get; set; }
    }

    public partial class OrderModifyProperties
    {
        [JsonProperty("data")]
        public DataClass Data { get; set; }

        [JsonProperty("status")]
        public OrderId Status { get; set; }
    }

    public partial class DataClass
    {
        [JsonProperty("$ref")]
        public string Ref { get; set; }
    }

    public partial class OrderModify
    {
        public static OrderModify FromJson(string json) => JsonConvert.DeserializeObject<OrderModify>(json, QuickType.Converter.Settings);
    }

    public static class Serialize
    {
        public static string ToJson(this OrderModify self) => JsonConvert.SerializeObject(self, QuickType.Converter.Settings);
    }

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
    }
}
