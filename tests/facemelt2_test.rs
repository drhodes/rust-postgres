import test_basic::*;
import glue::*;

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
