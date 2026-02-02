//! Macro that turns the schema docs into executable Arrow schemas and batch wrappers.

/// TXT axiom runtime check (safety net).
pub fn __check_txt_invariants(
    table_name: &str,
    plane: &crate::plane::Plane,
    schema: &::arrow::datatypes::Schema,
) -> anyhow::Result<()> {
    use crate::plane::Plane;
    use ::arrow::datatypes::DataType;

    if matches!(plane, Plane::Control | Plane::Execution) {
        for f in schema.fields() {
            if matches!(f.data_type(), DataType::LargeUtf8) {
                anyhow::bail!(
                    "TXT axiom violated: table {} in plane {:?} has LargeUtf8 column {}",
                    table_name,
                    plane,
                    f.name()
                );
            }
        }
    }
    Ok(())
}

/// Map DSL token -> Arrow DataType.
#[macro_export]
macro_rules! __ty_to_arrow {
    ( FixedSizeBinary ( $n:literal ) ) => {
        ::arrow::datatypes::DataType::FixedSizeBinary($n)
    };
    ( Utf8 ) => { ::arrow::datatypes::DataType::Utf8 };
    ( LargeUtf8 ) => { ::arrow::datatypes::DataType::LargeUtf8 };
    ( UInt32 ) => { ::arrow::datatypes::DataType::UInt32 };
    ( UInt16 ) => { ::arrow::datatypes::DataType::UInt16 };
    ( UInt8 )  => { ::arrow::datatypes::DataType::UInt8 };
    ( Int64 )  => { ::arrow::datatypes::DataType::Int64 };
    ( Int32 )  => { ::arrow::datatypes::DataType::Int32 };
    ( Int16 )  => { ::arrow::datatypes::DataType::Int16 };
    ( Boolean ) => { ::arrow::datatypes::DataType::Boolean };
    ( Float32 ) => { ::arrow::datatypes::DataType::Float32 };
    ( Float64 ) => { ::arrow::datatypes::DataType::Float64 };
    ( DictU8Utf8 ) => {
        ::arrow::datatypes::DataType::Dictionary(
            Box::new(::arrow::datatypes::DataType::UInt8),
            Box::new(::arrow::datatypes::DataType::Utf8),
        )
    };
    ( MapUtf8Utf8 ) => {
        ::arrow::datatypes::DataType::Map(
            Box::new(::arrow::datatypes::Field::new(
                "entry",
                ::arrow::datatypes::DataType::Struct(vec![
                    ::arrow::datatypes::Field::new("key", ::arrow::datatypes::DataType::Utf8, false),
                    ::arrow::datatypes::Field::new("value", ::arrow::datatypes::DataType::Utf8, true),
                ]),
                false,
            )),
            false,
        )
    };
    ( ListUtf8 ) => {
        ::arrow::datatypes::DataType::List(Box::new(
            ::arrow::datatypes::Field::new("item", ::arrow::datatypes::DataType::Utf8, true),
        ))
    };
    ( FixedSizeListF32 ( $n:literal ) ) => {
        ::arrow::datatypes::DataType::FixedSizeList(
            Box::new(::arrow::datatypes::Field::new("item", ::arrow::datatypes::DataType::Float32, true)),
            $n as i32,
        )
    };
    ( TimestampMsUtc ) => {
        ::arrow::datatypes::DataType::Timestamp(
            ::arrow::datatypes::TimeUnit::Millisecond,
            Some("UTC".into()),
        )
    };
}

/// TXT axiom compile-time guard: forbid LargeUtf8 in Control/Execution tables when declared in the DSL.
#[macro_export]
macro_rules! __guard_txt_large_utf8 {
    (
        table_name = $Name:ident,
        plane = $plane:path,
        $( $fty:ident $( ( $($args:tt)* ) )? ),*
    ) => {
        $( #[allow(dead_code)] const _: () = {
            use $crate::plane::Plane;
            if matches!($plane, Plane::Control | Plane::Execution) {
                if stringify!($fty) == "LargeUtf8" {
                    compile_error!("TXT axiom violated: LargeUtf8 not allowed in Control/Execution planes");
                }
            }
        }; )*
    };
}

/// Declarative table definitions (Spine + payload). Generates schemas and batch wrappers.
#[macro_export]
macro_rules! define_tables {
    (
        $(
            table $Name:ident {
                plane: $plane:path,
                kind: $kind:literal,
                fields: {
                    $( $fname:ident : $fty:ident $( ( $($args:tt)* ) )? ),* $(,)?
                }
            }
        ),* $(,)?
    ) => {
        $(
            /// Strongly-typed wrapper for the `$Name` table.
            #[derive(Clone)]
            pub struct $Name {
                inner: ::std::sync::Arc<::arrow::record_batch::RecordBatch>,
            }

            impl $Name {
                /// Arrow schema for this table (Spine + payload).
                pub fn schema() -> ::arrow::datatypes::Schema {
                    let mut fields = vec![
                        // Spine
                        ::arrow::datatypes::Field::new("row_id", ::arrow::datatypes::DataType::FixedSizeBinary(16), false),
                        ::arrow::datatypes::Field::new("tenant_id", ::arrow::datatypes::DataType::FixedSizeBinary(16), false),
                        ::arrow::datatypes::Field::new("plane", ::arrow::datatypes::DataType::UInt8, false),
                        ::arrow::datatypes::Field::new(
                            "kind",
                            ::arrow::datatypes::DataType::Dictionary(Box::new(::arrow::datatypes::DataType::UInt8), Box::new(::arrow::datatypes::DataType::Utf8)),
                            false,
                        ),
                        ::arrow::datatypes::Field::new("schema_version", ::arrow::datatypes::DataType::UInt16, false),
                        ::arrow::datatypes::Field::new(
                            "ts_created",
                            ::arrow::datatypes::DataType::Timestamp(::arrow::datatypes::TimeUnit::Millisecond, Some("UTC".into())),
                            false,
                        ),
                        ::arrow::datatypes::Field::new(
                            "ts_valid_from",
                            ::arrow::datatypes::DataType::Timestamp(::arrow::datatypes::TimeUnit::Millisecond, Some("UTC".into())),
                            false,
                        ),
                    ];

                    $(
                        fields.push(::arrow::datatypes::Field::new(
                            stringify!($fname),
                            $crate::__ty_to_arrow!($fty $( ( $($args)* ) )?),
                            true,
                        ));
                    )*

                    ::arrow::datatypes::Schema::new(fields)
                }

                /// Construct a new batch wrapper (will enforce TXT axiom at runtime).
                pub fn new(inner: ::std::sync::Arc<::arrow::record_batch::RecordBatch>) -> anyhow::Result<Self> {
                    $crate::macros::__check_txt_invariants(
                        stringify!($Name),
                        &$plane,
                        inner.schema().as_ref(),
                    )?;
                    Ok(Self { inner })
                }

                /// Underlying Arrow batch.
                pub fn inner(&self) -> &::std::sync::Arc<::arrow::record_batch::RecordBatch> {
                    &self.inner
                }
            }

            // Compile-time TXT guard for obvious violations.
            $crate::__guard_txt_large_utf8! {
                table_name = $Name,
                plane = $plane,
                $( $fty $( ( $($args)* ) )? ),*
            }
        )*
    };
}
