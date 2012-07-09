import test_basic::*;
import glue::*;

#[test]
fn MovieTest() {
    let conn = TestConnect();

    let person = Table("person", [
        ("did",    [Unique], SerialM),
        ("name",   [Insert], VarCharM(255))
    ]);
    
    let movie = Table("movie_alg", [
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
        let q2_ = "insert into movie_alg (title,year,director) VALUES ('starwars',1977,1)";
        assert q2 == q2_;
            
        Assure(conn.Exec(q2));
    }
}
