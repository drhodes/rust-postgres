import test_basic::*;
import glue::*;

#[test]
fn TestChar() {
    let conn = TestConnect();

    let testchar = Table("test_char", [
        ("did",        [Unique], SerialM),
        ("char_field", [Insert], CharM(9)),
    ]);

    // Insert
    Assure( testchar.DropTable(conn));
    Assure( testchar.CreateTable(conn));
    
    log(error, testchar.ShowCreate());

    // exclude the "did" field because it's does not have Insert attribute
    let q = testchar.Insert([
        Char("TEST_CHAR")
    ]);

    log(error, q);
    assert q == "insert into test_char (char_field) VALUES ('TEST_CHAR')";
    Assure(conn.Exec(q));
    
    // Select
    let s = "select * from test_char";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);    

    alt row[1] {
      Char(n) {
        assert n == "TEST_CHAR";
      }
      _ { 
        log(error, "Char Error--------------------------------------------");
        log(error, row);
        fail("pq: wrong Char value from db");
      }
    }
}
