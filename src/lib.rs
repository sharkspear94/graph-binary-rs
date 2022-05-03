mod graph_binary;
mod ser;
mod specs;
mod traits;

mod error;
mod message;
mod structure;

#[cfg(test)]
mod tests {}
// Example usage
// fn main() {
//     let int = 15_i32;
//     let option_int = Some(15_i32);

//     let mut buf = [0_u8;256];
//     graph_binary::encode_to_slice(buf,int);

//     let vec: Result<Vec<u8>> = graph_binary::encode_to_vec(int);
//     let vec: Result<Vec<u8>> = graph_binary::encode_to_vec(int);

//     let mut file = File::open(path)?;

//     graph_binary::encode_to_writer(file,int);
// }

// enum Parant {
//     Vertex(Vertex),
//     Edge(Edge)
// }

// struct Property<T>{
//     key: String,
//     value: T,
//     parent: Parant,
// }

// struct Date {

// // }
// struct List {
//     list: Vec<CoreType>
// }

// trait TypeCode {}

// enum CoreType {
//     Int = 0x01,
//     Long = 0x02,
//     String = 0x03,
//     Date = 0x04,
//     Timestamp = 0x05,
//     Class = 0x06,
//     Double = 0x07,
//     Float = 0x08,
//     List = 0x09,
//     Map = 0x0a,
//     Set = 0x0b,
//     Uuid = 0x0c,
//     Edge = 0x0d,
//     Path = 0x0e,
//     Property = 0x0f,
//     TinkerGraph = 0x10,
//     Vertex = 0x11,
//     VertexProperty = 0x12,
//     Barrier = 0x13,
//     Binding = 0x14,
//     Bytecode = 0x15,
//     Cardinality = 0x16,
//     Column = 0x17,
//     Direction = 0x18,
//     Operator = 0x19,
//     Order = 0x1a,
//     Pick = 0x1b,
//     Pop = 0x1c,
//     Lambda = 0x1d,
//     P = 0x1e,
//     Scope = 0x1f,
//     T = 0x20,
//     Traverser = 0x21,
//     BigDecimal = 0x22,
//     BigInteger = 0x23,
//     Byte = 0x24,
//     ByteBuffer = 0x25,
//     Short = 0x26,
//     Boolean = 0x27,
//     TextP = 0x28,
//     TraversalStrategy = 0x29,
//     BulkSet = 0x2a,
//     Tree = 0x2b,
//     Metrics = 0x2c,
//     TraversalMetrics = 0x2d,
//     UnspecifiedNull = 0xfe,
//     Custom = 0x00,
// }

// enum ExtendedType {
//     Char = 0x80,
//     Duration = 0x81,
//     InetAddress = 0x82,
//     Instant = 0x83,
//     LocalDate = 0x84,
//     LacalDateTime = 0x85,
//     LocalTime = 0x86,
//     MonthDay = 0x87,
//     OffsetDateTime = 0x88,
//     OffsetTime = 0x89,
//     Period = 0x8a,
//     Year = 0x8b,
//     YearMonth = 0x8c,
//     ZonedDateTime = 0x8d,
//     ZonedOffset = 0x8f,
// }

// impl TypeCode for CoreType {}

// impl TypeCode for ExtendedType {}
