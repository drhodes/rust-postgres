import test_basic::*;
import glue::*;

#[test]
fn TestFloat32() {
    let conn = TestConnect();

    let testfloat32 = Table("testfloat32", [
        ("did",        [Unique], SerialM),
        ("float32_field", [Insert], Float32M),
    ]);

    // Insert
    Assure( testfloat32.DropTable(conn));
    Assure( testfloat32.CreateTable(conn));
    
    // exclude the "did" field because it's does not have Insert attribute
    let q = testfloat32.Insert([
        Float32(16.00f32)
    ]);

    log(error, q);
    assert q == "insert into testfloat32 (float32_field) VALUES (16)";
    Assure(conn.Exec(q));
    
    // Select
    let s = "select * from testfloat32";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);    

    alt row[1] {
      Float32(n) {
        assert n == 16f32;
      }
      _ { 
        log(error, "Float32 Error--------------------------------------------");
        log(error, row[1]);
        fail("pq: wrong Float32 value from db");
      }
    }
}
