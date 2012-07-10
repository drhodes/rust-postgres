import test_basic::*;
import glue::*;

#[test]
fn TestInt32() {
    let conn = TestConnect();

    let testint32 = Table("testint32", [
        ("did",        [Unique], SerialM),
        ("int32field", [Insert], Int32M),
    ]);

    // Insert
    Assure( testint32.DropTable(conn));
    Assure( testint32.CreateTable(conn));

    // exclude the "did" field because it's does not have Insert attribute
    let q = testint32.Insert([
        Int32(1<<20) 
    ]);

    assert q == "insert into testint32 (int32field) VALUES (1048576)";    
    Assure(conn.Exec(q));

    // Select
    let s = "select * from testint32";
    let res = Assure(conn.Exec(s));
    let row = GetRow(res, 0);    

    alt row[1] {
      Int32(n) {        
        assert n == 1 << 20;
      }
      _ { 
        log(error, "Int32 Error--------------------------------------------");
        log(error, row[1]);
        fail("pq: wrong Int32 value from db");
      }
    }

    // This isn't Serial(1) because there is no Ftype "SERIAL"
    // as far as I understand, serial is a macro of "Int32 unique sequential" or the like
    // and the differentiation is lost upon insert.  I MAY BE WRONG! :)
    assert row[0] == Int32(1); 
}
