import std::time;
import result::{result, ok, err};
import libc::*;

enum IP {
    IPv4(u8,u8,u8,u8),
    IPv6(u16,u16,u16,u16,u16,u16,u16),
}


#[doc = "PgData are rust representations of postgres data \
         There are three constructors around each postgres type \
           Null: If there is something wrong with a query, Null<Postgres Type> ...
                 need to reconsider this. 
           Model: These have a trailing 'M' and are used to build
                  Table types that can be conveniently inserted.
           The other constructors are used by the library to supply well typed
           rust data, so there's no need to run scan functions after a query.
           todo: improve this docstring.
         "]
enum PgData {
    // +------------------------------------------------------------------------+
    // |          Name          |     Aliases      |        Description         |
    // |------------------------+------------------+----------------------------|
    // | bigint                 | int8             | signed eight-byte integer  |
    Int64(i64),
    Int64M,
    NullInt64,

    // |------------------------+------------------+----------------------------|
    // | bigserial              | serial8          | autoincrementing           |
    // |                        |                  | eight-byte integer         |
    BigSerial(i64),
    BigSerialM,
    NullBigSerial,

    // |------------------------+------------------+----------------------------|
    // | bit [ (n) ]            |                  | fixed-length bit string    |
    Bit([bool]),
    BitM(int),
    NullBit,

    // |------------------------+------------------+----------------------------|
    // | bit varying [ (n) ]    | varbit           | variable-length bit string |
    VarBit([bool]),
    VarBitM,
    NullVarBit,

    // |------------------------+------------------+----------------------------|
    // | boolean                | bool             | logical Boolean            |
    // |                        |                  | (true/false)               |
    Bool(bool),
    BoolM,
    NullBool,

    // |------------------------+------------------+----------------------------|
    // | box                    |                  | rectangular box on a plane |
    //Box(f64, f64),

    // |------------------------+------------------+----------------------------|
    // | bytea                  |                  | binary data ("byte array") |
    ByteA([u8]),
    ByteAM,
    NullByteA,

    // |------------------------+------------------+----------------------------|
    // | character varying [    | varchar [ (n) ]  | variable-length character  |
    // | (n) ]                  |                  | string                     |
    VarCharM(int), // model
    VarChar(str),
    NullVarChar,

    // |------------------------+------------------+----------------------------|
    // | character [ (n) ]      | char [ (n) ]     | fixed-length character     |
    // |                        |                  | string                     |
    Char(str),
    CharM(int),
    NullChar,

    // |------------------------+------------------+----------------------------|
    // | cidr                   |                  | IPv4 or IPv6 network       |
    // |                        |                  | address                    |
    Cidr(IP),
    CidrM,
    NullCidr,

    // |------------------------+------------------+----------------------------|
    // | circle                 |                  | circle on a plane          |
    // |------------------------+------------------+----------------------------|
    // | date                   |                  | calendar date (year,       |
    // |                        |                  | month, day)                |
    Date(u8,u8,u8),
    DateM,
    NullDate,

    // |------------------------+------------------+----------------------------|
    // |                        |                  | double precision           |
    // | double precision       | float8           | floating-point number (8   |
    // |                        |                  | bytes)                     |
    Float64(f64),
    Float64M,
    NullFloat64,

    // |------------------------+------------------+----------------------------|
    // | inet                   |                  | IPv4 or IPv6 host address  |
    Inet(IP),
    InetM,
    NullInet,

    // |------------------------+------------------+----------------------------|
    // | integer                | int, int4        | signed four-byte integer   |
    Int32(i32),
    Int32M,
    NullInt32,

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
    MacAddrM,
    NullMacAddr,

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
    Float32M,
    NullFloat32,

    // |------------------------+------------------+----------------------------|
    // | smallint               | int2             | signed two-byte integer    |
    Int16(i16),
    Int16M,
    NullInt16,

    // |------------------------+------------------+----------------------------|
    // | serial                 | serial4          | autoincrementing four-byte |
    // |                        |                  | integer                    |
    Serial(i32),
    SerialM,
    NullSerial,
    // rhodesd> This is just a (rust int32| sql int4)
    // todo: possible to discriminate between the two by inspecting PQresult?

    // |------------------------+------------------+----------------------------|
    // | text                   |                  | variable-length character  |
    // |                        |                  | string                     |
    Text(str),
    TextM,
    NullText,

    // |------------------------+------------------+----------------------------|
    // | time [ (p) ] [ without |                  | time of day (no time zone) |
    // | time zone ]            |                  |                            |
    Time(time::tm),
    TimeM,
    NullTime,

    // |------------------------+------------------+----------------------------|
    // | time [ (p) ] with time | timetz           | time of day, including     |
    // | zone                   |                  | time zone                  |
    TimeTz(time::tm),
    TimeTzM,
    NullTimeTz,

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
    UuidM,
    NullUuid,
    // |------------------------+------------------+----------------------------|
    // | xml                    |                  | XML data                   |

    Xml([u8]),
    XmlM,
    NullXml,
    // +------------------------------------------------------------------------+
    PgDataErr(str),
}


iface Show {
    fn show() -> str;
}

impl of Show for PgData {
    fn show() -> str {
        alt self {
          Int32(n) {#fmt("%d", n as int)}
          Int32M { "INT4" }
          //
          Int64(n) {#fmt("%?", n as int)} // todo: how to format a i64?
          //
          SerialM { "SERIAL" }
          //
          VarChar(s) {#fmt("'%s'", s)}
          VarCharM(n) {#fmt("VARCHAR(%d)", n)}
          //
          BigSerial(n) {#fmt("%?", n)}
          BigSerialM {
            "BIGSERIAL"
          }
          //
          Bit(bs) {
            let bvec = bs.map({|b| if b { "1" } else { "0" }});
            let bstr = str::connect(bvec, "");
            #fmt("B'%s'", bstr)
          }
          //
          BitM(n) { #fmt("BIT(%d)", n) }
          //
          Bool(b) { if b == true {"TRUE"} else {"FALSE"}}
          BoolM {"BOOL"}

          // ByteA([u8]),
          // ByteAM,
          // Char(str),
          // CharM(n) {#fmt("VARCHAR(%d)", n)}
          // Cidr(IP),
          // CidrM

          // Date(u8,u8,u8),
          // DateM

          // Float32(f32),
          // Float32M,

          // Float64(f64),
          // Float64M,

          // Inet(IP),
          // InetM,

          // Int16(i16),
          // Int16M,

          // Int32(i32),
          // Int32M,

          // Int64(i64),
          // Int64M,

          // MacAddr(u8,u8,u8,u8,u8,u8),
          // MacAddrM,

          // NullBigSerial {"NullBigSerial"}
          // NullBit {"NullBit"}
          // NullBool {"NullBool"}
          // NullByteA {"NullByteA"}
          // NullChar {"NullChar"}
          // NullCidr {"NullCidr"}
          // NullDate {"NullDate"}
          // NullFloat32 {"NullFloat32"}
          // NullFloat64 {"NullFloat64"}
          // NullInet {"NullInet"}
          // NullInt16 {"NullInt16"}
          // NullInt32 {"NullInt32"}
          // NullInt64 {"NullInt64"}
          // NullMacAddr {"NullMacAddr"}
          // NullSerial {"NullSerial"}
          // NullText {"NullText"}
          // NullTime {"NullTime"}
          // NullTimeTz {"NullTimeTz"}
          // NullUuid {"NullUuid"}
          // NullVarBit {"NullVarBit"}
          // NullVarChar {"NullVarChar"}
          // NullXml {"NullXml"}

          // Serial(i32),
          // SerialM,

          // Text(str),
          // TextM,

          // Time(time::tm),
          // TimeM

          // TimeTz(time::tm),
          // TimeTzM

          // Uuid([u8]),
          // UuidM,

          // VarBit([bool]),
          // VarBitM

          // VarChar(str),
          // VarCharM(int), // model

          // Xml([u8]),
          // XmlM.



          // PgDataErr(str),

          _ {"ASDF"}

        }
    }
}

fn seq(a: int, b: int) -> [int] {
    let mut i = a;
    let mut accum: [int] = [];
    while i < b {
        vec::push(accum, i);
        i += 1;
    }
    accum
}

// rhodesd> I find it odd that postgres is returning string data (*char)
// that needs to be converted, instead of binary data.

#[doc = "Extract PgData field given a Result, and two indexes: tuple and field."]
unsafe fn Result2PgData(res: Result, tup: int,  fld: int) -> PgData {
    // get the field type
    assert res.Ok(); // todo: this check needs to be gone at some point
    let getrep = {|t, f|
        unsafe::from_c_str(res.GetValue(t,f))
    };
    
    alt res.Ftype(fld) {
      // -------------------------------------------------------
      BOOL {         
        let s = getrep(tup, fld);
        alt s {
          "t" { Bool(true) }
          "f" { Bool(false) }
          _ { PgDataErr(#fmt["error: \
                              Expected (t|f) in the BOOL branch of \
                              Result2PgData, got `%s`.", s])
            }         
        }
      }

      // BYTEA {}
      // CHAR {}
      // NAME {}

      // -------------------------------------------------------
      INT8 {
        let s = getrep(tup, fld);
        alt int::from_str(s) {
          none {
            PgDataErr("error in INT8, can't parse " + s) 
          }
          some(n) {
            Int64(n as i64)
          }                
        }
      }


      // INT2 {}
      INT2 {
        let s = getrep(tup, fld);
        alt int::from_str(s) {
          none {
            PgDataErr("error in INT2, can't parse " + s) 
          }
          some(n) {
            Int16(n as i16)
          }                
        }
      }


      // INT2VECTOR {}

      // -------------------------------------------------------
      INT4 {
        let s = getrep(tup, fld);
        alt int::from_str(s) {
          none {
            PgDataErr("error in INT4, can't parse " + s) 
          }
          some(n) {
            Int32(n as i32)
          }                
        }
      }
      
      // REGPROC {}

      // -------------------------------------------------------
      TEXT {
        Text(unsafe::from_c_str(res.GetValue(tup, fld)))
      }

      // OID {}
      // TID {}
      // XID {}
      // CID {}
      // VECTOR {}
      // JSON {}
      // XML {}
      // PGNODETREE {}
      // POINT {}
      // LSEG {}
      // PATH {}
      // BOX {}
      // POLYGON {}
      // LINE {}
      // FLOAT4 {}
      // FLOAT8 {}
      // ABSTIME {}
      // RELTIME {}
      // TINTERVAL {}
      // UNKNOWN {}
      // CIRCLE {}
      // CASH {}
      // MACADDR {}
      // INET {}
      // CIDR {}
      // INT4ARRAY {}
      // TEXTARRAY {}
      // FLOAT4ARRAY {}
      // ACLITEM {}
      // CSTRINGARRAY {}
      // BPCHAR {}

      // -------------------------------------------------------
      VARCHAR {
        VarChar(unsafe::from_c_str(res.GetValue(tup, fld)))
      }
      // DATE {}
      // TIME {}
      // TIMESTAMP {}
      // TIMESTAMPTZ {}
      // INTERVAL {}
      // TIMETZ {}

      // -------------------------------------------------------
      // BIT {}
      BIT {
        let s = getrep(tup, fld);
        Bit(str::chars(s).map({|x| x == '1'}))
      }

      // VARBIT {}
      // NUMERIC {}
      // REFCURSOR {}
      // REGPROCEDURE {}
      // REGOPER {}
      // REGOPERATOR {}
      // REGCLASS {}
      // REGTYPE {}
      // REGTYPEARRAY {}
      // TSVECTOR {}
      // GTSVECTOR {}
      // TSQUERY {}
      // REGCONFIG {}
      // REGDICTIONARY {}
      // INT4RANGE {}
      // RECORD {}
      // RECORDARRAY {}
      // CSTRING {}
      // ANY {}
      // ANYARRAY {}
      // VOID {}
      // TRIGGER {}
      // LANGUAGE_HANDLER {}
      // INTERNAL {}
      // OPAQUE {}
      // ANYELEMENT {}
      // ANYNONARRAY {}
      // ANYENUM {}
      // FDW_HANDLER {}
      // ANYRANGE {}
      _ { PgDataErr("Unrecognized Ftype") }
      
    }
}

// enum Field {
//     Insert(PData),
//     Ignore(PgData)
// }

fn GetRow(res: Result, rownum: int) -> [PgData] {
    unsafe {
        if res.Ok() {
            let flds = seq(0, res.Nfields());
            let f = {| x
                     | Result2PgData(res, rownum, x)};
            flds.map(f)
                
        } else {
            [PgDataErr("GetRows fails because ... ")]
        }
    }
}
    

unsafe fn GetAllRows(res: Result) -> [[PgData]] {    
    if res.Ok() {
        let tups = seq(0, res.Ntuples());
        let f = {|x|GetRow(res, x)};
        tups.map(f)

    } else {
        [[PgDataErr("GetAllRows fails because ... ")]]
    }
}

fn InsertStarWars(conn: Conn, tablename: str) {
    let insertstr = #fmt("insert into %s (title, year, director) VALUES", tablename);
    (conn.Exec( insertstr + "('a new hope', 1977, 'lucas')"));
    conn.Exec( insertstr + "('the empire strikes back', 1980, 'Kershner')");
    conn.Exec( insertstr + "('return of the jedi', 1983, 'lucas')");   
}

type Row = [PgData];

enum FieldOption {
    Insert,
    Select,
    Unique,

    // table types here may preclude mutual recursive tables.
    // meh, do it live.
    ForeignKey(Table, str), 
}

// enum Field {
//     Field(str, [FieldOption], PgData)
//    (str, [FieldOption], PgData)
// }

type Field = (str, [FieldOption], PgData);

fn InsertFieldP(fld: Field) -> bool {
    let (_, xs, _) = copy fld;
    xs.contains(Insert) 
}

enum Table {
    Table(str, [Field]),
}

iface TableI {
    fn Insert(Row) -> str;
    fn DropTable(conn: Conn) -> Result;
    fn CreateTable(conn: Conn) -> Result;
}

impl of TableI for Table {
    fn Insert(r: Row) -> str {
        let mut keepers: [str] = [];
        let Table(tname, flds) = copy self;

        for flds.each |x| {
            if InsertFieldP(x) {
                let (fldname, _, _) = copy x;
                keepers += [fldname];
            }
        }
        if r.len() != keepers.len() {
            fail("Table must have the same number of Insert fields as values")
        }

        let mut vals: [str] = [];
        for r.each |fld| {
            vals += [fld.show()];
        }

        #fmt["insert into %s %s VALUES %s", tname, 
             "("+str::connect(keepers, ",")+")",
             "("+str::connect(vals,",")+")"]             
    }    

    fn DropTable(conn: Conn) -> Result {
        let Table(name, _) = copy self;
        let q = #fmt("drop table if exists %s cascade", name);
        conn.Exec(q)
    }

    fn CreateTable(conn: Conn) -> Result {
        let Table(name, flds) = copy self;
        let mut q: [str] = [];
        
        for flds.each |fld| {
            let (fname, ops, modt) = copy fld;
            q += [#fmt["  %s %s", fname, modt.show()]];
            // for each field option, add the string represenation to query string.
            for ops.each |op|{
                alt op {
                  Insert {}
                  Select {}
                  Unique { q += [#fmt["  UNIQUE(%s)", fname]]; }
                  ForeignKey(Table(tname, _), ffldname) {
                    if tname == name {
                        // this detects if the table is self referencing
                        // still need to work out the accepted error handling.
                    } else {
                        q += [#fmt["  foreign key (%s) references %s(%s)",
                                   fname, tname, ffldname]];                 
                    }
                  }  
                }  
            }       
        }
        let head = #fmt["create table %s (\n", name];
        let q = head + str::connect(q, ",\n") + "\n)";
        conn.Exec(q)
    }
}


