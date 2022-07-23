pub const CORE_TYPE_INT: u8 = 0x01;
pub const CORE_TYPE_LONG: u8 = 0x02;
pub const CORE_TYPE_STRING: u8 = 0x03;
pub const CORE_TYPE_DATE: u8 = 0x04;
pub const CORE_TYPE_TIMESTAMP: u8 = 0x05;
pub const CORE_TYPE_CLASS: u8 = 0x06;
pub const CORE_TYPE_DOUBLE: u8 = 0x07;
pub const CORE_TYPE_FLOAT: u8 = 0x08;
pub const CORE_TYPE_LIST: u8 = 0x09;
pub const CORE_TYPE_MAP: u8 = 0x0a;
pub const CORE_TYPE_SET: u8 = 0x0b;
pub const CORE_TYPE_UUID: u8 = 0x0c;
pub const CORE_TYPE_EDGE: u8 = 0x0d;
pub const CORE_TYPE_PATH: u8 = 0x0e;
pub const CORE_TYPE_PROPERTY: u8 = 0x0f;
pub const CORE_TYPE_TINKERGRAPH: u8 = 0x10;
pub const CORE_TYPE_VERTEX: u8 = 0x11;
pub const CORE_TYPE_VERTEX_PROPERTY: u8 = 0x12;
pub const CORE_TYPE_BARRIER: u8 = 0x13;
pub const CORE_TYPE_BINDING: u8 = 0x14;
pub const CORE_TYPE_BYTECODE: u8 = 0x15;
pub const CORE_TYPE_CARDINALITY: u8 = 0x16;
pub const CORE_TYPE_COLUMN: u8 = 0x17;
pub const CORE_TYPE_DIRECTION: u8 = 0x18;
pub const CORE_TYPE_OPERATOR: u8 = 0x19;
pub const CORE_TYPE_ORDER: u8 = 0x1a;
pub const CORE_TYPE_PICK: u8 = 0x1b;
pub const CORE_TYPE_POP: u8 = 0x1c;
pub const CORE_TYPE_LAMBDA: u8 = 0x1d;
pub const CORE_TYPE_P: u8 = 0x1e;
pub const CORE_TYPE_SCOPE: u8 = 0x1f;
pub const CORE_TYPE_T: u8 = 0x20;
pub const CORE_TYPE_TRAVERSER: u8 = 0x21;
pub const CORE_TYPE_BIG_DECIMAL: u8 = 0x22;
pub const CORE_TYPE_BIG_INTEGER: u8 = 0x23;
pub const CORE_TYPE_BYTE: u8 = 0x24;
pub const CORE_TYPE_BYTE_BUFFER: u8 = 0x25;
pub const CORE_TYPE_SHORT: u8 = 0x26;
pub const CORE_TYPE_BOOLEAN: u8 = 0x27;
pub const CORE_TYPE_TEXT_P: u8 = 0x28;
pub const CORE_TYPE_TRAVERSAL_STRATEGY: u8 = 0x29;
pub const CORE_TYPE_BULK_SET: u8 = 0x2a;
pub const CORE_TYPE_TREE: u8 = 0x2b;
pub const CORE_TYPE_METRICS: u8 = 0x2c;
pub const CORE_TYPE_TRAVERSAL_METRICS: u8 = 0x2d;
pub const CORE_TYPE_MERGE: u8 = 0x2e;
pub const CORE_TYPE_UNSPECIFIED_NULL: u8 = 0xfe;
pub const CORE_TYPE_CUSTOM: u8 = 0x00;

#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_CHAR: u8 = 0x80;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_DURATION: u8 = 0x81;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_INET_ADDRESS: u8 = 0x82;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_INSTANT: u8 = 0x83;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_LOCAL_DATE: u8 = 0x84;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_LOCAL_DATETIME: u8 = 0x85;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_LOCAL_TIME: u8 = 0x86;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_MONTH_DAY: u8 = 0x87;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_OFFSET_DATETIME: u8 = 0x88;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_OFFSET_TIME: u8 = 0x89;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_PERIOD: u8 = 0x8a;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_YEAR: u8 = 0x8b;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_YEAR_MONTH: u8 = 0x8c;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_ZONED_DATETIME: u8 = 0x8d;
#[cfg(feature = "extended")]
pub const EXTENDED_TYPE_ZONED_OFFSET: u8 = 0x8f;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CoreType {
    Int32,
    Long,
    String,
    Date,
    Timestamp,
    Class,
    Double,
    Float,
    List,
    Set,
    Map,
    Uuid,
    Edge,
    Path,
    Property,
    Graph,
    Vertex,
    VertexProperty,
    Barrier,
    Binding,
    ByteCode,
    Cardinality,
    Column,
    Direction,
    Operator,
    Order,
    Pick,
    Pop,
    Lambda,
    P,
    Scope,
    T,
    Traverser,
    BigInteger,
    BigDecimal,
    Byte,
    ByteBuffer,
    Short,
    Boolean,
    TextP,
    TraversalStrategy,
    BulkSet,
    Tree,
    Metrics,
    TraversalMetrics,
    Merge,

    UnspecifiedNullObject,
    // extended types
    #[cfg(feature = "extended")]
    Char,
    #[cfg(feature = "extended")]
    Duration,
    #[cfg(feature = "extended")]
    InetAddress,
    #[cfg(feature = "extended")]
    Instant,
    #[cfg(feature = "extended")]
    LocalDate,
    #[cfg(feature = "extended")]
    LocalDateTime,
    #[cfg(feature = "extended")]
    LocalTime,
    #[cfg(feature = "extended")]
    MonthDay,
    #[cfg(feature = "extended")]
    OffsetDateTime,
    #[cfg(feature = "extended")]
    OffsetTime,
    #[cfg(feature = "extended")]
    Period,
    #[cfg(feature = "extended")]
    Year,
    #[cfg(feature = "extended")]
    YearMonth,
    #[cfg(feature = "extended")]
    ZonedDateTime,
    #[cfg(feature = "extended")]
    ZoneOffset,
}

impl From<CoreType> for u8 {
    fn from(ct: CoreType) -> Self {
        match ct {
            CoreType::Int32 => CORE_TYPE_INT,
            CoreType::Long => CORE_TYPE_LONG,
            CoreType::String => CORE_TYPE_STRING,
            CoreType::Date => CORE_TYPE_DATE,
            CoreType::Timestamp => CORE_TYPE_TIMESTAMP,
            CoreType::Class => CORE_TYPE_CLASS,
            CoreType::Double => CORE_TYPE_DOUBLE,
            CoreType::Float => CORE_TYPE_FLOAT,
            CoreType::List => CORE_TYPE_LIST,
            CoreType::Set => CORE_TYPE_SET,
            CoreType::Map => CORE_TYPE_MAP,
            CoreType::Uuid => CORE_TYPE_UUID,
            CoreType::Edge => CORE_TYPE_EDGE,
            CoreType::Path => CORE_TYPE_PATH,
            CoreType::Property => CORE_TYPE_PROPERTY,
            CoreType::Graph => CORE_TYPE_TINKERGRAPH,
            CoreType::Vertex => CORE_TYPE_VERTEX,
            CoreType::VertexProperty => CORE_TYPE_VERTEX_PROPERTY,
            CoreType::Barrier => CORE_TYPE_BARRIER,
            CoreType::Binding => CORE_TYPE_BINDING,
            CoreType::ByteCode => CORE_TYPE_BYTECODE,
            CoreType::Cardinality => CORE_TYPE_CARDINALITY,
            CoreType::Column => CORE_TYPE_COLUMN,
            CoreType::Direction => CORE_TYPE_DIRECTION,
            CoreType::Operator => CORE_TYPE_OPERATOR,
            CoreType::Order => CORE_TYPE_ORDER,
            CoreType::Pick => CORE_TYPE_PICK,
            CoreType::Pop => CORE_TYPE_POP,
            CoreType::Lambda => CORE_TYPE_LAMBDA,
            CoreType::P => CORE_TYPE_P,
            CoreType::Scope => CORE_TYPE_SCOPE,
            CoreType::T => CORE_TYPE_T,
            CoreType::BigInteger => CORE_TYPE_BIG_INTEGER,
            CoreType::BigDecimal => CORE_TYPE_BIG_DECIMAL,
            CoreType::Short => CORE_TYPE_SHORT,
            CoreType::Boolean => CORE_TYPE_BOOLEAN,
            CoreType::Traverser => CORE_TYPE_TRAVERSER,
            CoreType::Byte => CORE_TYPE_BYTE,
            CoreType::ByteBuffer => CORE_TYPE_BYTE_BUFFER,
            CoreType::TextP => CORE_TYPE_TEXT_P,
            CoreType::TraversalStrategy => CORE_TYPE_TRAVERSAL_STRATEGY,
            CoreType::Tree => CORE_TYPE_TREE,
            CoreType::Metrics => CORE_TYPE_METRICS,
            CoreType::TraversalMetrics => CORE_TYPE_TRAVERSAL_METRICS,
            CoreType::BulkSet => CORE_TYPE_BULK_SET,
            CoreType::Merge => CORE_TYPE_MERGE,
            CoreType::UnspecifiedNullObject => CORE_TYPE_UNSPECIFIED_NULL,
            #[cfg(feature = "extended")]
            CoreType::Char => EXTENDED_TYPE_CHAR,
            #[cfg(feature = "extended")]
            CoreType::Duration => EXTENDED_TYPE_DURATION,
            #[cfg(feature = "extended")]
            CoreType::InetAddress => EXTENDED_TYPE_INET_ADDRESS,
            #[cfg(feature = "extended")]
            CoreType::Instant => EXTENDED_TYPE_INSTANT,
            #[cfg(feature = "extended")]
            CoreType::LocalDate => EXTENDED_TYPE_LOCAL_DATE,
            #[cfg(feature = "extended")]
            CoreType::LocalDateTime => EXTENDED_TYPE_LOCAL_DATETIME,
            #[cfg(feature = "extended")]
            CoreType::LocalTime => EXTENDED_TYPE_LOCAL_TIME,
            #[cfg(feature = "extended")]
            CoreType::MonthDay => EXTENDED_TYPE_MONTH_DAY,
            #[cfg(feature = "extended")]
            CoreType::OffsetDateTime => EXTENDED_TYPE_OFFSET_DATETIME,
            #[cfg(feature = "extended")]
            CoreType::OffsetTime => EXTENDED_TYPE_OFFSET_TIME,
            #[cfg(feature = "extended")]
            CoreType::Period => EXTENDED_TYPE_PERIOD,
            #[cfg(feature = "extended")]
            CoreType::Year => EXTENDED_TYPE_YEAR,
            #[cfg(feature = "extended")]
            CoreType::YearMonth => EXTENDED_TYPE_YEAR_MONTH,
            #[cfg(feature = "extended")]
            CoreType::ZonedDateTime => EXTENDED_TYPE_ZONED_DATETIME,
            #[cfg(feature = "extended")]
            CoreType::ZoneOffset => EXTENDED_TYPE_ZONED_OFFSET,
        }
    }
}

use crate::error::DecodeError;

impl TryFrom<u8> for CoreType {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            CORE_TYPE_INT => Ok(CoreType::Int32),
            CORE_TYPE_LONG => Ok(CoreType::Long),
            CORE_TYPE_STRING => Ok(CoreType::String),
            CORE_TYPE_DATE => Ok(CoreType::Date),
            CORE_TYPE_TIMESTAMP => Ok(CoreType::Timestamp),
            CORE_TYPE_CLASS => Ok(CoreType::Class),
            CORE_TYPE_DOUBLE => Ok(CoreType::Double),
            CORE_TYPE_FLOAT => Ok(CoreType::Float),
            CORE_TYPE_LIST => Ok(CoreType::List),
            CORE_TYPE_MAP => Ok(CoreType::Map),
            CORE_TYPE_SET => Ok(CoreType::Set),
            CORE_TYPE_UUID => Ok(CoreType::Uuid),
            CORE_TYPE_EDGE => Ok(CoreType::Edge),
            CORE_TYPE_PATH => Ok(CoreType::Path),
            CORE_TYPE_PROPERTY => Ok(CoreType::Property),
            CORE_TYPE_TINKERGRAPH => Ok(CoreType::Graph),
            CORE_TYPE_VERTEX => Ok(CoreType::Vertex),
            CORE_TYPE_VERTEX_PROPERTY => Ok(CoreType::VertexProperty),
            CORE_TYPE_BARRIER => Ok(CoreType::Barrier),
            CORE_TYPE_BINDING => Ok(CoreType::Binding),
            CORE_TYPE_BYTECODE => Ok(CoreType::ByteCode),
            CORE_TYPE_CARDINALITY => Ok(CoreType::Cardinality),
            CORE_TYPE_COLUMN => Ok(CoreType::Column),
            CORE_TYPE_DIRECTION => Ok(CoreType::Direction),
            CORE_TYPE_OPERATOR => Ok(CoreType::Operator),
            CORE_TYPE_ORDER => Ok(CoreType::Order),
            CORE_TYPE_PICK => Ok(CoreType::Pick),
            CORE_TYPE_POP => Ok(CoreType::Pop),
            CORE_TYPE_LAMBDA => Ok(CoreType::Lambda),
            CORE_TYPE_P => Ok(CoreType::P),
            CORE_TYPE_SCOPE => Ok(CoreType::Scope),
            CORE_TYPE_T => Ok(CoreType::T),
            CORE_TYPE_TRAVERSER => Ok(CoreType::Traverser),
            CORE_TYPE_BIG_DECIMAL => Ok(CoreType::BigDecimal),
            CORE_TYPE_BIG_INTEGER => Ok(CoreType::BigInteger),
            CORE_TYPE_BYTE => Ok(CoreType::Byte),
            CORE_TYPE_BYTE_BUFFER => Ok(CoreType::ByteBuffer),
            CORE_TYPE_SHORT => Ok(CoreType::Short),
            CORE_TYPE_BOOLEAN => Ok(CoreType::Boolean),
            CORE_TYPE_TEXT_P => Ok(CoreType::TextP),
            CORE_TYPE_TRAVERSAL_STRATEGY => Ok(CoreType::TraversalStrategy),
            CORE_TYPE_BULK_SET => Ok(CoreType::BulkSet),
            CORE_TYPE_TREE => Ok(CoreType::Tree),
            CORE_TYPE_METRICS => Ok(CoreType::Metrics),
            CORE_TYPE_TRAVERSAL_METRICS => Ok(CoreType::TraversalMetrics),
            CORE_TYPE_MERGE => Ok(CoreType::Merge),
            CORE_TYPE_UNSPECIFIED_NULL => Ok(CoreType::UnspecifiedNullObject),
            // CORE_TYPE_CUSTOM => Ok(CoreType::),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_CHAR => Ok(CoreType::Char),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_DURATION => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_INET_ADDRESS => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_INSTANT => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_LOCAL_DATE => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_LOCAL_DATETIME => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_LOCAL_TIME => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_MONTH_DAY => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_OFFSET_DATETIME => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_OFFSET_TIME => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_PERIOD => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_YEAR => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_YEAR_MONTH => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_ZONED_DATETIME => unimplemented!("extended Types are not yet supported"),
            #[cfg(feature = "extended")]
            EXTENDED_TYPE_ZONED_OFFSET => unimplemented!("extended Types are not yet supported"),
            rest => Err(DecodeError::ConvertError(format!("found {rest}"))),
        }
    }
}
