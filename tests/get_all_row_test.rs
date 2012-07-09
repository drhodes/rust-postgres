import test_basic::*;
import glue::*;

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
