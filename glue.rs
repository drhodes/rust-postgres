import std::time;

enum IP {
    IPv4(u8,u8,u8,u8),
    IPv6(u16,u16,u16,u16,u16,u16,u16),
}

enum PgData {
    // +------------------------------------------------------------------------+
    // |          Name          |     Aliases      |        Description         |
    // |------------------------+------------------+----------------------------|
    // | bigint                 | int8             | signed eight-byte integer  |
    BigInt(i64),
    
    // |------------------------+------------------+----------------------------|
    // | bigserial              | serial8          | autoincrementing           |
    // |                        |                  | eight-byte integer         |
    BigSerial(i64),
        
    // |------------------------+------------------+----------------------------|
    // | bit [ (n) ]            |                  | fixed-length bit string    |
    Bit([bool]),

    // |------------------------+------------------+----------------------------|
    // | bit varying [ (n) ]    | varbit           | variable-length bit string |
    VarBit([bool]),

    // |------------------------+------------------+----------------------------|
    // | boolean                | bool             | logical Boolean            |
    // |                        |                  | (true/false)               |
    Bool(bool),
        
    // |------------------------+------------------+----------------------------|
    // | box                    |                  | rectangular box on a plane |
    //Box(f64, f64),

    // |------------------------+------------------+----------------------------|
    // | bytea                  |                  | binary data ("byte array") |
    ByteA([u8]),

    // |------------------------+------------------+----------------------------|
    // | character varying [    | varchar [ (n) ]  | variable-length character  |
    // | (n) ]                  |                  | string                     |
    VarChar(int, str),

    // |------------------------+------------------+----------------------------|
    // | character [ (n) ]      | char [ (n) ]     | fixed-length character     |
    // |                        |                  | string                     |
    Char(str),

    // |------------------------+------------------+----------------------------|
    // | cidr                   |                  | IPv4 or IPv6 network       |
    // |                        |                  | address                    |
    Cidr(IP),

    // |------------------------+------------------+----------------------------|
    // | circle                 |                  | circle on a plane          |
    // |------------------------+------------------+----------------------------|
    // | date                   |                  | calendar date (year,       |
    // |                        |                  | month, day)                |
    Date(u8,u8,u8),
    
    // |------------------------+------------------+----------------------------|
    // |                        |                  | double precision           |
    // | double precision       | float8           | floating-point number (8   |
    // |                        |                  | bytes)                     |
    Float64(f64),    

    // |------------------------+------------------+----------------------------|
    // | inet                   |                  | IPv4 or IPv6 host address  |
    Inet(IP),
    
    // |------------------------+------------------+----------------------------|
    // | integer                | int, int4        | signed four-byte integer   |
    Int32(i32), // change from int4 to int32 because of brain violence.

    // |------------------------+------------------+----------------------------|
    // | interval [ fields ] [  |                  | time span                  |
    // | (p) ]                  |                  |                            |
    

    // |------------------------+------------------+----------------------------|
    // | line                   |                  | infinite line on a plane   |
    // |------------------------+------------------+----------------------------|
    // | lseg                   |                  | line segment on a plane    |
    // |------------------------+------------------+----------------------------|
    // | macaddr                |                  | MAC (Media Access Control) |
    // |                        |                  | address                    |
    MacAddr(u8,u8,u8,u8,u8,u8),

    // |------------------------+------------------+----------------------------|
    // | money                  |                  | currency amount            |
    // |------------------------+------------------+----------------------------|
    // | numeric [ (p, s) ]     | decimal [ (p, s) | exact numeric of           |
    // |                        | ]                | selectable precision       |
    // |------------------------+------------------+----------------------------|
    // | path                   |                  | geometric path on a plane  |
    // |------------------------+------------------+----------------------------|
    // | point                  |                  | geometric point on a plane |
    // |------------------------+------------------+----------------------------|
    // | polygon                |                  | closed geometric path on a |
    // |                        |                  | plane                      |
    // |------------------------+------------------+----------------------------|
    // |                        |                  | single precision           |
    // | real                   | float4           | floating-point number (4   |
    // |                        |                  | bytes)                     |
    Float32(f32),

    // |------------------------+------------------+----------------------------|
    // | smallint               | int2             | signed two-byte integer    |
    Int16(i16),
        
    // |------------------------+------------------+----------------------------|
    // | serial                 | serial4          | autoincrementing four-byte |
    // |                        |                  | integer                    |
    Serial(i32),

    // |------------------------+------------------+----------------------------|
    // | text                   |                  | variable-length character  |
    // |                        |                  | string                     |
    Text(str),

    // |------------------------+------------------+----------------------------|
    // | time [ (p) ] [ without |                  | time of day (no time zone) |
    // | time zone ]            |                  |                            |
    Time(time::tm),

    // |------------------------+------------------+----------------------------|
    // | time [ (p) ] with time | timetz           | time of day, including     |
    // | zone                   |                  | time zone                  |
    TimeTz(time::tm),

    // |------------------------+------------------+----------------------------|
    // | timestamp [ (p) ] [    |                  | date and time (no time     |
    // | without time zone ]    |                  | zone)                      |
    // |------------------------+------------------+----------------------------|
    // | timestamp [ (p) ] with | timestamptz      | date and time, including   |
    // | time zone              |                  | time zone                  |
    // |------------------------+------------------+----------------------------|
    // | tsquery                |                  | text search query          |
    // |------------------------+------------------+----------------------------|
    // | tsvector               |                  | text search document       |
    // |------------------------+------------------+----------------------------|
    // | txid_snapshot          |                  | user-level transaction ID  |
    // |                        |                  | snapshot                   |
    // |------------------------+------------------+----------------------------|
    // | uuid                   |                  | universally unique         |
    // |                        |                  | identifier                 |
    Uuid([u8]),
    // |------------------------+------------------+----------------------------|
    // | xml                    |                  | XML data                   |

    Xml([u8]),
    // +------------------------------------------------------------------------+
}

iface Show {
    fn show() -> str;
}

impl of Show for PgData {
    fn show() -> str { 
        alt self {
          BigInt(n) {"Asdf"}
          VarChar(n, s) {#fmt("'%s'", s)}
          Int4(n) {#fmt("%d", n as int)}
          _ { "asdf"}
        }
    }
}






          // BigSerial(n),
          // Bit(bs),
          // VarBit(bs),
          // Bool(b),
          // ByteA([u8]),
          // VarChar(int, str),
          // Char(str),
          // Cidr(IP),
          // Date(u8,u8,u8),
          // Double(f64),    
          // Inet(IP),
          // Int4(i32),
          // MacAddr(u8,u8,u8,u8,u8,u8),
          // Real(f32),
          // SmallInt(i16),
          // Serial(i32),
          // Text(str),
          // Uuid(str),
          // Xml([u8]),
