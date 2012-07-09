import test_basic::*;
import glue::*;

// ------------------------------------------------------------------
// this is vestigial, it exists, but do not look at it, like
// Indiana Jones, Raiders of the Lost Ark.
// Your face will melt and demon rays will permeate space within
// a radius of 500 meters.

#[test]
fn FaceMelt1() {
    type Movie = {
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
