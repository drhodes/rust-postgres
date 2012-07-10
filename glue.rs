import std::time;
import result::{result, ok, err};
import libc::*;
import vec::len;


enum IP {
    IPv4(u8,u8,u8,u8),
    IPv6(u16,u16,u16,u16,u16,u16,u16),
}

#[doc = "PgData are rust representations of postgres data  \
         There are two constructors for each postgres type \
         Data: e.g. VarChar(<string data>) 
           This is the rust side data used in your program.

         Model: e.g. VarCharM(255)
           These have a trailing 'M' and are used to build \
           Table instances that can be conveniently inserted. \
         
         The other constructors are used by the library to supply well typed \
         rust data, so there's no need to run scan functions after a query. \
         
         It may be prudent to separate these two types.\

         todo: improve this docstring. \
         "]

enum PgData {
    // +------------------------------------------------------------------------+
    // |          Name          |     Aliases      |        Description         |
    // |------------------------+------------------+----------------------------|
    // | bigint                 | int8             | signed eight-byte integer  |
    Int64(i64),
    Int64M,

    // |------------------------+------------------+----------------------------|
    // | bigserial              | serial8          | autoincrementing           |
    // |                        |                  | eight-byte integer         |
    BigSerial(i64),
    BigSerialM,

    // |------------------------+------------------+----------------------------|
    // | bit [ (n) ]            |                  | fixed-length bit string    |
    Bit([bool]),
    BitM(int),

    // |------------------------+------------------+----------------------------|
    // | bit varying [ (n) ]    | varbit           | variable-length bit string |
    VarBit([bool]),
    VarBitM(int),

    // |------------------------+------------------+----------------------------|
    // | boolean                | bool             | logical Boolean            |
    // |                        |                  | (true/false)               |
    Bool(bool),
    BoolM,

    // |------------------------+------------------+----------------------------|
    // | box                    |                  | rectangular box on a plane |
    //Box(f64, f64),    

    // |------------------------+------------------+----------------------------|
    // | bytea                  |                  | binary data ("byte array") |
    ByteA([u8]),
    ByteAM,

    // |------------------------+------------------+----------------------------|
    // | character varying [    | varchar [ (n) ]  | variable-length character  |
    // | (n) ]                  |                  | string                     |
    VarCharM(int), // model
    VarChar(str),

    // |------------------------+------------------+----------------------------|
    // | character [ (n) ]      | char [ (n) ]     | fixed-length character     |
    // |                        |                  | string                     |
    Char(str),
    CharM(int),

    // |------------------------+------------------+----------------------------|
    // | cidr                   |                  | IPv4 or IPv6 network       |
    // |                        |                  | address                    |
    Cidr(IP),
    CidrM,

    // |------------------------+------------------+----------------------------|
    // | circle                 |                  | circle on a plane          |
    //Circle(),
    //CircleM,

    // |------------------------+------------------+----------------------------|
    // | date                   |                  | calendar date (year,       |
    // |                        |                  | month, day)                |
    Date(u16,u8,u8),
    DateM,

    // |------------------------+------------------+----------------------------|
    // |                        |                  | double precision           |
    // | double precision       | float8           | floating-point number (8   |
    // |                        |                  | bytes)                     |
    Float64(f64),
    Float64M,

    // |------------------------+------------------+----------------------------|
    // | inet                   |                  | IPv4 or IPv6 host address  |
    Inet(IP),
    InetM,

    // |------------------------+------------------+----------------------------|
    // | integer                | int, int4        | signed four-byte integer   |
    Int32(i32),
    Int32M,

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

    // |------------------------+------------------+----------------------------|
    // | money                  |                  | currency amount            |
    // |------------------------+------------------+----------------------------|
    // | numeric [ (p, s) ]     | decimal [ (p, s) | exact numeric of           |
    // |                        | ]                | selectable precision       |
    // |------------------------+------------------+----------------------------|
    // | path                   |                  | geometric path on a plane  |
    // |------------------------+------------------+----------------------------|
    // | point                  |                  | geometric point on a plane |
    Point(float, float),
    PointM,

    // |------------------------+------------------+----------------------------|
    // | polygon                |                  | closed geometric path on a |
    // |                        |                  | plane                      |
    // |------------------------+------------------+----------------------------|
    // |                        |                  | single precision           |
    // | real                   | float4           | floating-point number (4   |
    // |                        |                  | bytes)                     |
        Float32(f32),
    Float32M,

    // |------------------------+------------------+----------------------------|
    // | smallint               | int2             | signed two-byte integer    |
    Int16(i16),
    Int16M,

    // |------------------------+------------------+----------------------------|
    // | serial                 | serial4          | autoincrementing four-byte |
    // |                        |                  | integer                    |
    Serial(i32),
    SerialM,

    // rhodesd> This is just a (rust int32| sql int4)
    // todo: possible to discriminate between the two by inspecting PQresult?
    // magic 8 ball says: Not likely.


    // |------------------------+------------------+----------------------------|
    // | text                   |                  | variable-length character  |
    // |                        |                  | string                     |
    Text(str),
    TextM,

    // |------------------------+------------------+----------------------------|
    // | time [ (p) ] [ without |                  | time of day (no time zone) |
    // | time zone ]            |                  |                            |
    Time(time::tm),
    TimeM,

    // |------------------------+------------------+----------------------------|
    // | time [ (p) ] with time | timetz           | time of day, including     |
    // | zone                   |                  | time zone                  |
    TimeTz(time::tm),
    TimeTzM,

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
    Uuid([u8]/16),
    UuidM,

    // |------------------------+------------------+----------------------------|
    // | xml                    |                  | XML data                   |
    Xml([u8]),
    XmlM,
    // +------------------------------------------------------------------------+
    PgDataErr(str),
}


iface Show {
    fn show() -> str;
}

impl of Show for PgData {   
    fn show() -> str {
        alt self {
          // ------------------------------------------------------------------
          Int16(n) {
            #fmt("%d", n as int)
          }
          Int16M { 
            "INT2" 
          }
          // ------------------------------------------------------------------         
          Int32(n) {
            #fmt("%d", n as int)
          }
          Int32M {
            "INT4" 
          }
          // ------------------------------------------------------------------
          Int64(n) {
            i64::to_str(n, 10)
          }
          Int64M {
            "INT8" 
          }
          // ------------------------------------------------------------------          
          SerialM {
            "SERIAL" 
          }
          // ------------------------------------------------------------------          
          VarChar(s) {
            #fmt("'%s'", s)
          }
          VarCharM(n) {
            #fmt("VARCHAR(%d)", n)
          }
          // ------------------------------------------------------------------        
          BigSerial(n) {
            i64::to_str(n, 10)
          }
          BigSerialM {
            "BIGSERIAL"
          }
          // ------------------------------------------------------------------         
          Bit(bs) {
            let bvec = bs.map({|b| if b { "1" } else { "0" }});
            let bstr = str::connect(bvec, "");
            #fmt("B'%s'", bstr)
          }          
          BitM(n) {
            #fmt("BIT(%d)", n) 
          }
          // ------------------------------------------------------------------        
          Bool(b) { 
            if b == true {
                "TRUE"
            } else {
                "FALSE"}
          }
          BoolM {
            "BOOL"
          }

          // ByteA([u8]),
          // ByteAM,

          // ------------------------------------------------------------------
          Char(s) {
            #fmt("'%s'", s)
          }
          CharM(n) {
            #fmt("CHAR(%d)", n)
          }

          // ------------------------------------------------------------------
          // Cidr(IP),
          // CidrM
          
          Date(y,m,d) {
            // Y,M,D
            #fmt("'%?,%?,%?'", y,m,d)
          }
          DateM { "DATE" }

          // ------------------------------------------------------------------
          Float32(n) {
            float::to_str(n as float, 10) // todo: (as float) is bad.
                // preserve all the bits
          }
          Float32M {"FLOAT4"}
          // ------------------------------------------------------------------
          Float64(n) {
            float::to_str(n as float, 10) // todo: (as float) is bad. need to
                // preserve all the bits
          }
          Float64M {"FLOAT8"}
          // ------------------------------------------------------------------

          // Inet(IP),
          // InetM,


          // ------------------------------------------------------------------
          MacAddr(a,b,c,d,e,f) {
            #fmt("'%02x:%02x:%02x:%02x:%02x:%02x'", 
                 a as uint,
                 b as uint,
                 c as uint,
                 d as uint,
                 e as uint,
                 f as uint
                )                
          }
          MacAddrM {
            "MACADDR"
          }

          // Serial(i32),
          // SerialM,
         
          // ------------------------------------------------------------------
          Text(s) {
            #fmt("'%s'", s)
          }
          TextM {
            "TEXT"
          }

          // Time(time::tm),
          // TimeM

          // TimeTz(time::tm),
          // TimeTzM

          // ------------------------------------------------------------------
          Uuid(xs) { 
            // A0EEBC99-9C0B-4EF8-BB6D-6BB9BD380A11
            let mut us: [uint] = [];
            for xs.each |x| {
                us += [x as uint];
            }
            
            #fmt("'\
                  %02x%02x%02x%02x-\
                  %02x%02x-\
                  %02x%02x-\
                  %02x%02x-\
                  %02x%02x%02x%02x%02x%02x\
                  '", 
                 us[0], us[1], us[2], us[3],
                 us[4], us[5],
                 us[6], us[7],
                 us[8], us[9],
                 us[10], us[11], us[12], us[13], us[14], us[15])
          }          
          
          UuidM {
            "UUID"
          }         

          // ------------------------------------------------------------------
          VarBit(bs) {
            let bvec = bs.map({|b| if b { "1" } else { "0" }});
            let bstr = str::connect(bvec, "");
            #fmt("B'%s'", bstr)
          }
          VarBitM(n) { #fmt("VARBIT(%d)", n) }

          // ------------------------------------------------------------------
          Point(x,y) {
            #fmt("'(%f,%f)'", x, y)
          }
          PointM {
            "POINT"
          }


          // Xml([u8]),
          // XmlM.

          // PgDataErr(str),
          
          _ {"ASDF"}
          
        }
    }
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

      // -------------------------------------------------------
      // this is the ftype for CHAR. CHAR is a synonym for BPCHAR
      BPCHAR {
        let s = getrep(tup, fld);
        Char(s)
      }

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

      // -------------------------------------------------------
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
        Text(getrep(tup, fld))
      }

      // OID {}
      // TID {}
      // XID {}
      // CID {}
      // VECTOR {}
      // JSON {}
      // XML {}
      // PGNODETREE {}

      // -------------------------------------------------------
      POINT {
        let s = getrep(tup, fld);
        let parts = str::split_char(str::slice(s, 1, str::len(s)-1), ',');

        alt float::from_str(parts[0]) {
          some(x) { 
            alt float::from_str(parts[1]) {
              some(y) { Point(x,y) }
              none { PgDataErr("error in POINT.y, can't parse " + s) }
            }
          }
          none { PgDataErr("error in POINT.x, can't parse " + s) }
        }
      }                                             
      // -------------------------------------------------------
      // LSEG {}
      // PATH {}
      // BOX {}
      // POLYGON {}
      // LINE {}
      

      // -----------------------------------------------------------
      FLOAT4 {
        let s = getrep(tup, fld);
        alt int::from_str(s) {
          none {
            PgDataErr("error in FLOAT4, can't parse " + s) 
          }
          some(n) {
            Float32(n as f32)
          }                
        }        
      }

      // -----------------------------------------------------------
      FLOAT8 {
        let s = getrep(tup, fld);
        alt int::from_str(s) {
          none {
            PgDataErr("error in FLOAT8, can't parse " + s) 
          }
          some(n) {
            Float64(n as f64)
          }                
        }        
      }

      // ABSTIME {}
      // RELTIME {}
      // TINTERVAL {}
      // UNKNOWN {}
      // CIRCLE {}

      // CASH {}
      
      // -------------------------------------------------------
      MACADDR {
        let s = getrep(tup, fld);
        let hexes = str::split_char(s, ':');

        let mut ns: [u8] = [];
        for hexes.each |x| {
            ns += [chars2hex(x[0] as char, x[1] as char)];
        }
        MacAddr(ns[0], ns[1], ns[2], ns[3], ns[4], ns[5])
      }

      // -------------------------------------------------------
      INET {
        Inet(IPv6(0,0,0,0,0,0,0))
      }

      // -------------------------------------------------------
      UUID { 
        let s = getrep(tup, fld);
        let parts = str::split_char(s, '-');
        let whole = str::connect(parts, "");      
        let pairs = couples(whole);
      
        // would like to do this.
        // let ns = pairs.map({|x| 
        //     chars2hex(x[0] as char, x[1] as char)
        // });

        let mut ns: [u8] = [];
        for pairs.each |x| {
            vec::push(ns, chars2hex(x[0] as char, x[1] as char));
        }
        Uuid([ns[0],ns[1],ns[2],ns[3],
              ns[4],ns[5],ns[6],ns[7],
              ns[8],ns[9],ns[10],ns[11],
              ns[12],ns[13],ns[14],ns[15]]/16)
      }

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

      // -------------------------------------------------------
      DATE {
        let s = getrep(tup, fld);
        let trip = str::split_char(s, '-');
        if len(trip) != 3 {
            ret PgDataErr("Expecting dash delimited date, got: " + s);
        }
        
        alt int::from_str(trip[0]) {
          some(y) {            
            alt int::from_str(trip[1]) {
              some(m) {
                alt int::from_str(trip[2]) {
                  some(d) {
                    ret Date(y as u16, m as u8, d as u8);
                  }
                  none {} } }
              none {} } }
          none {}             
        }
        ret PgDataErr("Date parse error: " + s);
      }
      
      // TIME {}
      // TIMESTAMP {}
      // TIMESTAMPTZ {}
      // INTERVAL {}
      // TIMETZ {}

      // -------------------------------------------------------
      BIT {
        let s = getrep(tup, fld);
        Bit(str::chars(s).map({|x| x == '1'}))
      }

      // -------------------------------------------------------
      VARBIT {
        let s = getrep(tup, fld);
        VarBit(str::chars(s).map({|x| x == '1'}))        
      }


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
      _ { 
        log(error, getrep(tup, fld));
        PgDataErr("Unrecognized Ftype") 
      }
      
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
    fn ShowCreate() -> str;
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
    
    fn ShowCreate() -> str {
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
        q
    }


    fn CreateTable(conn: Conn) -> Result {
        conn.Exec(self.ShowCreate())
            /*
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
            */
    }    
}


// util ------------------------------------------------------------------

fn pair<T: copy>(xs: [T]) -> [(T,T)] {
    if len(xs) == 0 {
        []
    } else {
        [(xs[0], xs[1])] + pair(xs.slice(2, len(xs)))
    }
}

#[test] 
fn pair_test() {
    assert pair([1,2,3,4]) == [(1,2), (3,4)];
}

fn couples(s: str) -> [str] {    
    if str::len(s) < 2 {
        []
    } else {
        let rest = couples(str::slice(s, 2, str::len(s)));
        [str::slice(s, 0, 2)] + rest
    }    
}

#[test] 
fn couples_test() {
    assert couples("1234") == ["12", "34"];
}

fn chars2hex(c1: char, c2: char) -> u8 {
    16 * char2hex(c1) + char2hex(c2)
}

fn char2hex(c: char) -> u8 {
    alt c {
      '0' {0}
      '1' {1}
      '2' {2}
      '3' {3}
      '4' {4}
      '5' {5}
      '6' {6}
      '7' {7}
      '8' {8}
      '9' {9}
      'a' {10}
      'b' {11}
      'c' {12}
      'd' {13}
      'e' {14}
      'f' {15}
      _ { 
        let msg = #fmt["char2hex must hexdigit char [0-9|a-f], got: %c", c];
        fail(msg)
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


fn to_u8(x: str) -> u8 {
    chars2hex(x[0] as char, x[1] as char)
}


