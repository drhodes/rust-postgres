/*
import std::time;
import result::{result, ok, err};
import libc::*;
import glue::*;
import test_basic::*;


// -----------------------------------------------------------------------------
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
    Assure( person.DropTable(conn));
    Assure( movie.DropTable(conn));

    // Create some tables
    Assure( person.CreateTable(conn));
    Assure( movie.CreateTable(conn));

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
        let q2_ = "insert into movie_algebra (title,year,director) VALUES ('starwars',1977,1)";
        assert q2 == q2_;
            
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
          NullMovie { fail("From title fails") }
        }
    }
    Assure(conn.Exec("drop table if exists movie4"));
}
*/
