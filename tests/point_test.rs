import test_basic::*;
import glue::*;

#[test]
fn TestPoint() {
    let conn = TestConnect();

    let testpoint = Table("test_point", [
        ("did",        [Unique], SerialM),
        ("point_field", [Insert], PointM),
    ]);

    // Insert
    Assure( testpoint.DropTable(conn));
    Assure( testpoint.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testpoint.Insert([
        Point(3f,4f)
    ]);

    assert q == "insert into test_point (point_field) VALUES ('(3,4)')";
    Assure(conn.Exec(q));
    
    // Select
    let s = "select * from test_point";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);    

    alt row[1] {
      Point(x,y) {
        assert x == 3f;
        assert y == 4f;
      }
      _ { 
        log(error, "Point Error--------------------------------------------");
        log(error, row);
        fail("pq: wrong Point value from db");
      }
    }
}
