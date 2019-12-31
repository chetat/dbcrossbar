CREATE TABLE "testme1"."cp_csv_to_postgres_to_gs_to_csv" (
    "test_null" text,
    "test_not_null" text NOT NULL,
    "test_bool" boolean,
    "test_bool_array" boolean[],
    "test_date" date,
    "test_date_array" date[],
    "test_float32" real,
    "test_float32_array" real[],
    "test_float64" double precision,
    "test_float64_array" double precision[],
    "test_geojson" public.geometry(Geometry, 4326),
    "test_geojson_3857" public.geometry(Geometry, 3857),
    "test_int16" smallint,
    "test_int16_array" smallint[],
    "test_int32" int,
    "test_int32_array" int[],
    "test_int64" bigint,
    "test_int64_array" bigint[],
    "test_json" jsonb,
    "test_text" text,
    "test_text_array" text[],
    "test_timestamp_without_time_zone" timestamp without time zone,
    "test_timestamp_without_time_zone_array" timestamp without time zone[],
    "test_timestamp_with_time_zone" timestamp with time zone,
    "test_timestamp_with_time_zone_array" timestamp with time zone[],
    "test_uuid" uuid,
    "test_uuid_array" uuid[]
);
