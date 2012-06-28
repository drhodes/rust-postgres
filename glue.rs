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
    NullInt64,

    // |------------------------+------------------+----------------------------|
    // | bigserial              | serial8          | autoincrementing           |
    // |                        |                  | eight-byte integer         |
    BigSerial(i64),
    NullBigSerial,

    // |------------------------+------------------+----------------------------|
    // | bit [ (n) ]            |                  | fixed-length bit string    |
    Bit([bool]),
    NullBit,

    // |------------------------+------------------+----------------------------|
    // | bit varying [ (n) ]    | varbit           | variable-length bit string |
    VarBit([bool]),
    NullVarBit,

    // |------------------------+------------------+----------------------------|
    // | boolean                | bool             | logical Boolean            |
    // |                        |                  | (true/false)               |
    Bool(bool),
    NullBool,

    // |------------------------+------------------+----------------------------|
    // | box                    |                  | rectangular box on a plane |
    //Box(f64, f64),

    // |------------------------+------------------+----------------------------|
    // | bytea                  |                  | binary data ("byte array") |
    ByteA([u8]),
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
    NullChar,

    // |------------------------+------------------+----------------------------|
    // | cidr                   |                  | IPv4 or IPv6 network       |
    // |                        |                  | address                    |
    Cidr(IP),
    NullCidr,

    // |------------------------+------------------+----------------------------|
    // | circle                 |                  | circle on a plane          |
    // |------------------------+------------------+----------------------------|
    // | date                   |                  | calendar date (year,       |
    // |                        |                  | month, day)                |
    Date(u8,u8,u8),
    NullDate,

    // |------------------------+------------------+----------------------------|
    // |                        |                  | double precision           |
    // | double precision       | float8           | floating-point number (8   |
    // |                        |                  | bytes)                     |
    Float64(f64),
    NullFloat64,

    // |------------------------+------------------+----------------------------|
    // | inet                   |                  | IPv4 or IPv6 host address  |
    Inet(IP),
    NullInet,

    // |------------------------+------------------+----------------------------|
    // | integer                | int, int4        | signed four-byte integer   |
    Int32M,
    Int32(i32),
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
    NullFloat32,

    // |------------------------+------------------+----------------------------|
    // | smallint               | int2             | signed two-byte integer    |
    Int16(i16),
    NullInt16,

    // |------------------------+------------------+----------------------------|
    // | serial                 | serial4          | autoincrementing four-byte |
    // |                        |                  | integer                    |
    Serial(i32),
    SerialM,
    // rhodesd> This is just a (rust int32| sql int4)
    // todo: possible to discriminate between the two by inspecting PQresult?

    // |------------------------+------------------+----------------------------|
    // | text                   |                  | variable-length character  |
    // |                        |                  | string                     |
    Text(str),
    NullText,

    // |------------------------+------------------+----------------------------|
    // | time [ (p) ] [ without |                  | time of day (no time zone) |
    // | time zone ]            |                  |                            |
    Time(time::tm),
    NullTime,

    // |------------------------+------------------+----------------------------|
    // | time [ (p) ] with time | timetz           | time of day, including     |
    // | zone                   |                  | time zone                  |
    TimeTz(time::tm),
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
    NullUuid,
    // |------------------------+------------------+----------------------------|
    // | xml                    |                  | XML data                   |

    Xml([u8]),
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
          Int64(n) {#fmt("%d", n as int)} // todo: how to format a i64?
          VarChar(s) {#fmt("'%s'", s)}
          Int32(n) {#fmt("%d", n as int)}
          VarCharM(n) {#fmt("VARCHAR(%d)", n)}
          Int32M { "INT4" }
          SerialM { "SERIAL" }
          _ {"ASDF"}

        }
    }
}


// There's got to be a better way. list comprehensions?
fn seq(a: int, b: int) -> [int] {
    // haskell? [a .. b] done.
    let mut i = a;
    let mut accum: [int] = [];
    //int::range(a,b,{|x| vec::push(accum, x)});
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

unsafe fn GetRow(res: Result, rownum: int) -> [PgData] {
    if res.Ok() {
        let flds = seq(0, res.Nfields());
        let f = {|x|ResultField2PGdata(res, rownum, x)};
        let rs = vec::map(flds, f);
        rs

    } else {
        [PgDataErr("GetRows fails because ... ")]
    }
}

unsafe fn GetAllRows(res: Result) -> [[PgData]] {    
    if res.Ok() {
        let tups = seq(0, res.Ntuples());
        let f = {|x|GetRow(res, x)};
        let rs = vec::map(tups, f);
        rs
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


#[test]
fn GetRowTest() {
    let conn = TestConnect();

    Assure(conn.Exec("drop table if exists movie"));

    Assure(conn.Exec("create table movie (\
                      did serial,\
                      unique(did),\
                      title varchar(255),\
                      year int,\
                      director varchar(255)\
                      );"
                    ));

    InsertStarWars(conn, "movie");
    let res = Assure(conn.Exec("select * from movie"));

    unsafe {
        assert GetRow(res, 0) == [
            Int32(1), 
            VarChar("a new hope"), 
            Int32(1977), 
            VarChar("lucas")
        ];
    }
    conn.Exec("drop table if exists movie");
}

// ------------------------------------------------------------------

type Row = [PgData];

enum FieldOption {
    Insert,
    Select,
    Unique,

    // table types here may preclude mutual recursive tables.
    // meh, we'll do it live. WE'LL DO IT LIVE.
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

iface Insert {
    fn Insert(Row) -> str;
}

impl of Insert for Table {
    fn Insert(r: Row) -> str {
        let mut keepers: [str] = [];
        let Table(tname, flds) = copy self;

        for flds.each {|x|
            if InsertFieldP(x) {
                let (fldname, _, _) = copy x;
                keepers += [fldname];
            }
        }
        if r.len() != keepers.len() {
            fail("Table must have the same number of Insert fields as values")
        }

        let mut vals: [str] = [];
        for r.each {|fld|
            vals += [fld.show()];
        }

        #fmt["insert into %s %s VALUES %s", tname, 
             "("+str::connect(keepers, ",")+")",
             "("+str::connect(vals,",")+")"]             
    }    
}



fn DropTable(tbl: Table) -> str {
    let Table(name, _) = copy tbl;
    #fmt("drop table if exists %s cascade", name)
}


fn CreateTable(tbl: Table) -> str {
    let Table(name, flds) = copy tbl;
    let mut q: [str] = [];
       
    for flds.each {|fld|
        let (fname, ops, modt) = copy fld;
        q += [#fmt["  %s %s", fname, modt.show()]];
        for ops.each {|op|
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
    head + str::connect(q, ",\n") + "\n)"
}


#[test]
fn AlgebraTest() {
    let conn = TestConnect();

    let person = Table("person", [
        ("did",    [Unique], SerialM),
        ("name",   [Insert], VarCharM(255))
    ]);
    
    let movie = Table("movie_algebra", [
        ("did",      [Unique],                                 SerialM),
        ("title",    [Insert],                                 VarCharM(255)),
        ("year",     [Insert],                                 Int32M),
        ("director", [Insert, ForeignKey(copy person, "did")], Int32M )
    ]);

    // Drop some tables
    Assure( conn.Exec( DropTable( person)));
    Assure( conn.Exec( DropTable( movie)));

    // Create some tables
    Assure( conn.Exec( CreateTable( person)));
    Assure( conn.Exec( CreateTable( movie)));

    // Test the fancy insert.
    let q1 = person.Insert( [VarChar("lucas")] );
    assert q1 == "insert into person (name) VALUES ('lucas')";
    Assure(conn.Exec(q1));

    // no fancy select yet.
    // Get the person.did of lucas to test foreign key insert

    unsafe {
        let res = Assure(conn.Exec("select * from person where name = 'lucas'"));
        let lucas_did = copy GetRow(res, 0)[0];
        
        let q2 =  movie.Insert( [VarChar("starwars"), Int32(1977), lucas_did]);
        assert q2 ==
            "insert into movie_algebra (title,year,director) VALUES ('starwars',1977,1)";
        Assure(conn.Exec(q2));
    }
}


// ------------------------------------------------------------------
#[test]
fn GetAllRowTest() {
    let conn = TestConnect();
    Assure(conn.Exec("drop table if exists movie2"));
    Assure(conn.Exec("create table movie2 (\
                      did serial,\
                      unique(did),\
                      title varchar(255),\
                      year int,\
                      director varchar(255)\
                      );"
                      ));

    InsertStarWars(conn, "movie2");
    let res = Assure(conn.Exec("select * from movie2"));

    unsafe {
        let rows = GetAllRows(res);
        assert rows == [
            [Int32(1), 
             VarChar("a new hope"), 
             Int32(1977), 
             VarChar("lucas"),
            ],
            [Int32(2), 
             VarChar("the empire strikes back"), 
             Int32(1980), 
             VarChar("Kershner")
            ],
            [Int32(3), 
             VarChar("return of the jedi"), 
             Int32(1983), 
             VarChar("lucas")
            ],            
        ];
    }
    conn.Exec("drop table if exists movie2");
}

// ------------------------------------------------------------------
#[test]
fn UseCase1() {
    type Movie = {
        // Maybe there's a way to strengthen the types
        did: PgData,
        title: PgData,
        year: PgData,
        director: PgData
            
    };
    
    let conn = TestConnect();
    Assure(conn.Exec("drop table if exists movie3"));
    Assure(conn.Exec("\
                      create table movie3 (\
                      did serial,\
                      unique(did),\
                      title varchar(255),\
                      year int,\
                      director varchar(255)\
                      );"
                      ));


    InsertStarWars(conn, "movie3");

    unsafe fn MovieFromTitle(c: Conn, title: str) -> Movie {
        let q = #fmt("select * from movie3 where title = '%s'", title);
        let res = Assure(c.Exec(q));
        let row = GetRow(res, 0);
        let mov = { 
            did:copy row[0],
            title:copy row[1],
            year:copy row[2],
            director:copy row[3]
        }; 
        //log(error, mov);
        ret mov

    }
    unsafe {
        assert MovieFromTitle(conn, "a new hope") == {
            did:Int32(1), 
            title:VarChar("a new hope"), 
            year:Int32(1977), 
            director:VarChar("lucas"),
        }
    }

    Assure(conn.Exec("drop table if exists movie3"));
}

// -----------------------------------------------------------------------------
#[test]
fn UseCase2() {
    enum Movie {
        Movie([PgData]),
        NullMovie
    };
    
    iface MovieI {        
        unsafe fn FromTitle(Conn, str) -> Movie;
    }

    impl MovieI for Movie {        
        unsafe fn FromTitle(c: Conn, title: str) -> Movie {
            let res = Assure(c.Exec(#fmt("select * from movie4 where title = '%s'", title)));
            let row = GetRow(res, 0);
            Movie(row)
        }
    }

    let conn = TestConnect();
    Assure(conn.Exec("drop table if exists movie4"));
    Assure(conn.Exec("\
                      create table movie4 (\
                      did serial,\
                      unique(did),\
                      title varchar(255),\
                      year int,\
                      director varchar(255)\
                      );"
                    ));
    
    InsertStarWars(conn, "movie4");
    unsafe {
        alt NullMovie.FromTitle(conn, "a new hope") {
          Movie(rs) { assert rs == [Int32(1),
                                    VarChar("a new hope"),
                                    Int32(1977),
                                    VarChar("lucas")
                                   ]}          
          NullMovie { log(error, "From title fails") }
        }
    }
    Assure(conn.Exec("drop table if exists movie4"));

}


