import test_basic::*;
import glue::*;


#[test]
fn TestDate() {
    let conn = TestConnect();

    let testdate = Table("test_date", [
        ("did",        [Unique], SerialM),
        ("date_field", [Insert], DateM),
    ]);

    // Insert
    Assure( testdate.DropTable(conn));
    Assure( testdate.CreateTable(conn));
    
    // exclude the "did" field because it's does not have Insert attribute
    let q = testdate.Insert([
        Date(2012, 4, 1)
    ]);

    assert q == "insert into test_date (date_field) VALUES ('2012,4,1')";
    Assure(conn.Exec(q));
    
    // Select
    let s = "select * from test_date";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);    

    alt row[1] {
      Date(y,m,d) {
        assert y == 2012;
        assert m == 4;
        assert d == 1;
      }
      _ { 
        log(error, "Date Error--------------------------------------------");
        log(error, row);
        fail("pq: wrong Date value from db");
      }
    }
}
