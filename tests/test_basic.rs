import test_basic::*;
import glue::*;

// ------------------------------------------------------------------
// #[test]
fn TestConnect() -> Conn {
    let connstr = "host=localhost \
                   port=5432 \
                   user=rustuser \
                   dbname=rust_test_db \
                   password=rustpass \
                   sslmode=disable";

    let conn = Conn(connstr);
    if !conn.Ok() {
        log(error, conn.StatusAsStr());
        log(error, conn.LibVersion());
        log(error, conn.ServerVersion());
        log(error, conn.ProtocolVersion());
    }
    assert conn.Db() == "rust_test_db";
    assert conn.User() == "rustuser";
    assert conn.Pass() == "rustpass";
    assert conn.Host() == "localhost";
    assert conn.Port() == "5432";
    ret conn
}

#[test]
fn ResultTest() {
    let conn = TestConnect();    
    Assure(conn.Exec("drop table if exists movie1"));
    Assure(conn.Exec("create table movie1 (\
                        did serial,\
                        unique(did),\
                        title varchar(255),\
                        year int,\
                        director varchar(255)\
                        );"
                      ));

    let insertstr = "insert into movie1 (title, year, director) VALUES";
    Assure(conn.Exec( insertstr + "('a new hope', 1977, 'lucas')"));
    Assure(conn.Exec( insertstr + "('the empire strikes back', 1980, 'Kershner')"));
    Assure(conn.Exec( insertstr + "('return of the jedi', 1983, 'lucas')"));

    let res = Assure(conn.Exec("select * from movie1"));
    assert res.Ntuples() == 3;
    assert res.Nfields() == 4; // why 4 && not 3? primary key isn't in insert query
    
    // log(error, res.BinaryTuples());
    // log(error, res.Fname(1));
    // log(error, res.Ftable(1));
    // log(error, res.Ftablecol(1));
    // log(error, res.Fformat(1));
    // log(error, res.Fsize(1));
    // log(error, res.Fmod(1));
    // log(error, "CmdStatus: " + res.CmdStatus());
    // log(error, res.OidValue());
    // log(error, res.CmdTuples());
    // log(error, res.GetValue(1,1));
    // log(error, res.GetLength(1,1));
    // log(error, res.GetIsNull(1,1));
    // log(error, res.Nparams());
    // log(error, res.ErrorField(1));
    conn.Exec("drop table if exists movie1");
}

// #define PQsetdb(M_PGHOST,M_PGPORT,M_PGOPT,M_PGTTY,M_DBNAME)  \
//  PQsetdbLogin(M_PGHOST, M_PGPORT, M_PGOPT, M_PGTTY, M_DBNAME, NULL, NULL)
