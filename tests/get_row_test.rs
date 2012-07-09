import test_basic::*;
import glue::*;

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
