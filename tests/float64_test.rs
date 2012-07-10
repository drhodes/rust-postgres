import test_basic::*;
import glue::*;

#[test]
fn TestFloat64() {
    let conn = TestConnect();

    let testfloat64 = Table("testfloat64", [
        ("did",        [Unique], SerialM),
        ("float64_field", [Insert], Float64M),
    ]);

    // Insert
    Assure( testfloat64.DropTable(conn));
    Assure( testfloat64.CreateTable(conn));
    
    // exclude the "did" field because it's does not have Insert attribute
    let q = testfloat64.Insert([
        Float64(1234.00f64)
    ]);

    log(error, q);
    assert q == "insert into testfloat64 (float64_field) VALUES (1234)";
    Assure(conn.Exec(q));
    
    // Select
    let s = "select * from testfloat64";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);    

    alt row[1] {
      Float64(n) {
        assert n == 1234f64;
      }
      _ { 
        log(error, "Float64 Error--------------------------------------------");
        log(error, row[1]);
        fail("pq: wrong Float64 value from db");
      }
    }
}
