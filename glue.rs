//import pq;
import std::time;
import result::{result, ok, err};
import libc::*;

enum IP {
    IPv4(u8,u8,u8,u8),
    IPv6(u16,u16,u16,u16,u16,u16,u16),
}

enum PgData {
    // +------------------------------------------------------------------------+
    // |          Name          |     Aliases      |        Description         |
    // |------------------------+------------------+----------------------------|
    // | bigint                 | int8             | signed eight-byte integer  |
    Int64(i64),

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
    VarChar(str),

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
    Int32(i32),

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
    // Serial(i32),
    // rhodesd> This is just a (rust int32| sql int4)
    // todo: possible to discriminate between the two by inspecting PQresult?

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
    PgDataErr(str),
}

iface Show {
    fn show() -> str;
}

impl of Show for PgData {
    fn show() -> str {
        alt self {
          Int64(n) {"Asdf"}
          VarChar(s) {#fmt("'%s'", s)}
          Int32(n) {#fmt("%d", n as int)}
          _ { "asdf"}
        }
    }
}

type Row = [PgData];

// There's got to be a better way. list comprehensions?
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

unsafe fn ResultField2PGdata(res: Result, tup: int,  fld: int) -> PgData {
    // get the field type
    assert res.Ok(); // todo: this check needs to be gone at some point
    let getrep = {|t,f| unsafe::from_c_str(res.GetValue(t,f))};
    
    alt res.Ftype(fld) {
      // -------------------------------------------------------
      BOOL { 
        let val = res.GetValue(tup, fld) as *c_int;
        Bool(
            // todo: setup a test case for this.
            alt *val {
                0 { false }
                1 { true }
                _ { fail("Need to figure out how postgres implements bools") }
            })
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
      VARCHAR {
        VarChar(unsafe::from_c_str(res.GetValue(tup, fld)))
      }
      // DATE {}
      // TIME {}
      // TIMESTAMP {}
      // TIMESTAMPTZ {}
      // INTERVAL {}
      // TIMETZ {}
      // BIT {}
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

unsafe fn GetRows(res: Result) -> [PgData] {
    // log(error, res.Ntuples());
    if res.Ok() {
        let flds = seq(0, res.Nfields());
        let f = {|x|ResultField2PGdata(res, 0, x)};
        let rs = vec::map(flds, f);
        rs

    } else {
        [PgDataErr("GetRows fails because ... ")]
    }
}

type Movie = {
    // Maybe there's a way to strengthen the types
    did: PgData,
    title: PgData,
    year: PgData,
    director: PgData
};

#[test]
fn ResultTest() {
    let conn = TestConnect();

    let r0 = conn.Exec("drop table if exists movie");
    if !r0.Ok() {
        log(error, r0.Status());
        log(error, "status:    " + r0.StatusAsStr());
        log(error, "error msg: " + r0.ErrorMessage());   
    }

    let r1 = conn.Exec("create table movie (\
                        did serial,\
                        unique(did),\
                        title varchar(255),\
                        year int,\
                        director varchar(255)\
                        );"
                      );

    assert r1.Ok();
    //log(error, "error msg: " + r1.ErrorMessage());

    let insertstr = "insert into movie (title, year, director) VALUES";
    conn.Exec( insertstr + "('a new hope', 1977, 'lucas')");
    conn.Exec( insertstr + "('the empire strikes back', 1980, 'Kershner')");
    conn.Exec( insertstr + "('return of the jedi', 1983, 'lucas')");

    log(error, "-------------------------------------------------------");
    let res = conn.Exec("select * from movie");

    unsafe {
        let rows = GetRows(res);
        log(error, rows);
        log(error, conn.StatusAsStr());
        assert rows == [
            Int32(1), 
            VarChar("a new hope"), 
            Int32(1977), 
            VarChar("lucas")
        ];
    }
    conn.Exec("drop table if exists movie");
}
