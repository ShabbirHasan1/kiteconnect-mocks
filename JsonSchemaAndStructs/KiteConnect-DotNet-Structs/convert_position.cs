// <auto-generated />
//
// To parse this JSON data, add NuGet 'Newtonsoft.Json' then do:
//
//    using QuickType;
//
//    var convertPosition = ConvertPosition.FromJson(jsonString);

namespace QuickType
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    public partial class ConvertPosition
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
        [JsonProperty("ConvertPosition")]
        public ConvertPositionClass ConvertPosition { get; set; }
    }

    public partial class ConvertPositionClass
    {
        [JsonProperty("additionalProperties")]
        public bool AdditionalProperties { get; set; }

        [JsonProperty("properties")]
        public Properties Properties { get; set; }

        [JsonProperty("required")]
        public string[] ConvertPositionClassRequired { get; set; }

        [JsonProperty("title")]
        public string Title { get; set; }

        [JsonProperty("type")]
        public string Type { get; set; }
    }

    public partial class Properties
    {
        [JsonProperty("data")]
        public Data Data { get; set; }

        [JsonProperty("status")]
        public Data Status { get; set; }
    }

    public partial class Data
    {
        [JsonProperty("type")]
        public string Type { get; set; }
    }

    public partial class ConvertPosition
    {
        public static ConvertPosition FromJson(string json) => JsonConvert.DeserializeObject<ConvertPosition>(json, QuickType.Converter.Settings);
    }

    public static class Serialize
    {
        public static string ToJson(this ConvertPosition self) => JsonConvert.SerializeObject(self, QuickType.Converter.Settings);
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
